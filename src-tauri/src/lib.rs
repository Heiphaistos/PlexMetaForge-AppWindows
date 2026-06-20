mod config;
mod database;
mod error;
mod export;
mod generator;
mod metadata;
mod scanner;
mod settings;
mod store;

use config::PlexPaths;
use database::{BatchUpdateResult, DatabaseStats, MediaItem, PlexSection};
use export::ExportResult;
use store::{InstallResult, StorePlugin};
use generator::{PluginConfig, SelectiveConfig};
use metadata::{InjectionReport, MetadataPayload};
use settings::AppSettings;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub plex_paths: Mutex<Option<PlexPaths>>,
    pub settings: Mutex<AppSettings>,
}

// ─── Settings ─────────────────────────────────────────────────

#[tauri::command]
fn get_settings(state: State<AppState>) -> AppSettings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn save_settings(new_settings: AppSettings, state: State<AppState>) -> Result<(), String> {
    settings::save(&new_settings).map_err(|e| e.to_string())?;
    *state.settings.lock().unwrap() = new_settings;
    Ok(())
}

#[tauri::command]
async fn test_plex_connection(state: State<'_, AppState>) -> Result<String, String> {
    let s = state.settings.lock().unwrap().clone();
    if s.plex_token.is_empty() {
        return Err("Token Plex vide — configure-le dans Paramètres.".to_string());
    }
    metadata::plex_api::test_connection(&s.plex_url, &s.plex_token)
        .await
        .map_err(|e| e.to_string())
}

// ─── Config ───────────────────────────────────────────────────

#[tauri::command]
fn get_plex_paths(state: State<AppState>) -> Result<serde_json::Value, String> {
    // Clone settings first, then drop lock before acquiring plex_paths lock
    let (custom_plugins, custom_db) = {
        let s = state.settings.lock().unwrap();
        (s.custom_plugins_dir.clone(), s.custom_db_path.clone())
    };

    let plugins_dir = custom_plugins
        .map(std::path::PathBuf::from)
        .or_else(|| state.plex_paths.lock().unwrap().as_ref().map(|p| p.plugins_dir.clone()));
    let db_path = custom_db
        .map(std::path::PathBuf::from)
        .or_else(|| state.plex_paths.lock().unwrap().as_ref().map(|p| p.database_path.clone()));

    match (plugins_dir, db_path) {
        (Some(pd), Some(db)) => Ok(serde_json::json!({
            "plugins_dir": pd.to_string_lossy(),
            "database_path": db.to_string_lossy(),
            "plugins_dir_exists": pd.exists(),
            "database_exists": db.exists(),
        })),
        _ => Err("Plex non détecté. Configure les chemins dans Paramètres.".to_string()),
    }
}

// ─── Scanner ──────────────────────────────────────────────────

#[tauri::command]
fn list_plugins(state: State<AppState>) -> Result<Vec<scanner::Plugin>, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    scanner::list_plugins(&plugins_dir).map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_plugin(path: String, enable: bool, state: State<AppState>) -> Result<String, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    let plugin_path = std::path::PathBuf::from(&path);
    // FIX — Confinement : le plugin doit rester sous le dossier plugins
    let canonical_plugins = plugins_dir.canonicalize()
        .map_err(|e| format!("Dossier plugins inaccessible: {}", e))?;
    // Le path peut pointer vers .disabled — on normalise en ignorant le suffixe
    let base_path = std::path::PathBuf::from(
        path.trim_end_matches(".disabled")
    );
    let canonical_plugin = base_path.canonicalize()
        .or_else(|_| plugin_path.canonicalize())
        .map_err(|e| format!("Chemin plugin invalide: {}", e))?;
    if !canonical_plugin.starts_with(&canonical_plugins) {
        return Err("Accès refusé: chemin hors du dossier plugins".to_string());
    }
    scanner::toggle_plugin(&plugin_path, enable).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_plugin(path: String, state: State<AppState>) -> Result<(), String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    let plugin_path = std::path::PathBuf::from(&path);
    // FIX — Confinement : le plugin doit rester sous le dossier plugins
    let canonical_plugins = plugins_dir.canonicalize()
        .map_err(|e| format!("Dossier plugins inaccessible: {}", e))?;
    let canonical_plugin = plugin_path.canonicalize()
        .map_err(|e| format!("Chemin plugin invalide: {}", e))?;
    if !canonical_plugin.starts_with(&canonical_plugins) {
        return Err("Accès refusé: chemin hors du dossier plugins".to_string());
    }
    scanner::delete_plugin(&plugin_path).map_err(|e| e.to_string())
}

