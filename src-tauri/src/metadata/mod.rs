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
