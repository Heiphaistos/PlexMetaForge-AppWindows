pub mod combiner;
pub mod templates;

pub use combiner::SelectiveConfig;

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PluginTemplate {
    Blank,
    Cinema,
    Series,
    Musique,
    Anime,
    Universal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    pub name: String,
    pub template: PluginTemplate,
    pub tmdb_api_key: Option<String>,
    pub lastfm_api_key: Option<String>,
}

// ─── Selective plugin (custom combination) ───────────────────

pub fn create_selective_plugin(plugins_dir: &PathBuf, cfg: &SelectiveConfig) -> Result<String> {
    let safe_name = cfg.name.trim().replace(' ', "_");
    let bundle_path = plugins_dir.join(format!("{}.bundle", safe_name));

    let code_dir      = bundle_path.join("Contents").join("Code");
    let resources_dir = bundle_path.join("Contents").join("Resources");
    std::fs::create_dir_all(&code_dir)?;
    std::fs::create_dir_all(&resources_dir)?;

    std::fs::write(
        bundle_path.join("Contents").join("Info.plist"),
        templates::info_plist(&safe_name, &cfg.name),
    )?;

    std::fs::write(
        bundle_path.join("Contents").join("DefaultPrefs.json"),
        combiner::prefs_json(cfg),
    )?;

    let mut code = combiner::generate(cfg);

    // Inject API keys directly into the code if provided
    if let Some(ref k) = cfg.tmdb_key {
        if !k.is_empty() {
            code = code.replace("Prefs[\"tmdb_api_key\"]", &format!("\"{}\"", k));
        }
    }
    if let Some(ref k) = cfg.lastfm_key {
        if !k.is_empty() {
            code = code.replace("Prefs.get(\"lastfm_api_key\",\"\")", &format!("\"{}\"", k));
        }
    }

    std::fs::write(code_dir.join("__init__.py"), code)?;

    let readme = build_selective_readme(cfg);
    std::fs::write(resources_dir.join("README.txt"), readme)?;

    Ok(bundle_path.to_string_lossy().to_string())
}

fn build_selective_readme(cfg: &SelectiveConfig) -> String {
    let agents: Vec<&str> = [
        if cfg.films        { "Films (Agent.Movies)" } else { "" },
        if cfg.series       { "Séries TV (Agent.TV_Shows)" } else { "" },
        if cfg.anime        { "Anime/Manga (détection automatique)" } else { "" },
        if cfg.music_artist { "Musique — Artiste (Agent.Artist)" } else { "" },
        if cfg.music_album  { "Musique — Album (Agent.Album)" } else { "" },
    ].iter().filter(|s| !s.is_empty()).copied().collect();

    let sources: Vec<&str> = [
        if cfg.use_tmdb   { "TMDB (films/séries)" } else { "" },
        if cfg.use_anilist && cfg.anime { "AniList (anime — sans clé)" } else { "" },
        if cfg.use_lastfm { "Last.fm (musique)" } else { "" },
    ].iter().filter(|s| !s.is_empty()).copied().collect();

    format!(
        "Plugin Plex Universel Personnalisé\n{}\n\nAgents : {}\nSources : {}\n\nInstallation :\n1. Copier dans %LOCALAPPDATA%\\Plex Media Server\\Plug-ins\\\n2. Redémarrer Plex\n3. Configurer dans Paramètres > Agents\n",
        "=".repeat(40),
        agents.join(", "),
        sources.join(", ")
    )
}

// ─── Simple blank plugin (legacy API) ────────────────────────

pub fn create_plugin(plugins_dir: &PathBuf, plugin_name: &str) -> Result<String> {
    create_plugin_from_config(plugins_dir, &PluginConfig {
        name: plugin_name.to_string(),
        template: PluginTemplate::Blank,
        tmdb_api_key: None,
        lastfm_api_key: None,
    })
}

// ─── Template-based creation ──────────────────────────────────

