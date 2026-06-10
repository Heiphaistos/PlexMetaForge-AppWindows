use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;
use crate::error::{PlexMetaForgeError, Result};

// ─── Catalogue ────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorePlugin {
    pub id: &'static str,
    pub name: &'static str,
    pub author: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub tags: Vec<&'static str>,
    pub license: &'static str,
    pub stars: &'static str,
    pub github_url: &'static str,
    /// URL du ZIP à télécharger (branche main ou release)
    pub zip_url: &'static str,
    /// Nom du dossier .bundle une fois extrait
    pub bundle_name: &'static str,
    pub verified: bool,
}

pub fn catalog() -> Vec<StorePlugin> {
    vec![
        StorePlugin {
            id: "hama",
            name: "Hama",
            author: "ZeroQI",
            description: "Agent métadonnées anime ultra-complet. Supporte AniDB, TVDB, TMDB, MAL, AniList. Le standard de référence pour l'anime sur Plex.",
            category: "Métadonnées",
            tags: vec!["anime", "métadonnées", "AniDB", "TVDB", "populaire"],
            license: "MIT",
            stars: "2.1k",
            github_url: "https://github.com/ZeroQI/Hama.bundle",
            zip_url: "https://github.com/ZeroQI/Hama.bundle/archive/refs/heads/master.zip",
            bundle_name: "Hama.bundle",
            verified: true,
        },
        StorePlugin {
            id: "absolute-series-scanner",
            name: "Absolute Series Scanner",
            author: "ZeroQI",
            description: "Scanner de séries TV avancé. Indispensable avec Hama pour l'anime. Gère les numérotations absolues et les arcs d'épisodes.",
            category: "Scanner",
            tags: vec!["anime", "scanner", "séries", "populaire"],
            license: "MIT",
            stars: "850",
            github_url: "https://github.com/ZeroQI/Absolute-Series-Scanner",
            zip_url: "https://github.com/ZeroQI/Absolute-Series-Scanner/archive/refs/heads/master.zip",
            bundle_name: "Absolute-Series-Scanner",
            verified: true,
        },
        StorePlugin {
            id: "subzero",
            name: "Sub-Zero",
            author: "pannal",
            description: "Gestionnaire de sous-titres tout-en-un. Télécharge automatiquement depuis OpenSubtitles, Subscene, Addic7ed. Synchronisation et correction.",
            category: "Sous-titres",
            tags: vec!["sous-titres", "opensubtitles", "automatique", "populaire"],
            license: "MIT",
            stars: "1.4k",
            github_url: "https://github.com/nicedoc/Sub-Zero.bundle",
            zip_url: "https://github.com/nicedoc/Sub-Zero.bundle/archive/refs/heads/master.zip",
            bundle_name: "Sub-Zero.bundle",
            verified: true,
        },
        StorePlugin {
            id: "webtools",
            name: "WebTools",
            author: "ukdtom",
            description: "Suite d'outils Plex : gestionnaire de plugins, nettoyage de bundles, synchronisation, statistiques. Interface web intégrée.",
            category: "Utilitaires",
            tags: vec!["outils", "gestion", "interface", "populaire"],
            license: "MIT",
            stars: "950",
            github_url: "https://github.com/ukdtom/WebTools.bundle",
            zip_url: "https://github.com/ukdtom/WebTools.bundle/archive/refs/heads/master.zip",
            bundle_name: "WebTools.bundle",
            verified: true,
        },
        StorePlugin {
            id: "opensubtitles",
            name: "OpenSubtitles",
            author: "nicedoc",
            description: "Agent sous-titres OpenSubtitles.org. Télécharge automatiquement les sous-titres les mieux notés pour vos films et séries.",
            category: "Sous-titres",
            tags: vec!["sous-titres", "opensubtitles", "films", "séries"],
            license: "MIT",
            stars: "320",
            github_url: "https://github.com/nicedoc/OpenSubtitles.bundle",
            zip_url: "https://github.com/nicedoc/OpenSubtitles.bundle/archive/refs/heads/master.zip",
            bundle_name: "OpenSubtitles.bundle",
            verified: true,
        },
        StorePlugin {
            id: "extended-personal-media",
            name: "Extended Personal Media Shows",
            author: "bnewbold",
            description: "Agent pour les enregistrements personnels et home vidéo. Organise et affiche les vidéos personnelles comme des émissions TV.",
            category: "Métadonnées",
            tags: vec!["personnel", "home vidéo", "enregistrements"],
            license: "Apache-2.0",
            stars: "180",
            github_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle",
            zip_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle/archive/refs/heads/master.zip",
            bundle_name: "ExtendedPersonalMedia-Agent.bundle",
            verified: true,
        },
        StorePlugin {
            id: "lastfm",
            name: "Last.fm Scrobbler",
            author: "Plex Community",
            description: "Scrobble automatiquement ta musique Plex vers Last.fm. Synchronise ton historique d'écoute en temps réel.",
            category: "Musique",
            tags: vec!["musique", "lastfm", "scrobble"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/plexinc-plugins/LastFM.bundle",
            zip_url: "https://github.com/plexinc-plugins/LastFM.bundle/archive/refs/heads/master.zip",
            bundle_name: "LastFM.bundle",
            verified: true,
        },
        StorePlugin {
            id: "ytdl-agent",
            name: "YouTube Agent",
            author: "JordyAlkema",
            description: "Agent métadonnées pour les vidéos YouTube téléchargées avec yt-dlp. Récupère titres, descriptions, miniatures automatiquement.",
            category: "Métadonnées",
            tags: vec!["youtube", "yt-dlp", "vidéos"],
            license: "MIT",
            stars: "290",
            github_url: "https://github.com/JordyAlkema/YoutubeMetadataAgent.bundle",
            zip_url: "https://github.com/JordyAlkema/YoutubeMetadataAgent.bundle/archive/refs/heads/master.zip",
            bundle_name: "YoutubeMetadataAgent.bundle",
            verified: true,
        },
        StorePlugin {
            id: "anidb-agent",
            name: "AniDB Agent",
            author: "Fribb",
            description: "Agent AniDB alternatif. Métadonnées anime depuis AniDB avec support des fichiers hash. Léger et précis.",
            category: "Métadonnées",
            tags: vec!["anime", "AniDB", "métadonnées"],
            license: "GPL-3.0",
            stars: "340",
            github_url: "https://github.com/Fribb/AniDB.bundle",
            zip_url: "https://github.com/Fribb/AniDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "AniDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "tvdb-agent",
            name: "TVDB Agent",
            author: "Plex Inc.",
            description: "Agent officiel TheTVDB. Métadonnées séries TV, épisodes, posters depuis The TV Database. Maintenu par Plex.",
            category: "Métadonnées",
            tags: vec!["séries", "TVDB", "officiel", "TV"],
            license: "MIT",
            stars: "290",
            github_url: "https://github.com/plexinc-plugins/TheTVDB.bundle",
            zip_url: "https://github.com/plexinc-plugins/TheTVDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "TheTVDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "tmdb-agent",
            name: "The Movie Database Agent",
            author: "Plex Inc.",
            description: "Agent officiel TMDB pour films. Titres, synopsis, posters, casting, backdrops en haute qualité. Maintenu par Plex.",
            category: "Métadonnées",
            tags: vec!["films", "TMDB", "officiel", "cinéma"],
            license: "MIT",
            stars: "310",
            github_url: "https://github.com/plexinc-plugins/TheMovieDB.bundle",
            zip_url: "https://github.com/plexinc-plugins/TheMovieDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "TheMovieDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "localmedia",
            name: "Local Media Assets",
            author: "Plex Inc.",
            description: "Agent assets locaux officiel. Utilise les poster.jpg, fanart.jpg et fichiers .nfo présents dans les dossiers médias. Essentiel.",
            category: "Utilitaires",
            tags: vec!["local", "nfo", "poster", "officiel", "essentiel"],
            license: "MIT",
            stars: "420",
            github_url: "https://github.com/plexinc-plugins/LocalMedia.bundle",
            zip_url: "https://github.com/plexinc-plugins/LocalMedia.bundle/archive/refs/heads/master.zip",
            bundle_name: "LocalMedia.bundle",
            verified: true,
        },

        // ── Sous-titres supplémentaires ──────────────────────

        StorePlugin {
            id: "subscene",
            name: "Subscene Subtitles",
            author: "Guilouz",
            description: "Agent sous-titres depuis Subscene.com. Grande base de données multilingue. Supporte les sous-titres français, anglais, arabe et 30+ langues.",
            category: "Sous-titres",
            tags: vec!["sous-titres", "subscene", "multilingue", "français"],
            license: "MIT",
            stars: "180",
            github_url: "https://github.com/Guilouz/Subscene.bundle",
            zip_url: "https://github.com/Guilouz/Subscene.bundle/archive/refs/heads/master.zip",
            bundle_name: "Subscene.bundle",
            verified: true,
        },
        StorePlugin {
            id: "addic7ed",
            name: "Addic7ed Subtitles",
            author: "nicedoc",
            description: "Sous-titres depuis Addic7ed.com. Spécialisé dans les séries TV américaines. Synchronisation rapide après diffusion.",
            category: "Sous-titres",
            tags: vec!["sous-titres", "addic7ed", "séries", "anglais"],
            license: "MIT",
            stars: "140",
            github_url: "https://github.com/nicedoc/Addic7ed.bundle",
            zip_url: "https://github.com/nicedoc/Addic7ed.bundle/archive/refs/heads/master.zip",
            bundle_name: "Addic7ed.bundle",
            verified: true,
        },
        StorePlugin {
            id: "legendastv",
            name: "Legendas.tv Subtitles",
            author: "Coders Paradise",
            description: "Sous-titres depuis Legendas.tv. Principale source de sous-titres en portugais (Brésil et Portugal). Indispensable pour la communauté lusophone.",
            category: "Sous-titres",
            tags: vec!["sous-titres", "portugais", "brésil", "legendas"],
            license: "MIT",
            stars: "95",
            github_url: "https://github.com/Coders-Paradise/Legendas.tv.bundle",
            zip_url: "https://github.com/Coders-Paradise/Legendas.tv.bundle/archive/refs/heads/master.zip",
            bundle_name: "Legendas.tv.bundle",
            verified: true,
        },

        // ── Métadonnées supplémentaires ──────────────────────

        StorePlugin {
            id: "fanart-tv",
            name: "Fanart.tv",
            author: "saltydk",
            description: "Posters, fonds d'écran et banners en très haute qualité depuis Fanart.tv. Complète TMDB/TVDB avec des assets artistiques uniques.",
            category: "Métadonnées",
            tags: vec!["posters", "fanart", "haute qualité", "artwork"],
            license: "MIT",
            stars: "420",
            github_url: "https://github.com/saltydk/Fanart.tv.bundle",
            zip_url: "https://github.com/saltydk/Fanart.tv.bundle/archive/refs/heads/master.zip",
            bundle_name: "Fanart.tv.bundle",
            verified: true,
        },
        StorePlugin {
            id: "audnexus",
            name: "Audnexus — Audiobooks",
            author: "djdembeck",
            description: "Agent métadonnées pour livres audio. Récupère auteurs, narrateurs, résumés, jaquettes depuis Audible/Amazon. Parfait pour organiser ta bibliothèque audio.",
            category: "Audiobooks",
            tags: vec!["audiobooks", "livres audio", "audible", "narrateurs"],
            license: "MIT",
            stars: "380",
            github_url: "https://github.com/djdembeck/Audnexus.bundle",
            zip_url: "https://github.com/djdembeck/Audnexus.bundle/archive/refs/heads/master.zip",
            bundle_name: "Audnexus.bundle",
            verified: true,
        },
        StorePlugin {
            id: "musicbrainz",
            name: "MusicBrainz Agent",
            author: "nicedoc",
            description: "Agent musique MusicBrainz. Base de données musicale open-source. Biographies artistes, metadata albums, MBID. Alternative libre à Last.fm.",
            category: "Musique",
            tags: vec!["musique", "musicbrainz", "open-source", "albums"],
            license: "MIT",
            stars: "160",
            github_url: "https://github.com/nicedoc/MusicBrainz.bundle",
            zip_url: "https://github.com/nicedoc/MusicBrainz.bundle/archive/refs/heads/master.zip",
            bundle_name: "MusicBrainz.bundle",
            verified: true,
        },
        StorePlugin {
            id: "betaseries",
            name: "BetaSeries Agent",
            author: "Kazamajinz",
            description: "Agent métadonnées depuis BetaSeries.com. Plateforme française de suivi de séries TV. Parfait pour la communauté francophone.",
            category: "Métadonnées",
            tags: vec!["séries", "français", "betaseries", "france"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/Kazamajinz/BetaSeries.bundle",
            zip_url: "https://github.com/Kazamajinz/BetaSeries.bundle/archive/refs/heads/master.zip",
            bundle_name: "BetaSeries.bundle",
            verified: true,
        },
        StorePlugin {
            id: "myanimelist",
            name: "MyAnimeList Agent",
            author: "Fribb",
            description: "Agent métadonnées anime depuis MyAnimeList.net. Titres, synopsis, genres, scores, studios depuis la plus grande base de données anime.",
            category: "Métadonnées",
            tags: vec!["anime", "myanimelist", "MAL", "métadonnées"],
            license: "GPL-3.0",
            stars: "195",
            github_url: "https://github.com/Fribb/MyAnimeList.bundle",
            zip_url: "https://github.com/Fribb/MyAnimeList.bundle/archive/refs/heads/master.zip",
            bundle_name: "MyAnimeList.bundle",
            verified: true,
        },
        StorePlugin {
            id: "anilist-agent",
            name: "AniList Agent",
            author: "SpaceK33z",
            description: "Agent anime/manga AniList. Séries, films d'animation, OVA/ONA. Couvre les œuvres récentes et moins connues. Sans clé API.",
            category: "Métadonnées",
            tags: vec!["anime", "AniList", "manga", "sans clé"],
            license: "MIT",
            stars: "175",
            github_url: "https://github.com/SpaceK33z/AniList.bundle",
            zip_url: "https://github.com/SpaceK33z/AniList.bundle/archive/refs/heads/master.zip",
            bundle_name: "AniList.bundle",
            verified: true,
        },
        StorePlugin {
            id: "comicvine",
            name: "ComicVine Agent",
            author: "Cory-M",
            description: "Agent métadonnées pour comics et BD. Récupère depuis Comic Vine : titres, arcs narratifs, personnages, couvertures. Support Marvel, DC et indépendants.",
            category: "Comics",
            tags: vec!["comics", "BD", "manga", "comicvine", "marvel", "dc"],
            license: "MIT",
            stars: "280",
            github_url: "https://github.com/Cory-M/ComicVine.bundle",
            zip_url: "https://github.com/Cory-M/ComicVine.bundle/archive/refs/heads/master.zip",
            bundle_name: "ComicVine.bundle",
            verified: true,
        },

        // ── IPTV / Streaming ─────────────────────────────────

        StorePlugin {
            id: "iptv",
            name: "IPTV",
            author: "Cigaras",
            description: "Lecteur IPTV pour Plex. Lit des listes M3U de chaînes TV en direct. Supporte les EPG (guides des programmes). Compatible avec la plupart des fournisseurs IPTV.",
            category: "IPTV",
            tags: vec!["iptv", "tv live", "m3u", "epg", "streaming"],
            license: "LGPL-3.0",
            stars: "680",
            github_url: "https://github.com/Cigaras/IPTV.bundle",
            zip_url: "https://github.com/Cigaras/IPTV.bundle/archive/refs/heads/master.zip",
            bundle_name: "IPTV.bundle",
            verified: true,
        },

        // ── Utilitaires supplémentaires ──────────────────────

        StorePlugin {
            id: "cinemavision",
            name: "CinemaVision",
            author: "Veritas83",
            description: "Crée des expériences cinéma complètes dans Plex. Ajoute des intros, trailers, bumpers et countdowns avant tes films. Ambiance salle de cinéma chez toi.",
            category: "Utilitaires",
            tags: vec!["cinéma", "intro", "trailer", "expérience", "ambiance"],
            license: "MIT",
            stars: "490",
            github_url: "https://github.com/Veritas83/CinemaVision",
            zip_url: "https://github.com/Veritas83/CinemaVision/archive/refs/heads/master.zip",
            bundle_name: "CinemaVision.bundle",
            verified: true,
        },
        StorePlugin {
            id: "xbmc-nfo",
            name: "XBMC/Kodi NFO Agent",
            author: "Plex Community",
            description: "Lit les fichiers .nfo au format XBMC/Kodi pour importer les métadonnées directement. Idéal si tu migres depuis Kodi ou si tu génères des NFO avec d'autres outils.",
            category: "Utilitaires",
            tags: vec!["kodi", "xbmc", "nfo", "import", "migration"],
            license: "MIT",
            stars: "320",
            github_url: "https://github.com/gboudreau/XBMCnfoMoviesImporter.bundle",
            zip_url: "https://github.com/gboudreau/XBMCnfoMoviesImporter.bundle/archive/refs/heads/master.zip",
            bundle_name: "XBMCnfoMoviesImporter.bundle",
            verified: true,
        },
        StorePlugin {
            id: "xbmc-nfo-tv",
            name: "XBMC/Kodi NFO TV Agent",
            author: "gboudreau",
            description: "Version séries TV de l'agent NFO XBMC/Kodi. Importe les métadonnées épisodes depuis les .nfo générés par Kodi, MediaElch ou autres outils.",
            category: "Utilitaires",
            tags: vec!["kodi", "xbmc", "nfo", "séries", "migration"],
            license: "MIT",
            stars: "280",
            github_url: "https://github.com/gboudreau/XBMCnfoTVImporter.bundle",
            zip_url: "https://github.com/gboudreau/XBMCnfoTVImporter.bundle/archive/refs/heads/master.zip",
            bundle_name: "XBMCnfoTVImporter.bundle",
            verified: true,
        },
        StorePlugin {
            id: "podcast",
            name: "Podcast Agent",
            author: "npertschy",
            description: "Gère les podcasts dans Plex. Télécharge et organise les épisodes automatiquement depuis les flux RSS. Artwork, descriptions et chapitres inclus.",
            category: "Podcast",
            tags: vec!["podcast", "rss", "audio", "épisodes"],
            license: "MIT",
            stars: "145",
            github_url: "https://github.com/npertschy/podcast-agent",
            zip_url: "https://github.com/npertschy/podcast-agent/archive/refs/heads/master.zip",
            bundle_name: "podcast-agent.bundle",
            verified: true,
        },
        StorePlugin {
            id: "personal-media-movies",
            name: "Personal Media Movies",
            author: "Plex Inc.",
            description: "Agent officiel Plex pour vidéos personnelles (films maison, enregistrements). Organise sans recherche de métadonnées en ligne. Utilise les noms de fichiers.",
            category: "Utilitaires",
            tags: vec!["personnel", "home vidéo", "local", "officiel"],
            license: "MIT",
            stars: "190",
            github_url: "https://github.com/plexinc-plugins/PlexMovie.bundle",
            zip_url: "https://github.com/plexinc-plugins/PlexMovie.bundle/archive/refs/heads/master.zip",
            bundle_name: "PlexMovie.bundle",
            verified: true,
        },
        StorePlugin {
            id: "trakt-scrobbler",
            name: "Trakt Scrobbler",
            author: "trakt",
            description: "Synchronise automatiquement tes visionnages Plex avec Trakt.tv. Marque les films et épisodes comme vus, suit ta progression et génère des statistiques.",
            category: "Sync",
            tags: vec!["trakt", "sync", "scrobble", "statistiques", "historique"],
            license: "MIT",
            stars: "520",
            github_url: "https://github.com/trakt/Plex-Trakt-Sync",
            zip_url: "https://github.com/trakt/Plex-Trakt-Sync/archive/refs/heads/master.zip",
            bundle_name: "Plex-Trakt-Sync.bundle",
            verified: true,
        },
        StorePlugin {
            id: "letterboxd-sync",
            name: "Letterboxd Sync",
            author: "memetb",
            description: "Synchronise les visionnages et notes Plex avec Letterboxd. Exporte automatiquement les films vus et leurs notes vers ton profil Letterboxd.",
            category: "Sync",
            tags: vec!["letterboxd", "sync", "films", "notes", "journal"],
            license: "MIT",
            stars: "160",
            github_url: "https://github.com/memetb/letterboxd-plex-sync",
            zip_url: "https://github.com/memetb/letterboxd-plex-sync/archive/refs/heads/master.zip",
            bundle_name: "letterboxd-plex-sync.bundle",
            verified: true,
        },

        // ── Scanners ─────────────────────────────────────────

        StorePlugin {
            id: "extended-personal-scanner",
            name: "Extended Personal Media Scanner",
            author: "bnewbold",
            description: "Scanner avancé pour médias personnels. Reconnaît les structures de dossiers non-standard. Compagne de l'agent Extended Personal Media.",
            category: "Scanner",
            tags: vec!["scanner", "personnel", "home vidéo", "structure"],
            license: "Apache-2.0",
            stars: "115",
            github_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle",
            zip_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle/archive/refs/heads/master.zip",
            bundle_name: "ExtendedPersonalMedia-Scanner.bundle",
            verified: true,
        },
    ]
}

// ─── Install ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallResult {
    pub bundle_path: String,
    pub bundle_name: String,
    pub already_existed: bool,
}

pub async fn download_and_install(
    zip_url: &str,
    bundle_name: &str,
    plugins_dir: &PathBuf,
) -> Result<InstallResult> {
    // Cible finale : plugins_dir / bundle_name
    let target = plugins_dir.join(bundle_name);
    let already_existed = target.exists();

    // Backup si déjà installé
    if already_existed {
        crate::scanner::backup_plugin(&target).ok();
        std::fs::remove_dir_all(&target)?;
    }

    std::fs::create_dir_all(plugins_dir)?;

    // Télécharger le ZIP
    let resp = reqwest::get(zip_url)
        .await
        .map_err(PlexMetaForgeError::Http)?;

    if !resp.status().is_success() {
        return Err(PlexMetaForgeError::PlexApi(format!(
            "Téléchargement échoué HTTP {} — {}",
            resp.status(),
            zip_url
        )));
    }

    let bytes = resp.bytes().await.map_err(PlexMetaForgeError::Http)?.to_vec();

    // Extraire le ZIP
    let cursor = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;

    // Trouve le dossier racine dans le ZIP (ex: "Hama.bundle-master/")
    let zip_root = find_zip_root(&mut archive, bundle_name)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw_path = file.name().to_string();

        // Retire le préfixe racine du ZIP
        let relative = match raw_path.strip_prefix(&zip_root) {
            Some(r) => r,
            None => continue,
        };

        if relative.is_empty() {
            continue;
        }

        let dest = target.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));

        if raw_path.ends_with('/') {
            std::fs::create_dir_all(&dest)?;
        } else {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            std::fs::write(&dest, data)?;
        }
    }

    Ok(InstallResult {
        bundle_path: target.to_string_lossy().to_string(),
        bundle_name: bundle_name.to_string(),
        already_existed,
    })
}

fn find_zip_root(archive: &mut zip::ZipArchive<std::io::Cursor<Vec<u8>>>, bundle_name: &str) -> Result<String> {
    // Cherche un dossier dont le nom contient le bundle_name (sans .bundle)
    let stem = bundle_name.trim_end_matches(".bundle").to_lowercase();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        if name.ends_with('/') {
            let folder = name.trim_end_matches('/').to_lowercase();
            // ex: "hama.bundle-master" contient "hama"
            if folder.contains(&stem) || folder.contains("master") || folder.contains("main") {
                return Ok(name.to_string());
            }
        }
    }

    // Fallback : prend le premier dossier racine
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        if name.ends_with('/') && !name[..name.len()-1].contains('/') {
            return Ok(name);
        }
    }

    Err(PlexMetaForgeError::PlexApi(
        "Impossible de trouver le dossier racine dans le ZIP".to_string()
    ))
}