// ─── Generator ────────────────────────────────────────────────

#[tauri::command]
fn create_plugin(name: String, state: State<AppState>) -> Result<String, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    generator::create_plugin(&plugins_dir, &name).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_plugin_from_template(config: PluginConfig, state: State<AppState>) -> Result<String, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    generator::create_plugin_from_config(&plugins_dir, &config).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_selective_plugin(config: SelectiveConfig, state: State<AppState>) -> Result<String, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    generator::create_selective_plugin(&plugins_dir, &config).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_plugin_templates() -> serde_json::Value {
    serde_json::json!([
        { "id": "blank",     "label": "Vierge",                 "icon": "📄",
          "description": "Structure minimale. Tu codes tout toi-même.",
          "requires_tmdb": false, "requires_lastfm": false },
        { "id": "cinema",    "label": "Films",                  "icon": "🎬",
          "description": "Agent complet films. Titres, synopsis, posters, casting depuis TMDB.",
          "requires_tmdb": true,  "requires_lastfm": false },
        { "id": "series",    "label": "Séries TV",              "icon": "📺",
          "description": "Agent séries avec épisodes, saisons, vignettes via TMDB.",
          "requires_tmdb": true,  "requires_lastfm": false },
        { "id": "musique",   "label": "Musique",                "icon": "🎵",
          "description": "Double agent Artiste + Album. Biographies, jaquettes via Last.fm.",
          "requires_tmdb": false, "requires_lastfm": true  },
        { "id": "anime",     "label": "Anime / Manga",          "icon": "⛩️",
          "description": "Spécialisé anime/manga via AniList. Sans clé API requise.",
          "requires_tmdb": false, "requires_lastfm": false },
        { "id": "universal", "label": "Universel (All-in-one)", "icon": "🌐",
          "description": "Films + Séries + Anime + Musique. Détection automatique.",
          "requires_tmdb": true,  "requires_lastfm": false }
    ])
}

