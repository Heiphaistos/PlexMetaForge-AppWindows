use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub plex_url: String,
    pub plex_token: String,
    pub custom_plugins_dir: Option<String>,
    pub custom_db_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            plex_url: "http://localhost:32400".to_string(),
            plex_token: String::new(),
            custom_plugins_dir: None,
            custom_db_path: None,
        }
    }
}

fn settings_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(appdata).join("PlexMetaForge").join("settings.json")
}

pub fn load() -> AppSettings {
    let path = settings_path();
    if !path.exists() {
        return AppSettings::default();
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(settings: &AppSettings) -> Result<()> {
    let path = settings_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    let json = serde_json::to_string_pretty(settings)?;
    std::fs::write(&path, json)?;
    Ok(())
}
