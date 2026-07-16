pub mod nfo;
pub mod plex_api;
pub mod sqlite_direct;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::PlexPaths;
use crate::error::{PlexMetaForgeError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataPayload {
    pub title: String,
    pub year: Option<i32>,
    pub plot: Option<String>,
    pub poster_url: Option<String>,
    pub fanart_url: Option<String>,
    pub tmdb_id: Option<String>,
    pub imdb_id: Option<String>,
    pub tagline: Option<String>,
    pub studio: Option<String>,
    pub rating: Option<f64>,
    pub media_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InjectionReport {
    pub nfo_written: bool,
    pub poster_saved: bool,
    pub fanart_saved: bool,
    pub plex_api_refreshed: bool,
    pub sqlite_updated: bool,
    pub errors: Vec<String>,
}

pub async fn inject(
    payload: MetadataPayload,
    plex_paths: Option<PlexPaths>,
    plex_url: String,
    plex_token: Option<String>,
) -> Result<InjectionReport> {
    let media_path = PathBuf::from(&payload.media_path);
    let mut report = InjectionReport {
        nfo_written: false,
        poster_saved: false,
        fanart_saved: false,
        plex_api_refreshed: false,
        sqlite_updated: false,
        errors: Vec::new(),
    };

    // Axe Passif 1 — NFO
    match nfo::write_nfo(&media_path, &payload) {
        Ok(_) => report.nfo_written = true,
        Err(e) => report.errors.push(format!("NFO: {}", e)),
    }

    // Axe Passif 2 — Poster
    if let Some(ref url) = payload.poster_url {
        if !url.is_empty() {
            match download_image(url, &media_path.join("poster.jpg")).await {
                Ok(_) => report.poster_saved = true,
                Err(e) => report.errors.push(format!("Poster: {}", e)),
            }
        }
    }

    // Axe Passif 3 — Fanart
    if let Some(ref url) = payload.fanart_url {
        if !url.is_empty() {
            match download_image(url, &media_path.join("fanart.jpg")).await {
                Ok(_) => report.fanart_saved = true,
                Err(e) => report.errors.push(format!("Fanart: {}", e)),
            }
        }
    }

    // Axe Actif — API Plex, fallback SQLite
    if let Some(ref token) = plex_token {
        match plex_api::refresh_section(&plex_url, &payload.media_path, token).await {
            Ok(_) => report.plex_api_refreshed = true,
            Err(e) => {
                report.errors.push(format!("Plex API: {}", e));
                try_sqlite_update(&plex_paths, &payload, &mut report);
            }
        }
    } else {
        // Pas de token → SQLite direct
        try_sqlite_update(&plex_paths, &payload, &mut report);
    }

    Ok(report)
}

fn try_sqlite_update(
    plex_paths: &Option<PlexPaths>,
    payload: &MetadataPayload,
    report: &mut InjectionReport,
) {
    match plex_paths {
        Some(paths) => match sqlite_direct::update_metadata(paths, payload) {
            Ok(n) if n > 0 => report.sqlite_updated = true,
            Ok(_) => report
                .errors
                .push("SQLite: aucun enregistrement correspondant".to_string()),
            Err(e) => report.errors.push(format!("SQLite: {}", e)),
        },
        None => report
            .errors
            .push("SQLite: chemin DB indisponible".to_string()),
    }
}

/// Valide qu'une URL est HTTPS vers un domaine public (bloque SSRF vers réseau interne).
fn validate_image_url(url: &str) -> Result<()> {
    // Exige HTTPS uniquement pour les images externes (poster/fanart depuis TMDB/TVDB/etc.)
    if !url.starts_with("https://") {
        return Err(PlexMetaForgeError::PlexApi(format!(
            "URL invalide (HTTPS requis) : {}",
            url
        )));
    }
    // Bloque les cibles SSRF connues : localhost, 127.x, 192.168.x, 10.x, 172.16-31.x, [::1]
    let lower = url.to_lowercase();
    let blocked = ["localhost", "127.", "192.168.", "10.", "[::1]", "0.0.0.0",
                   "169.254.", "metadata.google", "169.254.169.254"];
    for pattern in &blocked {
        if lower.contains(pattern) {
            return Err(PlexMetaForgeError::PlexApi(format!(
                "URL refusée (cible interne interdite) : {}",
                url
            )));
        }
    }
    // Vérifie les plages 172.16.0.0/12
    for i in 16u8..=31 {
        if lower.contains(&format!("172.{}.", i)) {
            return Err(PlexMetaForgeError::PlexApi(
                "URL refusée (réseau privé 172.16-31.x)".to_string()
            ));
        }
    }
    Ok(())
}

async fn download_image(url: &str, dest: &PathBuf) -> Result<()> {
    validate_image_url(url)?;
    let resp = reqwest::get(url).await.map_err(PlexMetaForgeError::Http)?;
    if !resp.status().is_success() {
        return Err(PlexMetaForgeError::PlexApi(format!(
            "HTTP {} — {}",
            resp.status(),
            url
        )));
    }
    let bytes = resp.bytes().await.map_err(PlexMetaForgeError::Http)?;
    if bytes.is_empty() {
        return Err(PlexMetaForgeError::PlexApi(format!("Image vide reçue depuis : {}", url)));
    }
    std::fs::write(dest, bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn make_fixture_db(dir: &PathBuf) -> PathBuf {
        let db_path = dir.join("com.plexapp.plugins.library.db");
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE metadata_items (
                id INTEGER PRIMARY KEY,
                title TEXT, year INTEGER, summary TEXT, user_thumb_url TEXT,
                metadata_type INTEGER, library_section_id INTEGER, duration INTEGER,
                rating REAL, tagline TEXT, studio TEXT, originally_available_at TEXT
            )",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO metadata_items (id, title, year, summary, metadata_type)
             VALUES (1, 'Test Movie', 2020, 'old summary', 1)",
            [],
        ).unwrap();
        db_path
    }

    #[test]
    fn inject_writes_nfo_and_full_sqlite_row_when_no_plex_token() {
        let base = std::env::temp_dir().join(format!("pmf_test_{}", std::process::id()));
        let media_dir = base.join("Test Movie (2020)");
        std::fs::create_dir_all(&media_dir).unwrap();
        let db_path = make_fixture_db(&base);

        let payload = MetadataPayload {
            title: "Test Movie".to_string(),
            year: Some(2020),
            plot: Some("A new summary".to_string()),
            poster_url: None,
            fanart_url: None,
            tmdb_id: Some("550".to_string()),
            imdb_id: None,
            tagline: Some("Rien n'est impossible.".to_string()),
            studio: Some("Test Studio".to_string()),
            rating: Some(7.5),
            media_path: media_dir.to_string_lossy().to_string(),
        };

        let plex_paths = PlexPaths {
            plugins_dir: base.clone(),
            database_path: db_path.clone(),
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let report = rt.block_on(inject(payload, Some(plex_paths), String::new(), None)).unwrap();

        assert!(report.nfo_written, "NFO devrait être écrit: {:?}", report.errors);
        assert!(report.sqlite_updated, "SQLite devrait être mis à jour: {:?}", report.errors);

        // Vérifie le contenu NFO réel sur disque
        let nfo_content = std::fs::read_to_string(media_dir.join("Test Movie (2020).nfo")).unwrap();
        assert!(nfo_content.contains("<tagline>Rien n&apos;est impossible.</tagline>"));
        assert!(nfo_content.contains("<studio>Test Studio</studio>"));
        assert!(nfo_content.contains("<rating>7.5</rating>"));

        // Vérifie la ligne SQLite réellement persistée
        let conn = Connection::open(&db_path).unwrap();
        let (summary, tagline, studio, rating): (String, String, String, f64) = conn.query_row(
            "SELECT summary, tagline, studio, rating FROM metadata_items WHERE id = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        ).unwrap();
        assert_eq!(summary, "A new summary");
        assert_eq!(tagline, "Rien n'est impossible.");
        assert_eq!(studio, "Test Studio");
        assert_eq!(rating, 7.5);

        std::fs::remove_dir_all(&base).ok();
    }
}
