use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub plex_url: String,
    /// Jamais ecrit dans settings.json (trousseau) ni renvoye a l'interface.
    #[serde(default)]
    pub plex_token: String,
    /// Indique a l'interface qu'un jeton est enregistre (sans le reveler).
    #[serde(default)]
    pub plex_token_set: bool,
    pub custom_plugins_dir: Option<String>,
    pub custom_db_path: Option<String>,
    /// Verification silencieuse des mises a jour au demarrage.
    #[serde(default = "default_true")]
    pub auto_update_check: bool,
}

fn default_true() -> bool {
    true
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            plex_url: "http://localhost:32400".to_string(),
            plex_token: String::new(),
            plex_token_set: false,
            custom_plugins_dir: None,
            custom_db_path: None,
            auto_update_check: true,
        }
    }
}

fn settings_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(appdata).join("PlexMetaForge").join("settings.json")
}

/// Coffre du jeton (Gestionnaire d'identifiants Windows / trousseau du systeme).
trait TokenStore {
    fn get(&self) -> Option<String>;
    /// `None` supprime l'entree.
    fn set(&self, value: Option<&str>) -> std::result::Result<(), String>;
}

struct Keyring;

impl Keyring {
    fn entry() -> std::result::Result<keyring::Entry, String> {
        keyring::Entry::new("PlexMetaForge", "plex_token").map_err(|e| e.to_string())
    }
}

impl TokenStore for Keyring {
    fn get(&self) -> Option<String> {
        Self::entry().ok()?.get_password().ok()
    }
    fn set(&self, value: Option<&str>) -> std::result::Result<(), String> {
        let entry = Self::entry()?;
        match value {
            Some(v) => entry.set_password(v).map_err(|e| e.to_string()),
            None => match entry.delete_credential() {
                Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
        }
    }
}

fn write_json(path: &std::path::Path, settings: &AppSettings) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut on_disk = settings.clone();
    on_disk.plex_token.clear();
    on_disk.plex_token_set = false;
    std::fs::write(path, serde_json::to_string_pretty(&on_disk)?)?;
    Ok(())
}

/// Charge les reglages ; un jeton encore en clair dans le JSON (anciennes versions)
/// migre vers le coffre puis est efface du fichier.
fn load_from(path: &std::path::Path, store: &dyn TokenStore) -> AppSettings {
    let mut s: AppSettings = std::fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default();
    if s.plex_token.is_empty() {
        s.plex_token = store.get().unwrap_or_default();
    } else if store.set(Some(&s.plex_token)).is_ok() {
        write_json(path, &s).ok();
    }
    s.plex_token_set = !s.plex_token.is_empty();
    s
}

fn save_to(path: &std::path::Path, store: &dyn TokenStore, settings: &AppSettings) -> Result<()> {
    let value = Some(settings.plex_token.as_str()).filter(|t| !t.is_empty());
    store.set(value).map_err(crate::error::PlexMetaForgeError::PermissionDenied)?;
    write_json(path, settings)
}

pub fn load() -> AppSettings {
    load_from(&settings_path(), &Keyring)
}

pub fn save(settings: &AppSettings) -> Result<()> {
    save_to(&settings_path(), &Keyring, settings)
}

/// Version envoyee a l'interface : jamais le jeton.
pub fn masked(settings: &AppSettings) -> AppSettings {
    AppSettings { plex_token: String::new(), plex_token_set: !settings.plex_token.is_empty(), ..settings.clone() }
}

/// Reglages recus de l'interface : jeton vide + `plex_token_set` vrai = on garde l'actuel ;
/// jeton vide + `plex_token_set` faux = on le supprime ; sinon on le remplace.
pub fn merge_incoming(current: &AppSettings, mut incoming: AppSettings) -> AppSettings {
    if incoming.plex_token.is_empty() && incoming.plex_token_set {
        incoming.plex_token = current.plex_token.clone();
    }
    incoming.plex_token_set = !incoming.plex_token.is_empty();
    incoming
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct Mem(RefCell<Option<String>>);
    impl TokenStore for Mem {
        fn get(&self) -> Option<String> {
            self.0.borrow().clone()
        }
        fn set(&self, v: Option<&str>) -> std::result::Result<(), String> {
            *self.0.borrow_mut() = v.map(String::from);
            Ok(())
        }
    }

    fn with_token(t: &str) -> AppSettings {
        AppSettings { plex_token: t.into(), plex_token_set: !t.is_empty(), ..Default::default() }
    }

    #[test]
    fn save_keeps_token_out_of_the_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let store = Mem::default();
        save_to(&path, &store, &with_token("T0KEN")).unwrap();
        assert!(!std::fs::read_to_string(&path).unwrap().contains("T0KEN"));
        assert_eq!(load_from(&path, &store).plex_token, "T0KEN");
    }

    #[test]
    fn load_migrates_a_plaintext_token() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(&path, r#"{"plex_url":"http://x","plex_token":"OLD","custom_plugins_dir":null,"custom_db_path":null}"#).unwrap();
        let store = Mem::default();
        let s = load_from(&path, &store);
        assert_eq!(s.plex_token, "OLD");
        assert_eq!(store.get().as_deref(), Some("OLD"));
        assert!(!std::fs::read_to_string(&path).unwrap().contains("OLD"));
    }

    #[test]
    fn masked_never_exposes_the_token() {
        let m = masked(&with_token("T0KEN"));
        assert_eq!(m.plex_token, "");
        assert!(m.plex_token_set);
        assert!(!serde_json::to_string(&m).unwrap().contains("T0KEN"));
    }

    #[test]
    fn merge_keeps_replaces_or_clears_the_token() {
        let cur = with_token("OLD");
        let keep = merge_incoming(&cur, AppSettings { plex_token_set: true, ..Default::default() });
        assert_eq!(keep.plex_token, "OLD");
        let new = merge_incoming(&cur, with_token("NEW"));
        assert_eq!(new.plex_token, "NEW");
        let clear = merge_incoming(&cur, AppSettings { plex_token_set: false, ..Default::default() });
        assert_eq!((clear.plex_token.as_str(), clear.plex_token_set), ("", false));
    }
}