#[tauri::command]
fn read_plugin_code(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    let plugin_path = std::path::PathBuf::from(&path);
    let canonical_plugins = plugins_dir
        .canonicalize()
        .map_err(|e| format!("Dossier plugins inaccessible: {}", e))?;
    let canonical_plugin = plugin_path
        .canonicalize()
        .map_err(|e| format!("Chemin plugin invalide: {}", e))?;
    if !canonical_plugin.starts_with(&canonical_plugins) {
        return Err("Accès refusé: chemin hors du dossier plugins".to_string());
    }
    let init = canonical_plugin
        .join("Contents")
        .join("Code")
        .join("__init__.py");
    std::fs::read_to_string(&init).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_plugin_code(path: String, content: String, state: State<'_, AppState>) -> Result<(), String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    let plugin_path = std::path::PathBuf::from(&path);
    let canonical_plugins = plugins_dir
        .canonicalize()
        .map_err(|e| format!("Dossier plugins inaccessible: {}", e))?;
    let canonical_plugin = plugin_path
        .canonicalize()
        .map_err(|e| format!("Chemin plugin invalide: {}", e))?;
    if !canonical_plugin.starts_with(&canonical_plugins) {
        return Err("Accès refusé: chemin hors du dossier plugins".to_string());
    }

    let init = canonical_plugin
        .join("Contents")
        .join("Code")
        .join("__init__.py");
    std::fs::write(&init, content).map_err(|e| e.to_string())
}

// ─── Store ────────────────────────────────────────────────────

#[tauri::command]
fn get_store_catalog() -> Vec<StorePlugin> {
    store::catalog()
}

#[tauri::command]
fn get_store_categories() -> serde_json::Value {
    let cats = store::categories();
    serde_json::json!(cats.into_iter().map(|(cat, subs)| {
        serde_json::json!({ "name": cat, "subcategories": subs })
    }).collect::<Vec<_>>())
}

#[tauri::command]
async fn install_store_plugin(
    zip_url: String,
    bundle_name: String,
    state: State<'_, AppState>,
) -> Result<InstallResult, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    store::download_and_install(&zip_url, &bundle_name, &plugins_dir)
        .await
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
struct BatchInstallResult {
    results: Vec<BatchItem>,
}

#[derive(serde::Serialize)]
struct BatchItem {
    bundle_name: String,
    ok: bool,
    message: String,
}

#[tauri::command]
async fn batch_install_plugins(
    plugins: Vec<serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<BatchInstallResult, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    let mut results = Vec::new();

    for plugin in plugins {
        let zip_url = plugin["zip_url"].as_str().unwrap_or("").to_string();
        let bundle_name = plugin["bundle_name"].as_str().unwrap_or("").to_string();
        if zip_url.is_empty() || bundle_name.is_empty() { continue; }

        match store::download_and_install(&zip_url, &bundle_name, &plugins_dir).await {
            Ok(r) => results.push(BatchItem {
                bundle_name: r.bundle_name,
                ok: true,
                message: if r.already_existed { "Mis à jour".to_string() } else { "Installé".to_string() },
            }),
            Err(e) => results.push(BatchItem {
                bundle_name: bundle_name.clone(),
                ok: false,
                message: e.to_string(),
            }),
        }
    }

    Ok(BatchInstallResult { results })
}

#[tauri::command]
fn get_installed_plugin_ids(state: State<AppState>) -> Vec<String> {
    let Ok(plugins_dir) = resolve_plugins_dir(&state) else { return vec![]; };
    let Ok(plugins) = scanner::list_plugins(&plugins_dir) else { return vec![]; };
    plugins.into_iter().map(|p| {
        p.name
            .trim_end_matches(".bundle")
            .to_lowercase()
            .replace(['-', '_', ' '], "")
    }).collect()
}

// ─── Export ───────────────────────────────────────────────────

#[tauri::command]
fn export_plugin(path: String, dest_dir: Option<String>, state: State<AppState>) -> Result<ExportResult, String> {
    let bundle = std::path::PathBuf::from(&path);
    // FIX — Confinement : le bundle doit rester sous le dossier plugins
    let plugins_dir = resolve_plugins_dir(&state)?;
    let canonical_plugins = plugins_dir.canonicalize()
        .map_err(|e| format!("Dossier plugins inaccessible: {}", e))?;
    let canonical_bundle = bundle.canonicalize()
        .map_err(|e| format!("Chemin bundle invalide: {}", e))?;
    if !canonical_bundle.starts_with(&canonical_plugins) {
        return Err("Accès refusé: chemin hors du dossier plugins".to_string());
    }
    let dest = dest_dir.map(std::path::PathBuf::from).unwrap_or_else(export::default_export_dir);
    export::export_plugin_zip(&bundle, &dest).map_err(|e| e.to_string())
}

#[tauri::command]
fn export_all_plugins(state: State<AppState>, dest_dir: Option<String>) -> Result<ExportResult, String> {
    let plugins_dir = resolve_plugins_dir(&state)?;
    let dest = dest_dir.map(std::path::PathBuf::from).unwrap_or_else(export::default_export_dir);
    export::export_all_plugins_zip(&plugins_dir, &dest).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_export_dir() -> String {
    export::default_export_dir().to_string_lossy().to_string()
}

// ─── Database ─────────────────────────────────────────────────

#[tauri::command]
fn get_db_stats(state: State<AppState>) -> Result<DatabaseStats, String> {
    let db = open_db(&state)?;
    db.get_stats().map_err(|e| e.to_string())
}

#[tauri::command]
fn get_db_sections(state: State<AppState>) -> Result<Vec<PlexSection>, String> {
    let db = open_db(&state)?;
    db.get_sections().map_err(|e| e.to_string())
}

#[tauri::command]
fn search_plex_db(query: String, state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let db = open_db(&state)?;
    db.search_metadata_items(&query).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_items_by_section(section_id: i64, limit: Option<i64>, state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let db = open_db(&state)?;
    db.get_items_by_section(section_id, limit.unwrap_or(200)).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_incomplete_items(limit: Option<i64>, state: State<AppState>) -> Result<Vec<MediaItem>, String> {
    let db = open_db(&state)?;
    db.get_incomplete_items(limit.unwrap_or(100)).map_err(|e| e.to_string())
}

#[tauri::command]
fn db_batch_clear_locks(state: State<AppState>) -> Result<BatchUpdateResult, String> {
    let db = open_db(&state)?;
    db.batch_clear_locks().map_err(|e| e.to_string())
}

// ─── Metadata Engine ──────────────────────────────────────────

#[tauri::command]
async fn inject_metadata(payload: MetadataPayload, state: State<'_, AppState>) -> Result<InjectionReport, String> {
    let s = state.settings.lock().unwrap().clone();
    let plex_paths = resolve_plex_paths(&state);
    let token = if s.plex_token.is_empty() { None } else { Some(s.plex_token.clone()) };
    metadata::inject(payload, plex_paths, s.plex_url, token)
        .await
        .map_err(|e| e.to_string())
}

// ─── Helpers ──────────────────────────────────────────────────

fn resolve_plugins_dir(state: &State<AppState>) -> Result<std::path::PathBuf, String> {
    let s = state.settings.lock().unwrap();
    if let Some(ref custom) = s.custom_plugins_dir {
        return Ok(std::path::PathBuf::from(custom));
    }
    drop(s);
    let guard = state.plex_paths.lock().unwrap();
    guard
        .as_ref()
        .map(|p| p.plugins_dir.clone())
        .ok_or_else(|| "Dossier Plug-ins introuvable. Configure-le dans ⚙ Paramètres.".to_string())
}

fn resolve_plex_paths(state: &State<AppState>) -> Option<PlexPaths> {
    // Clone settings first to avoid holding two locks simultaneously
    let (custom_plugins, custom_db) = {
        let s = state.settings.lock().unwrap();
        (s.custom_plugins_dir.clone(), s.custom_db_path.clone())
    };
    let guard = state.plex_paths.lock().unwrap();
    let base = guard.as_ref()?;
    let plugins_dir = custom_plugins
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| base.plugins_dir.clone());
    let database_path = custom_db
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| base.database_path.clone());
    Some(PlexPaths { plugins_dir, database_path })
}

fn open_db(state: &State<AppState>) -> Result<database::PlexDatabase, String> {
    let s = state.settings.lock().unwrap();
    let db_path = if let Some(ref custom) = s.custom_db_path {
        std::path::PathBuf::from(custom)
    } else {
        drop(s);
        let guard = state.plex_paths.lock().unwrap();
        guard
            .as_ref()
            .map(|p| p.database_path.clone())
            .ok_or_else(|| "Base de données introuvable. Configure-la dans ⚙ Paramètres.".to_string())?
    };
    database::PlexDatabase::open(&db_path).map_err(|e| e.to_string())
}

// ─── Entry Point ──────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    let plex_paths = PlexPaths::detect().ok();
    let loaded_settings = settings::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            plex_paths: Mutex::new(plex_paths),
            settings: Mutex::new(loaded_settings),
        })
        .invoke_handler(tauri::generate_handler![
            // Settings
            get_settings, save_settings, test_plex_connection,
            // Config
            get_plex_paths,
            // Scanner
            list_plugins, toggle_plugin, delete_plugin,
            // Generator
            create_plugin, create_plugin_from_template, create_selective_plugin, get_plugin_templates,
            read_plugin_code, write_plugin_code,
            // Store
            get_store_catalog, get_store_categories,
            install_store_plugin, batch_install_plugins, get_installed_plugin_ids,
            // Export
            export_plugin, export_all_plugins, get_export_dir,
            // Database
            get_db_stats, get_db_sections, search_plex_db,
            get_items_by_section, get_incomplete_items, db_batch_clear_locks,
            // Metadata
            inject_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running PlexMetaForge");
}
