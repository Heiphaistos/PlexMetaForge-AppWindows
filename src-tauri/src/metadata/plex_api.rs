use serde::Deserialize;
use crate::error::{PlexMetaForgeError, Result};

#[derive(Deserialize)]
struct SectionsEnvelope {
    #[serde(rename = "MediaContainer")]
    media_container: SectionsContainer,
}

#[derive(Deserialize)]
struct SectionsContainer {
    #[serde(rename = "Directory", default)]
    directories: Vec<PlexSection>,
}

#[derive(Deserialize)]
struct PlexSection {
    key: String,
    #[serde(rename = "Location", default)]
    locations: Vec<PlexLocation>,
}

#[derive(Deserialize)]
struct PlexLocation {
    path: String,
}

pub async fn test_connection(base_url: &str, token: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(PlexMetaForgeError::Http)?;

    let resp = client
        .get(format!("{}/identity", base_url))
        .header("X-Plex-Token", token)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| PlexMetaForgeError::PlexApi(format!("Connexion impossible : {}", e)))?;

    if !resp.status().is_success() {
        return Err(PlexMetaForgeError::PlexApi(format!(
            "Plex répond HTTP {} — token invalide ou serveur inaccessible",
            resp.status()
        )));
    }

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| PlexMetaForgeError::PlexApi(format!("Réponse invalide : {}", e)))?;

    let version = json
        .pointer("/MediaContainer/version")
        .and_then(|v| v.as_str())
        .unwrap_or("?");
    let friendly_name = json
        .pointer("/MediaContainer/friendlyName")
        .and_then(|v| v.as_str())
        .unwrap_or("Plex");

    Ok(format!("{} — v{}", friendly_name, version))
}

pub async fn refresh_section(base_url: &str, media_path: &str, token: &str) -> Result<()> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(PlexMetaForgeError::Http)?;

    let envelope: SectionsEnvelope = client
        .get(format!("{}/library/sections", base_url))
        .header("X-Plex-Token", token)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(PlexMetaForgeError::Http)?
        .json()
        .await
        .map_err(|e| PlexMetaForgeError::PlexApi(format!("Parse sections : {}", e)))?;

    let key = envelope
        .media_container
        .directories
        .iter()
        .find(|sec| {
            sec.locations
                .iter()
                .any(|loc| media_path.starts_with(&loc.path))
        })
        .map(|sec| sec.key.clone())
        .ok_or_else(|| {
            PlexMetaForgeError::PlexApi(format!(
                "Aucune section ne correspond au chemin : {}",
                media_path
            ))
        })?;

    let resp = client
        .get(format!("{}/library/sections/{}/refresh", base_url, key))
        .header("X-Plex-Token", token)
        .query(&[("path", media_path)])
        .send()
        .await
        .map_err(PlexMetaForgeError::Http)?;

    if !resp.status().is_success() {
        return Err(PlexMetaForgeError::PlexApi(format!(
            "Refresh HTTP {}",
            resp.status()
        )));
    }

    Ok(())
}
