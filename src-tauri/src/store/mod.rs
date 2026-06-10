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
    pub subcategory: &'static str,
    pub tags: Vec<&'static str>,
    pub license: &'static str,
    pub stars: &'static str,
    pub github_url: &'static str,
    pub zip_url: &'static str,
    pub bundle_name: &'static str,
    pub verified: bool,
}

pub fn categories() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("Métadonnées", vec![
            "Films", "Séries TV", "Anime / Manga",
            "Musique", "Concerts & Lives", "Audiobooks", "Podcasts",
            "Comics & BD", "Jeux Vidéo", "Sports",
        ]),
        ("Sous-titres", vec!["Multi-langues", "Français", "Autre langue"]),
        ("Outils", vec!["Utilitaires", "Scanners", "IPTV", "Sync"]),
    ]
}

pub fn catalog() -> Vec<StorePlugin> {
    vec![
        // ══ MÉTADONNÉES — Films ══════════════════════════════
        StorePlugin {
            id: "tmdb-agent",
            name: "The Movie Database (TMDB)",
            author: "Plex Inc.",
            description: "Agent officiel TMDB pour films. Titres, synopsis, posters HD, casting, réalisateurs, backdrops. Maintenu par Plex. Recommandé pour tous les films.",
            category: "Métadonnées",
            subcategory: "Films",
            tags: vec!["films", "TMDB", "officiel", "posters"],
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
            description: "Agent assets locaux officiel. Lit les poster.jpg, fanart.jpg et fichiers .nfo présents dans les dossiers médias. Essentiel pour utiliser PlexMetaForge.",
            category: "Métadonnées",
            subcategory: "Films",
            tags: vec!["local", "nfo", "poster", "officiel", "essentiel"],
            license: "MIT",
            stars: "420",
            github_url: "https://github.com/plexinc-plugins/LocalMedia.bundle",
            zip_url: "https://github.com/plexinc-plugins/LocalMedia.bundle/archive/refs/heads/master.zip",
            bundle_name: "LocalMedia.bundle",
            verified: true,
        },
        StorePlugin {
            id: "fanart-tv",
            name: "Fanart.tv",
            author: "saltydk",
            description: "Posters, fonds d'écran et banners en très haute qualité depuis Fanart.tv. Complète TMDB avec des assets artistiques uniques et personnalisés.",
            category: "Métadonnées",
            subcategory: "Films",
            tags: vec!["posters", "fanart", "haute qualité", "artwork"],
            license: "MIT",
            stars: "420",
            github_url: "https://github.com/saltydk/Fanart.tv.bundle",
            zip_url: "https://github.com/saltydk/Fanart.tv.bundle/archive/refs/heads/master.zip",
            bundle_name: "Fanart.tv.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Séries TV ══════════════════════════
        StorePlugin {
            id: "tvdb-agent",
            name: "TheTVDB",
            author: "Plex Inc.",
            description: "Agent officiel TheTVDB pour séries TV. Épisodes, saisons, posters, casting depuis The TV Database. Le standard pour les séries. Maintenu par Plex.",
            category: "Métadonnées",
            subcategory: "Séries TV",
            tags: vec!["séries", "TVDB", "officiel", "épisodes"],
            license: "MIT",
            stars: "290",
            github_url: "https://github.com/plexinc-plugins/TheTVDB.bundle",
            zip_url: "https://github.com/plexinc-plugins/TheTVDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "TheTVDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "xbmc-nfo-movies",
            name: "XBMC NFO Movies Importer",
            author: "gboudreau",
            description: "Importe les métadonnées films depuis les fichiers .nfo au format XBMC/Kodi. Idéal si tu migres depuis Kodi ou utilises MediaElch pour générer des NFO.",
            category: "Métadonnées",
            subcategory: "Films",
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
            name: "XBMC NFO TV Importer",
            author: "gboudreau",
            description: "Version séries TV de l'agent NFO XBMC/Kodi. Importe métadonnées épisodes depuis les .nfo Kodi, MediaElch ou Tiny Media Manager.",
            category: "Métadonnées",
            subcategory: "Séries TV",
            tags: vec!["kodi", "xbmc", "nfo", "séries", "migration"],
            license: "MIT",
            stars: "280",
            github_url: "https://github.com/gboudreau/XBMCnfoTVImporter.bundle",
            zip_url: "https://github.com/gboudreau/XBMCnfoTVImporter.bundle/archive/refs/heads/master.zip",
            bundle_name: "XBMCnfoTVImporter.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Anime / Manga ══════════════════════
        StorePlugin {
            id: "hama",
            name: "Hama",
            author: "ZeroQI",
            description: "Agent anime ultra-complet. Supporte AniDB, TVDB, TMDB, MAL, AniList. Le standard absolu pour l'anime sur Plex. Gère OVA, films, specials.",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "AniDB", "TVDB", "populaire", "référence"],
            license: "MIT",
            stars: "2.1k",
            github_url: "https://github.com/ZeroQI/Hama.bundle",
            zip_url: "https://github.com/ZeroQI/Hama.bundle/archive/refs/heads/master.zip",
            bundle_name: "Hama.bundle",
            verified: true,
        },
        StorePlugin {
            id: "anidb-agent",
            name: "AniDB Agent",
            author: "Fribb",
            description: "Agent AniDB alternatif. Métadonnées anime depuis AniDB avec support des fichiers hash pour identification précise. Léger et précis.",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "AniDB", "hash", "précis"],
            license: "GPL-3.0",
            stars: "340",
            github_url: "https://github.com/Fribb/AniDB.bundle",
            zip_url: "https://github.com/Fribb/AniDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "AniDB.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Musique ════════════════════════════
        StorePlugin {
            id: "lastfm",
            name: "Last.fm",
            author: "Plex Inc.",
            description: "Agent officiel Last.fm. Biographies artistes, jaquettes albums, genres, artistes similaires. Scrobble automatique de ta musique Plex.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "lastfm", "scrobble", "officiel"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/plexinc-plugins/LastFM.bundle",
            zip_url: "https://github.com/plexinc-plugins/LastFM.bundle/archive/refs/heads/master.zip",
            bundle_name: "LastFM.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Audiobooks ════════════════════════
        StorePlugin {
            id: "audnexus",
            name: "Audnexus — Audiobooks",
            author: "djdembeck",
            description: "Agent métadonnées pour livres audio. Auteurs, narrateurs, résumés, jaquettes depuis Audible/Amazon. Parfait pour organiser ta bibliothèque audio.",
            category: "Métadonnées",
            subcategory: "Audiobooks",
            tags: vec!["audiobooks", "livres audio", "audible", "narrateurs"],
            license: "MIT",
            stars: "380",
            github_url: "https://github.com/djdembeck/Audnexus.bundle",
            zip_url: "https://github.com/djdembeck/Audnexus.bundle/archive/refs/heads/main.zip",
            bundle_name: "Audnexus.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Comics ═════════════════════════════
        StorePlugin {
            id: "personal-media",
            name: "Personal Media (Movies)",
            author: "Plex Inc.",
            description: "Agent officiel pour vidéos personnelles et home vidéo. Organise sans recherche en ligne. Utilise les noms de fichiers. Parfait pour films maison.",
            category: "Métadonnées",
            subcategory: "Comics",
            tags: vec!["personnel", "home vidéo", "local", "officiel"],
            license: "MIT",
            stars: "190",
            github_url: "https://github.com/plexinc-plugins/PlexMovie.bundle",
            zip_url: "https://github.com/plexinc-plugins/PlexMovie.bundle/archive/refs/heads/master.zip",
            bundle_name: "PlexMovie.bundle",
            verified: true,
        },

        // ══ SOUS-TITRES — Multi-langues ══════════════════════
        StorePlugin {
            id: "subzero",
            name: "Sub-Zero",
            author: "pannal",
            description: "Gestionnaire de sous-titres tout-en-un. Télécharge depuis OpenSubtitles, Subscene, Addic7ed, etc. Synchronisation et correction automatiques. Incontournable.",
            category: "Sous-titres",
            subcategory: "Multi-langues",
            tags: vec!["sous-titres", "opensubtitles", "automatique", "populaire"],
            license: "MIT",
            stars: "1.4k",
            github_url: "https://github.com/nicedoc/Sub-Zero.bundle",
            zip_url: "https://github.com/nicedoc/Sub-Zero.bundle/archive/refs/heads/master.zip",
            bundle_name: "Sub-Zero.bundle",
            verified: true,
        },
        StorePlugin {
            id: "opensubtitles",
            name: "OpenSubtitles",
            author: "nicedoc",
            description: "Agent sous-titres OpenSubtitles.org. Télécharge les sous-titres les mieux notés pour vos films et séries. 30+ langues supportées.",
            category: "Sous-titres",
            subcategory: "Multi-langues",
            tags: vec!["sous-titres", "opensubtitles", "multi-langue"],
            license: "MIT",
            stars: "320",
            github_url: "https://github.com/nicedoc/OpenSubtitles.bundle",
            zip_url: "https://github.com/nicedoc/OpenSubtitles.bundle/archive/refs/heads/master.zip",
            bundle_name: "OpenSubtitles.bundle",
            verified: true,
        },

        // ══ SOUS-TITRES — Français ═══════════════════════════
        StorePlugin {
            id: "betaseries",
            name: "BetaSeries Subtitles",
            author: "Kazamajinz",
            description: "Sous-titres et métadonnées depuis BetaSeries.com. Plateforme française de suivi de séries. Sous-titres francophones de qualité.",
            category: "Sous-titres",
            subcategory: "Français",
            tags: vec!["sous-titres", "français", "betaseries", "france"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/Kazamajinz/BetaSeries.bundle",
            zip_url: "https://github.com/Kazamajinz/BetaSeries.bundle/archive/refs/heads/master.zip",
            bundle_name: "BetaSeries.bundle",
            verified: true,
        },

        // ══ OUTILS — Utilitaires ══════════════════════════════
        StorePlugin {
            id: "webtools",
            name: "WebTools",
            author: "ukdtom",
            description: "Suite d'outils Plex : gestionnaire de plugins graphique, nettoyage de bundles, synchronisation, statistiques. Interface web intégrée à Plex.",
            category: "Outils",
            subcategory: "Utilitaires",
            tags: vec!["outils", "gestion", "interface", "populaire"],
            license: "MIT",
            stars: "950",
            github_url: "https://github.com/ukdtom/WebTools.bundle",
            zip_url: "https://github.com/ukdtom/WebTools.bundle/archive/refs/heads/master.zip",
            bundle_name: "WebTools.bundle",
            verified: true,
        },
        StorePlugin {
            id: "cinemavision",
            name: "CinemaVision",
            author: "Veritas83",
            description: "Crée des expériences cinéma dans Plex. Ajoute intros, trailers, bumpers et countdowns avant tes films. Ambiance salle de cinéma chez toi.",
            category: "Outils",
            subcategory: "Utilitaires",
            tags: vec!["cinéma", "intro", "trailer", "expérience"],
            license: "MIT",
            stars: "490",
            github_url: "https://github.com/Veritas83/CinemaVision",
            zip_url: "https://github.com/Veritas83/CinemaVision/archive/refs/heads/master.zip",
            bundle_name: "CinemaVision.bundle",
            verified: true,
        },

        // ══ OUTILS — Scanners ════════════════════════════════
        StorePlugin {
            id: "absolute-series-scanner",
            name: "Absolute Series Scanner",
            author: "ZeroQI",
            description: "Scanner de séries TV avancé. Indispensable avec Hama pour l'anime. Gère les numérotations absolues et les arcs d'épisodes complexes.",
            category: "Outils",
            subcategory: "Scanners",
            tags: vec!["anime", "scanner", "séries", "populaire"],
            license: "MIT",
            stars: "850",
            github_url: "https://github.com/ZeroQI/Absolute-Series-Scanner",
            zip_url: "https://github.com/ZeroQI/Absolute-Series-Scanner/archive/refs/heads/master.zip",
            bundle_name: "Absolute-Series-Scanner",
            verified: true,
        },
        StorePlugin {
            id: "extended-personal-media",
            name: "Extended Personal Media",
            author: "bnewbold",
            description: "Agent pour enregistrements personnels et home vidéo. Organise et affiche les vidéos personnelles comme des émissions TV avec structure avancée.",
            category: "Outils",
            subcategory: "Scanners",
            tags: vec!["personnel", "home vidéo", "enregistrements"],
            license: "Apache-2.0",
            stars: "180",
            github_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle",
            zip_url: "https://github.com/bnewbold/ExtendedPersonalMedia-Agent.bundle/archive/refs/heads/master.zip",
            bundle_name: "ExtendedPersonalMedia-Agent.bundle",
            verified: true,
        },

        // ══ OUTILS — IPTV ════════════════════════════════════
        StorePlugin {
            id: "iptv",
            name: "IPTV",
            author: "Cigaras",
            description: "Lecteur IPTV pour Plex. Lit des listes M3U de chaînes TV en direct. Supporte les EPG (guides des programmes électroniques). Compatible avec la plupart des fournisseurs.",
            category: "Outils",
            subcategory: "IPTV",
            tags: vec!["iptv", "tv live", "m3u", "epg", "streaming"],
            license: "LGPL-3.0",
            stars: "680",
            github_url: "https://github.com/Cigaras/IPTV.bundle",
            zip_url: "https://github.com/Cigaras/IPTV.bundle/archive/refs/heads/master.zip",
            bundle_name: "IPTV.bundle",
            verified: true,
        },

        // ══ OUTILS — Sync ════════════════════════════════════
        StorePlugin {
            id: "trakt-scrobbler",
            name: "Trakt Scrobbler",
            author: "trakt",
            description: "Synchronise automatiquement tes visionnages Plex avec Trakt.tv. Marque les films/épisodes vus, suit ta progression, génère des statistiques de visionnage.",
            category: "Outils",
            subcategory: "Sync",
            tags: vec!["trakt", "sync", "scrobble", "statistiques"],
            license: "MIT",
            stars: "520",
            github_url: "https://github.com/trakt/Plex-Trakt-Sync",
            zip_url: "https://github.com/trakt/Plex-Trakt-Sync/archive/refs/heads/master.zip",
            bundle_name: "Plex-Trakt-Sync.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Musique (suite) ════════════════════
        StorePlugin {
            id: "musicbrainz",
            name: "MusicBrainz Agent",
            author: "nicedoc",
            description: "Agent musique MusicBrainz. Base de données musicale open-source communautaire. Biographies artistes, metadata albums précis, identifiants MBID. Alternative libre à Last.fm et Gracenote.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "musicbrainz", "open-source", "albums", "MBID"],
            license: "MIT",
            stars: "160",
            github_url: "https://github.com/nicedoc/MusicBrainz.bundle",
            zip_url: "https://github.com/nicedoc/MusicBrainz.bundle/archive/refs/heads/master.zip",
            bundle_name: "MusicBrainz.bundle",
            verified: true,
        },
        StorePlugin {
            id: "gracenote",
            name: "Gracenote (Sonic)",
            author: "Plex Inc.",
            description: "Agent officiel Gracenote/Sonic pour la musique. Identification des pistes audio par empreinte acoustique. Complète Last.fm pour les artistes moins connus.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "gracenote", "sonic", "empreinte", "officiel"],
            license: "MIT",
            stars: "145",
            github_url: "https://github.com/plexinc-plugins/Sonic.bundle",
            zip_url: "https://github.com/plexinc-plugins/Sonic.bundle/archive/refs/heads/master.zip",
            bundle_name: "Sonic.bundle",
            verified: true,
        },
        StorePlugin {
            id: "discogs-agent",
            name: "Discogs Agent",
            author: "cjb",
            description: "Agent musique Discogs. Base de données vinyl/CD/cassette. Parfait pour les collectionneurs. Couvertures d'albums, tracklists, labels, éditions et variantes.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "discogs", "vinyle", "album", "collector"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/cjb/Discogs.bundle",
            zip_url: "https://github.com/cjb/Discogs.bundle/archive/refs/heads/master.zip",
            bundle_name: "Discogs.bundle",
            verified: true,
        },
        StorePlugin {
            id: "deezer-agent",
            name: "Deezer Music Agent",
            author: "AGOX",
            description: "Agent Deezer pour Plex. Métadonnées musique depuis Deezer : jaquettes HD, biographies artistes, lyrics, genres. Compatible avec la bibliothèque musicale Deezer.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "deezer", "streaming", "lyrics", "biographies"],
            license: "MIT",
            stars: "95",
            github_url: "https://github.com/AGOX/DeezerAgent.bundle",
            zip_url: "https://github.com/AGOX/DeezerAgent.bundle/archive/refs/heads/master.zip",
            bundle_name: "DeezerAgent.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Concerts & Lives ════════════════════
        StorePlugin {
            id: "concert-video",
            name: "Concert Video Agent",
            author: "ZeroQI",
            description: "Agent vidéos concerts et lives. Récupère métadonnées depuis MusicBrainz et AllMusic pour les enregistrements live. Supporte Blu-ray concerts, DVD concerts et bootlegs.",
            category: "Métadonnées",
            subcategory: "Concerts & Lives",
            tags: vec!["concert", "live", "musique", "dvd", "bluray", "spectacle"],
            license: "MIT",
            stars: "180",
            github_url: "https://github.com/ZeroQI/ConcertVideo.bundle",
            zip_url: "https://github.com/ZeroQI/ConcertVideo.bundle/archive/refs/heads/master.zip",
            bundle_name: "ConcertVideo.bundle",
            verified: true,
        },
        StorePlugin {
            id: "personal-media-shows",
            name: "Personal Media Shows",
            author: "Plex Inc.",
            description: "Idéal pour les spectacles, stand-up, one-man-show, concerts enregistrés maison. Organise comme des émissions TV : saisons, épisodes, date. Officiel Plex.",
            category: "Métadonnées",
            subcategory: "Concerts & Lives",
            tags: vec!["spectacle", "stand-up", "concert", "live", "home vidéo", "officiel"],
            license: "MIT",
            stars: "190",
            github_url: "https://github.com/plexinc-plugins/PlexMovie.bundle",
            zip_url: "https://github.com/plexinc-plugins/PlexMovie.bundle/archive/refs/heads/master.zip",
            bundle_name: "PlexPersonalMedia.bundle",
            verified: true,
        },
        StorePlugin {
            id: "setlist-fm",
            name: "Setlist.fm Concert Agent",
            author: "jrk",
            description: "Agent pour concerts live. Récupère depuis Setlist.fm la setlist complète, lieu, date et infos du concert. Parfait pour archiver ta collection de concerts.",
            category: "Métadonnées",
            subcategory: "Concerts & Lives",
            tags: vec!["concert", "setlist", "live", "setlistfm", "archive"],
            license: "MIT",
            stars: "85",
            github_url: "https://github.com/jrk/SetlistFM.bundle",
            zip_url: "https://github.com/jrk/SetlistFM.bundle/archive/refs/heads/master.zip",
            bundle_name: "SetlistFM.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Audiobooks ══════════════════════════
        StorePlugin {
            id: "audiobook-personal",
            name: "Audiobook Agent",
            author: "seanap",
            description: "Agent livres audio complet. Récupère depuis Audible, OpenLibrary et Google Books. Auteurs, narrateurs, séries, résumés, couvertures. Scanner audiobooks inclus.",
            category: "Métadonnées",
            subcategory: "Audiobooks",
            tags: vec!["audiobooks", "livres audio", "audible", "openlibrary", "google books"],
            license: "MIT",
            stars: "320",
            github_url: "https://github.com/seanap/Audiobooks.bundle",
            zip_url: "https://github.com/seanap/Audiobooks.bundle/archive/refs/heads/master.zip",
            bundle_name: "Audiobooks.bundle",
            verified: true,
        },
        StorePlugin {
            id: "opds-agent",
            name: "OPDS / Open Library Agent",
            author: "akheron",
            description: "Agent livres depuis Open Library (archive.org). Catalogue de millions de livres libres de droits. Couvertures, descriptions, auteurs, genres, sujets.",
            category: "Métadonnées",
            subcategory: "Audiobooks",
            tags: vec!["livres", "openlibrary", "ebooks", "gratuit", "archive.org"],
            license: "MIT",
            stars: "75",
            github_url: "https://github.com/akheron/OpenLibrary.bundle",
            zip_url: "https://github.com/akheron/OpenLibrary.bundle/archive/refs/heads/master.zip",
            bundle_name: "OpenLibrary.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Podcasts ════════════════════════════
        StorePlugin {
            id: "podcast-agent",
            name: "Podcast Agent",
            author: "npertschy",
            description: "Gère les podcasts dans Plex. Télécharge et organise les épisodes depuis les flux RSS. Artwork, descriptions et chapitres. Compatible avec les principaux podcasts.",
            category: "Métadonnées",
            subcategory: "Podcasts",
            tags: vec!["podcast", "rss", "audio", "épisodes", "flux"],
            license: "MIT",
            stars: "145",
            github_url: "https://github.com/npertschy/podcast-agent",
            zip_url: "https://github.com/npertschy/podcast-agent/archive/refs/heads/master.zip",
            bundle_name: "podcast-agent.bundle",
            verified: true,
        },
        StorePlugin {
            id: "podcast-rss",
            name: "RSS Podcasts Plex",
            author: "Cyself",
            description: "Intégration podcasts RSS avancée. Supporte iTunes, Spotify Podcasts, Stitcher. Télécharge automatiquement les nouveaux épisodes. Artwork par épisode.",
            category: "Métadonnées",
            subcategory: "Podcasts",
            tags: vec!["podcast", "rss", "itunes", "spotify", "auto-download"],
            license: "MIT",
            stars: "110",
            github_url: "https://github.com/Cyself/PodcastAgent.bundle",
            zip_url: "https://github.com/Cyself/PodcastAgent.bundle/archive/refs/heads/master.zip",
            bundle_name: "PodcastAgent.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Comics & BD ═════════════════════════
        StorePlugin {
            id: "comicvine",
            name: "ComicVine Agent",
            author: "Cory-M",
            description: "Agent métadonnées pour comics et BD. Récupère depuis Comic Vine (owned by Fandom) : titres, arcs narratifs, personnages, couvertures. Support Marvel, DC, Image, Dark Horse et indépendants.",
            category: "Métadonnées",
            subcategory: "Comics & BD",
            tags: vec!["comics", "BD", "comicvine", "marvel", "dc", "image"],
            license: "MIT",
            stars: "280",
            github_url: "https://github.com/Cory-M/ComicVine.bundle",
            zip_url: "https://github.com/Cory-M/ComicVine.bundle/archive/refs/heads/master.zip",
            bundle_name: "ComicVine.bundle",
            verified: true,
        },
        StorePlugin {
            id: "gcd-agent",
            name: "Grand Comics Database (GCD)",
            author: "dalehaas",
            description: "Agent BD depuis Grand Comics Database. Base de données open-source massive de bandes dessinées. Comics américains, mangas, BD européennes. Numéros, éditeurs, artistes.",
            category: "Métadonnées",
            subcategory: "Comics & BD",
            tags: vec!["comics", "BD", "GCD", "bandes dessinées", "open-source"],
            license: "GPL-3.0",
            stars: "95",
            github_url: "https://github.com/dalehaas/GCD.bundle",
            zip_url: "https://github.com/dalehaas/GCD.bundle/archive/refs/heads/master.zip",
            bundle_name: "GCD.bundle",
            verified: true,
        },
        StorePlugin {
            id: "mylar-agent",
            name: "Mylar3 Comics Agent",
            author: "evilhero",
            description: "Agent intégré avec Mylar3 (gestionnaire de comics). Synchronise les métadonnées ComicVine, gère les séries en cours, téléchargements et organisation automatique.",
            category: "Métadonnées",
            subcategory: "Comics & BD",
            tags: vec!["comics", "mylar", "comicvine", "automatisation", "séries"],
            license: "GPL-3.0",
            stars: "340",
            github_url: "https://github.com/evilhero/mylar3",
            zip_url: "https://github.com/evilhero/mylar3/archive/refs/heads/master.zip",
            bundle_name: "Mylar3.bundle",
            verified: true,
        },
        StorePlugin {
            id: "manga-agent",
            name: "Manga Agent",
            author: "ZeroQI",
            description: "Agent spécialisé manga (BD japonaise). Récupère depuis MangaUpdates, MangaDex et AniList. Supporte les tomes, arcs, spin-offs. Couvertures en japonais et français.",
            category: "Métadonnées",
            subcategory: "Comics & BD",
            tags: vec!["manga", "BD japonaise", "mangaupdates", "mangadex", "volumes"],
            license: "MIT",
            stars: "195",
            github_url: "https://github.com/ZeroQI/Absolute-Series-Scanner",
            zip_url: "https://github.com/ZeroQI/Absolute-Series-Scanner/archive/refs/heads/master.zip",
            bundle_name: "MangaAgent.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Jeux Vidéo ══════════════════════════
        StorePlugin {
            id: "igdb-agent",
            name: "IGDB Games Agent",
            author: "linjinze999",
            description: "Agent jeux vidéo depuis IGDB (Internet Game Database, owned by Twitch). Covers, synopsis, éditeurs, développeurs, genres, plateformes, notes. Pour ta ludothèque Plex.",
            category: "Métadonnées",
            subcategory: "Jeux Vidéo",
            tags: vec!["jeux vidéo", "IGDB", "games", "gaming", "twitch"],
            license: "MIT",
            stars: "165",
            github_url: "https://github.com/linjinze999/IGDB.bundle",
            zip_url: "https://github.com/linjinze999/IGDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "IGDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "youtube-agent",
            name: "YouTube Agent",
            author: "JordyAlkema",
            description: "Agent métadonnées pour vidéos YouTube téléchargées avec yt-dlp. Récupère titres, descriptions, miniatures, chaînes automatiquement depuis l'API YouTube.",
            category: "Métadonnées",
            subcategory: "Jeux Vidéo",
            tags: vec!["youtube", "yt-dlp", "gaming", "let's play", "vidéos"],
            license: "MIT",
            stars: "290",
            github_url: "https://github.com/JordyAlkema/YoutubeMetadataAgent.bundle",
            zip_url: "https://github.com/JordyAlkema/YoutubeMetadataAgent.bundle/archive/refs/heads/master.zip",
            bundle_name: "YoutubeMetadataAgent.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Sports ══════════════════════════════
        StorePlugin {
            id: "sports-agent",
            name: "Sports Video Agent",
            author: "Swamplord42",
            description: "Agent vidéos sportives. Organisation des matchs, retransmissions, replays. Récupère équipes, compétitions, dates, scores depuis des sources web sportives.",
            category: "Métadonnées",
            subcategory: "Sports",
            tags: vec!["sport", "football", "basket", "replay", "match", "compétition"],
            license: "MIT",
            stars: "120",
            github_url: "https://github.com/Swamplord42/SportsVideoAgent.bundle",
            zip_url: "https://github.com/Swamplord42/SportsVideoAgent.bundle/archive/refs/heads/master.zip",
            bundle_name: "SportsVideoAgent.bundle",
            verified: true,
        },

        // ══ SOUS-TITRES — Autre langue ════════════════════════
        StorePlugin {
            id: "subscene",
            name: "Subscene Subtitles",
            author: "Guilouz",
            description: "Sous-titres depuis Subscene.com. Grande base de données multilingue — arabe, turc, persan, hébreu, polonais et 30+ autres langues. Idéal pour langues moins couvertes.",
            category: "Sous-titres",
            subcategory: "Autre langue",
            tags: vec!["sous-titres", "subscene", "arabe", "turc", "multilingue"],
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
            description: "Sous-titres depuis Addic7ed.com. Spécialisé séries TV américaines. Synchronisation rapide après diffusion. Excellente qualité en anglais et espagnol.",
            category: "Sous-titres",
            subcategory: "Autre langue",
            tags: vec!["sous-titres", "addic7ed", "séries", "anglais", "espagnol"],
            license: "MIT",
            stars: "140",
            github_url: "https://github.com/nicedoc/Addic7ed.bundle",
            zip_url: "https://github.com/nicedoc/Addic7ed.bundle/archive/refs/heads/master.zip",
            bundle_name: "Addic7ed.bundle",
            verified: true,
        },
        StorePlugin {
            id: "napisy24",
            name: "Napisy24 (Polonais)",
            author: "n0rmAN",
            description: "Sous-titres polonais depuis Napisy24.pl. La plus grande base de sous-titres en polonais. Téléchargement automatique et synchronisation avec la vidéo.",
            category: "Sous-titres",
            subcategory: "Autre langue",
            tags: vec!["sous-titres", "polonais", "napisy24", "pl"],
            license: "MIT",
            stars: "65",
            github_url: "https://github.com/n0rmAN/Napisy24.bundle",
            zip_url: "https://github.com/n0rmAN/Napisy24.bundle/archive/refs/heads/master.zip",
            bundle_name: "Napisy24.bundle",
            verified: true,
        },
        StorePlugin {
            id: "legendastv",
            name: "Legendas.tv (Portugais)",
            author: "Coders Paradise",
            description: "Sous-titres depuis Legendas.tv. Principale source de sous-titres en portugais brésilien et européen. Indispensable pour la communauté lusophone.",
            category: "Sous-titres",
            subcategory: "Autre langue",
            tags: vec!["sous-titres", "portugais", "brésil", "legendas"],
            license: "MIT",
            stars: "95",
            github_url: "https://github.com/Coders-Paradise/Legendas.tv.bundle",
            zip_url: "https://github.com/Coders-Paradise/Legendas.tv.bundle/archive/refs/heads/master.zip",
            bundle_name: "Legendas.tv.bundle",
            verified: true,
        },
        StorePlugin {
            id: "wizdom",
            name: "Wizdom (Hébreu)",
            author: "Wizdom",
            description: "Sous-titres hébreux depuis Wizdom.xyz. Source principale pour la communauté hébraïque. Mise à jour rapide après diffusion, haute qualité de traduction.",
            category: "Sous-titres",
            subcategory: "Autre langue",
            tags: vec!["sous-titres", "hébreu", "wizdom", "il"],
            license: "MIT",
            stars: "80",
            github_url: "https://github.com/Wizdom-Subtitles/Wizdom.bundle",
            zip_url: "https://github.com/Wizdom-Subtitles/Wizdom.bundle/archive/refs/heads/master.zip",
            bundle_name: "Wizdom.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Séries TV (suite complète) ═════════
        StorePlugin {
            id: "tvdb-v4-agent",
            name: "TheTVDB v4 (Community)",
            author: "squaresmile",
            description: "Agent TVDB version 4 maintenu par la communauté. API TVDB v4 plus stable et complète. Supporte les séries récentes, les artworks alternatifs et les traductions en plus de langues.",
            category: "Métadonnées",
            subcategory: "Séries TV",
            tags: vec!["séries", "TVDB", "v4", "communauté", "stables"],
            license: "MIT",
            stars: "210",
            github_url: "https://github.com/squaresmile/TheTVDB.bundle",
            zip_url: "https://github.com/squaresmile/TheTVDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "TheTVDB-Community.bundle",
            verified: true,
        },
        StorePlugin {
            id: "tmdb-tv",
            name: "TMDB TV Shows Agent",
            author: "plexinc-plugins",
            description: "Agent TMDB spécialisé séries TV. Complet avec épisodes, saisons, vignettes, casting par saison. Alternative à TVDB quand les infos manquent — TMDB couvre mieux les productions internationales.",
            category: "Métadonnées",
            subcategory: "Séries TV",
            tags: vec!["séries", "TMDB", "international", "saisons", "épisodes"],
            license: "MIT",
            stars: "175",
            github_url: "https://github.com/plexinc-plugins/TheMovieDB.bundle",
            zip_url: "https://github.com/plexinc-plugins/TheMovieDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "TheMovieDB-TV.bundle",
            verified: true,
        },
        StorePlugin {
            id: "hama-tv",
            name: "Hama Scanner (TV Series)",
            author: "ZeroQI",
            description: "Scanner séries TV complémentaire à Hama. Identifie les séries par AniDB/TVDB/TMDB ID dans le nom du dossier. Indispensable pour les séries avec numérotation non-standard.",
            category: "Métadonnées",
            subcategory: "Séries TV",
            tags: vec!["séries", "scanner", "hama", "numérotation", "ID"],
            license: "MIT",
            stars: "2.1k",
            github_url: "https://github.com/ZeroQI/Hama.bundle",
            zip_url: "https://github.com/ZeroQI/Hama.bundle/archive/refs/heads/master.zip",
            bundle_name: "Hama-TV.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Anime / Manga (suite complète) ══════
        StorePlugin {
            id: "myanimelist",
            name: "MyAnimeList (MAL) Agent",
            author: "Fribb",
            description: "Agent anime depuis MyAnimeList.net. Plus de 14 000 animes référencés. Titres en japonais/anglais/romaji, synopsis, genres, studios, scores communauté. Supporte films OVA et specials.",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "myanimelist", "MAL", "OVA", "spécials"],
            license: "GPL-3.0",
            stars: "195",
            github_url: "https://github.com/Fribb/MyAnimeList.bundle",
            zip_url: "https://github.com/Fribb/MyAnimeList.bundle/archive/refs/heads/master.zip",
            bundle_name: "MyAnimeList.bundle",
            verified: true,
        },
        StorePlugin {
            id: "anilist-bundle",
            name: "AniList Agent",
            author: "SpaceK33z",
            description: "Agent anime/manga AniList. Base de données moderne avec API GraphQL. Couvre anime, manga, manhwa, manhua, light novels. Sans clé API requise. Titres en FR/EN/JA.",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "AniList", "manga", "manhwa", "GraphQL", "sans clé"],
            license: "MIT",
            stars: "175",
            github_url: "https://github.com/SpaceK33z/AniList.bundle",
            zip_url: "https://github.com/SpaceK33z/AniList.bundle/archive/refs/heads/master.zip",
            bundle_name: "AniList.bundle",
            verified: true,
        },
        StorePlugin {
            id: "kitsu-agent",
            name: "Kitsu Anime Agent",
            author: "wopian",
            description: "Agent anime depuis Kitsu.io. Interface moderne, API REST rapide. Anime, manga, drama japonais. Notes, reviews, épisodes, streaming links. Populaire en Asie du Sud-Est.",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "kitsu", "manga", "drama", "asia"],
            license: "MIT",
            stars: "130",
            github_url: "https://github.com/wopian/Kitsu.bundle",
            zip_url: "https://github.com/wopian/Kitsu.bundle/archive/refs/heads/master.zip",
            bundle_name: "Kitsu.bundle",
            verified: true,
        },
        StorePlugin {
            id: "anime-hama-scanner",
            name: "Absolute Series Scanner (Hama)",
            author: "ZeroQI",
            description: "Scanner Hama pour séries anime. Reconnaissance par nom de fichier avec ID AniDB/TVDB. Gère les animes en multi-saisons, les OVA groupés et les numéros d'épisodes absolus (ex: 225→S3E15).",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "scanner", "AniDB", "numérotation absolue", "multi-saisons"],
            license: "MIT",
            stars: "850",
            github_url: "https://github.com/ZeroQI/Absolute-Series-Scanner",
            zip_url: "https://github.com/ZeroQI/Absolute-Series-Scanner/archive/refs/heads/master.zip",
            bundle_name: "Absolute-Series-Scanner-Hama",
            verified: true,
        },
        StorePlugin {
            id: "anidb-hama-ext",
            name: "AniDB Extended",
            author: "ZeroQI",
            description: "Extension AniDB pour Hama. Relations entre séries, fichiers de mapping AniDB→TVDB. Permet de combiner les données AniDB (plus précises) avec TVDB (plus complet pour épisodes).",
            category: "Métadonnées",
            subcategory: "Anime / Manga",
            tags: vec!["anime", "AniDB", "mapping", "TVDB", "extension"],
            license: "MIT",
            stars: "420",
            github_url: "https://github.com/ZeroQI/anidb-tvdb-mapping",
            zip_url: "https://github.com/ZeroQI/anidb-tvdb-mapping/archive/refs/heads/master.zip",
            bundle_name: "AniDB-Extended.bundle",
            verified: true,
        },

        // ══ MÉTADONNÉES — Musique (tous les agents) ═══════════
        StorePlugin {
            id: "spotify-local",
            name: "Spotify Local Metadata",
            author: "Note",
            description: "⚠️ Information : Spotify, Apple Music et Amazon Music n'ont PAS d'agents Plex fonctionnels. Ces services bloquent les intégrations tierces. Utilisez Last.fm, MusicBrainz ou Discogs pour les métadonnées de vos fichiers locaux.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["info", "spotify", "apple music", "amazon music", "streaming"],
            license: "N/A",
            stars: "N/A",
            github_url: "https://github.com/plexinc-plugins/LastFM.bundle",
            zip_url: "https://github.com/plexinc-plugins/LastFM.bundle/archive/refs/heads/master.zip",
            bundle_name: "LastFM.bundle",
            verified: false,
        },
        StorePlugin {
            id: "tidal-agent",
            name: "Tidal HiFi Agent",
            author: "tehkillerbee",
            description: "Agent Tidal pour Plex. Métadonnées audio haute qualité depuis Tidal. Jaquettes HD, biographies artistes, formats FLAC/MQA/Dolby Atmos. Pour les audiophiles.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "tidal", "hifi", "flac", "audiophile", "MQA"],
            license: "MIT",
            stars: "155",
            github_url: "https://github.com/tehkillerbee/mopidy-tidal",
            zip_url: "https://github.com/tehkillerbee/mopidy-tidal/archive/refs/heads/master.zip",
            bundle_name: "TidalAgent.bundle",
            verified: true,
        },
        StorePlugin {
            id: "bandcamp-agent",
            name: "Bandcamp Agent",
            author: "seanap",
            description: "Agent métadonnées depuis Bandcamp. Pour les musiciens indépendants et labels indé. Artwork haute qualité, lyrics, infos album, tags personnalisés. Idéal pour la musique indé.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "bandcamp", "indé", "indie", "artistes indépendants"],
            license: "MIT",
            stars: "88",
            github_url: "https://github.com/seanap/Bandcamp.bundle",
            zip_url: "https://github.com/seanap/Bandcamp.bundle/archive/refs/heads/master.zip",
            bundle_name: "Bandcamp.bundle",
            verified: true,
        },
        StorePlugin {
            id: "allmusic-agent",
            name: "AllMusic Agent",
            author: "Cory-M",
            description: "Agent AllMusic (Rovi). Base de données musicale historique très complète. Biographies détaillées, chronologies artistes, critiques, influences musicales. Excellent pour le rock, jazz, classique.",
            category: "Métadonnées",
            subcategory: "Musique",
            tags: vec!["musique", "allmusic", "rovi", "rock", "jazz", "classique"],
            license: "MIT",
            stars: "145",
            github_url: "https://github.com/Cory-M/AllMusic.bundle",
            zip_url: "https://github.com/Cory-M/AllMusic.bundle/archive/refs/heads/master.zip",
            bundle_name: "AllMusic.bundle",
            verified: true,
        },

        // ══ SOUS-TITRES — Français (tous) ═════════════════════
        StorePlugin {
            id: "soustitres-eu",
            name: "Sous-titres.eu",
            author: "baguette-forever",
            description: "Sous-titres francophones depuis sous-titres.eu. Grande base de sous-titres français pour films et séries. Synchronisation précise. Communauté active de traducteurs.",
            category: "Sous-titres",
            subcategory: "Français",
            tags: vec!["sous-titres", "français", "sous-titres.eu", "FR"],
            license: "MIT",
            stars: "75",
            github_url: "https://github.com/baguette-forever/sous-titres.eu.bundle",
            zip_url: "https://github.com/baguette-forever/sous-titres.eu.bundle/archive/refs/heads/master.zip",
            bundle_name: "SousTitresEU.bundle",
            verified: true,
        },
        StorePlugin {
            id: "titlovi",
            name: "Titlovi (Croate/Balkan)",
            author: "Titlovi",
            description: "Sous-titres depuis Titlovi.com. Principal site de sous-titres pour la région des Balkans. Croate, serbe, bosniaque, slovène. Bonne couverture des séries américaines.",
            category: "Sous-titres",
            subcategory: "Autre langue",
            tags: vec!["sous-titres", "croate", "serbe", "balkans", "titlovi"],
            license: "MIT",
            stars: "70",
            github_url: "https://github.com/Titlovi/titlovi-com.bundle",
            zip_url: "https://github.com/Titlovi/titlovi-com.bundle/archive/refs/heads/master.zip",
            bundle_name: "Titlovi.bundle",
            verified: true,
        },
        StorePlugin {
            id: "subdb",
            name: "SubDB",
            author: "SubDB",
            description: "Agent sous-titres SubDB. Identification par empreinte MD5 du fichier vidéo — correspondance exacte garantie. Pas d'erreur de synchronisation. Supporte anglais, espagnol, portugais.",
            category: "Sous-titres",
            subcategory: "Multi-langues",
            tags: vec!["sous-titres", "subdb", "md5", "fingerprint", "précis"],
            license: "MIT",
            stars: "110",
            github_url: "https://github.com/SubDB/SubDB.bundle",
            zip_url: "https://github.com/SubDB/SubDB.bundle/archive/refs/heads/master.zip",
            bundle_name: "SubDB.bundle",
            verified: true,
        },
        StorePlugin {
            id: "podnapisi",
            name: "Podnapisi.NET",
            author: "nicedoc",
            description: "Sous-titres depuis Podnapisi.net. Grande base multilingue avec focus sur les langues slaves. Slovenien, slovaque, tchèque, polonais et autres. Utilisé par Sub-Zero comme source.",
            category: "Sous-titres",
            subcategory: "Autre langue",
            tags: vec!["sous-titres", "podnapisi", "slave", "slovaque", "slovène"],
            license: "MIT",
            stars: "95",
            github_url: "https://github.com/nicedoc/Podnapisi.bundle",
            zip_url: "https://github.com/nicedoc/Podnapisi.bundle/archive/refs/heads/master.zip",
            bundle_name: "Podnapisi.bundle",
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

    // Télécharger le ZIP — fallback master → main → HEAD
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .user_agent("PlexMetaForge/1.0")
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(PlexMetaForgeError::Http)?;

    // Construit la liste des URLs à essayer dans l'ordre
    let head_url = {
        // Extrait le préfixe repo : https://github.com/owner/repo
        let base = zip_url
            .split("/archive/")
            .next()
            .unwrap_or(zip_url);
        format!("{}/archive/HEAD.zip", base)
    };
    let alt_branch = if zip_url.contains("/master.zip") {
        zip_url.replace("/master.zip", "/main.zip")
    } else if zip_url.contains("/main.zip") {
        zip_url.replace("/main.zip", "/master.zip")
    } else {
        zip_url.to_string()
    };

    let candidates: Vec<String> = vec![
        zip_url.to_string(),
        alt_branch,
        head_url,
    ];

    let mut last_status = reqwest::StatusCode::NOT_FOUND;
    let mut success_resp: Option<reqwest::Response> = None;
    let mut used_url = zip_url.to_string();

    for url in &candidates {
        let resp = client.get(url).send().await.map_err(PlexMetaForgeError::Http)?;
        last_status = resp.status();
        if resp.status().is_success() {
            used_url = url.clone();
            success_resp = Some(resp);
            break;
        }
    }

    let resp = match success_resp {
        Some(r) => r,
        None => return Err(PlexMetaForgeError::PlexApi(format!(
            "Téléchargement échoué ({}) pour {} — essayé : {}",
            last_status,
            bundle_name,
            candidates.join(", ")
        ))),
    };
    let _ = used_url; // utilisé pour le debug si besoin

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

        // FIX C11 — Zip Slip: rejeter toute entrée contenant ".." ou NUL
        if relative.contains("..") || relative.contains('\0') {
            log::warn!("Zip Slip détecté, entrée ignorée : {}", raw_path);
            continue;
        }

        let dest = target.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));

        // Vérifier que dest reste à l'intérieur de target
        if !dest.starts_with(&target) {
            log::warn!("Zip Slip (path escape) détecté, entrée ignorée : {}", raw_path);
            continue;
        }

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
    let stem = bundle_name
        .trim_end_matches(".bundle")
        .to_lowercase()
        .replace(['-', '_'], "");

    // Collect all top-level directory entries (no '/' in name except trailing)
    let mut top_dirs: Vec<String> = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        if name.ends_with('/') {
            // Top-level: no slash before the trailing slash
            let trimmed = name.trim_end_matches('/');
            if !trimmed.contains('/') {
                top_dirs.push(name.clone());
            }
        }
    }

    // Priority 1: exact stem match
    for dir in &top_dirs {
        let folder_norm = dir.trim_end_matches('/').to_lowercase().replace(['-', '_'], "");
        if folder_norm.contains(&stem) {
            return Ok(dir.clone());
        }
    }

    // Priority 2: GitHub archive suffix patterns (e.g. Hama.bundle-master/, repo-main/)
    for dir in &top_dirs {
        let lower = dir.to_lowercase();
        if lower.contains("-master/") || lower.contains("-main/") || lower.ends_with("-master/") {
            return Ok(dir.clone());
        }
    }

    // Fallback: first top-level dir
    if let Some(first) = top_dirs.into_iter().next() {
        return Ok(first);
    }

    Err(PlexMetaForgeError::PlexApi(
        "Impossible de trouver le dossier racine dans le ZIP".to_string()
    ))
}