pub fn create_plugin_from_config(plugins_dir: &PathBuf, config: &PluginConfig) -> Result<String> {
    let safe_name = config.name.trim().replace(' ', "_");
    let bundle_path = plugins_dir.join(format!("{}.bundle", safe_name));

    let code_dir      = bundle_path.join("Contents").join("Code");
    let resources_dir = bundle_path.join("Contents").join("Resources");
    std::fs::create_dir_all(&code_dir)?;
    std::fs::create_dir_all(&resources_dir)?;

    // Info.plist
    std::fs::write(
        bundle_path.join("Contents").join("Info.plist"),
        templates::info_plist(&safe_name, &config.name),
    )?;

    // DefaultPrefs.json
    let (has_tmdb, has_lastfm) = match config.template {
        PluginTemplate::Cinema  => (true, false),
        PluginTemplate::Series  => (true, false),
        PluginTemplate::Musique => (false, true),
        PluginTemplate::Anime   => (false, false),
        PluginTemplate::Universal => (true, true),
        PluginTemplate::Blank   => (false, false),
    };
    std::fs::write(
        bundle_path.join("Contents").join("DefaultPrefs.json"),
        templates::default_prefs_json(has_tmdb, has_lastfm),
    )?;

    // __init__.py
    let mut init_py = match config.template {
        PluginTemplate::Cinema    => templates::cinema_init_py(&config.name),
        PluginTemplate::Series    => templates::series_init_py(&config.name),
        PluginTemplate::Musique   => templates::musique_init_py(&config.name),
        PluginTemplate::Anime     => templates::anime_init_py(&config.name),
        PluginTemplate::Universal => templates::universal_init_py(&config.name),
        PluginTemplate::Blank     => blank_init_py(&config.name),
    };

    // Injecter les clés API si fournies
    if let Some(ref k) = config.tmdb_api_key {
        if !k.is_empty() {
            init_py = init_py.replace("YOUR_TMDB_API_KEY_HERE", k);
        }
    }
    if let Some(ref k) = config.lastfm_api_key {
        if !k.is_empty() {
            init_py = init_py.replace("YOUR_LASTFM_API_KEY_HERE", k);
        }
    }

    std::fs::write(code_dir.join("__init__.py"), init_py)?;

    // README dans Resources
    std::fs::write(
        resources_dir.join("README.txt"),
        build_readme(config),
    )?;

    Ok(bundle_path.to_string_lossy().to_string())
}

fn blank_init_py(name: &str) -> String {
    format!(r#"# ================================================================
# {} — Plugin Plex (Framework 2)
# Généré par PlexMetaForge — Version 1.0.0
# ================================================================

AGENT_VERSION = "1.0.0"

def Start():
    Log.Info("[{}] v{{}} démarré".format(AGENT_VERSION))

def ValidatePrefs():
    return MessageContainer("OK", "Prêt.")

class {}(Agent.Movies):
    name             = "{}"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        # TODO: implémenter la recherche
        results.Append(MetadataSearchResult(
            id    = media.name,
            name  = media.name,
            score = 80,
            lang  = lang
        ))

    def update(self, metadata, media, lang, force):
        # TODO: implémenter la mise à jour des métadonnées
        metadata.title = media.name
"#,
        name, name, name, name
    )
}

fn build_readme(config: &PluginConfig) -> String {
    let template_label = match config.template {
        PluginTemplate::Cinema    => "Films (TMDB)",
        PluginTemplate::Series    => "Séries TV (TMDB)",
        PluginTemplate::Musique   => "Musique (Last.fm)",
        PluginTemplate::Anime     => "Anime/Manga (AniList)",
        PluginTemplate::Universal => "Universel (TMDB + AniList + Last.fm)",
        PluginTemplate::Blank     => "Vierge",
    };

    format!(r#"================================================
{}
Plugin Plex — Généré par PlexMetaForge
================================================

Template : {}
Version  : 1.0.0

INSTALLATION
------------
1. Copier ce dossier dans :
   %LOCALAPPDATA%\Plex Media Server\Plug-ins\

2. Redémarrer Plex Media Server.

3. Configurer dans :
   Plex > Préférences > Agents > (type de média) > {}

CONFIGURATION DES CLÉS API
---------------------------
{}

FICHIERS GÉNÉRÉS
----------------
  Contents/
    Info.plist         — Identité du plugin
    DefaultPrefs.json  — Préférences configurables
    Code/
      __init__.py      — Code principal (Python 2/3)
    Resources/
      README.txt       — Ce fichier

================================================
"#,
        config.name,
        template_label,
        config.name,
        match config.template {
            PluginTemplate::Cinema | PluginTemplate::Series | PluginTemplate::Universal =>
                "TMDB : https://www.themoviedb.org/settings/api\n  (compte gratuit requis)",
            PluginTemplate::Musique =>
                "Last.fm : https://www.last.fm/api/account/create\n  (compte gratuit requis)",
            PluginTemplate::Anime =>
                "Aucune clé requise — AniList GraphQL est libre d'accès.",
            PluginTemplate::Blank =>
                "Aucune clé requise par défaut.",
        }
    )
}
