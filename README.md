# PlexMetaForge

**Gestionnaire de plugins et injecteur de métadonnées pour Plex Media Server — Windows 10/11**

![Version](https://img.shields.io/badge/version-1.0.0-orange)
![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Tauri](https://img.shields.io/badge/Tauri-v2-purple)

---

## Fonctionnalités

| Module | Description |
|--------|-------------|
| 🔌 **Gestionnaire de plugins** | Lister, activer/désactiver, supprimer, sauvegarder les bundles Plex |
| 🛒 **Catalogue** | 28 plugins open-source installables en 1 clic depuis GitHub |
| 🎬 **Créateur de plugins** | 5 templates prêts à l'emploi (Films, Séries, Anime, Musique, Universel) |
| 🎬 **Injection de métadonnées** | NFO + poster + fanart + API Plex + SQLite direct |
| 🗄️ **Base de données** | Parcourir la DB SQLite Plex, voir les incomplets, libérer les verrous |
| 📝 **Éditeur de code** | Modifier le Python des plugins directement dans l'app |
| ⬇ **Export ZIP** | Exporter ses plugins créés en archives ZIP |
| ⚙️ **Paramètres** | Token Plex persisté, URL configurable, test de connexion |

---

## Installation

### Télécharger

Télécharge la dernière release depuis [Releases](../../releases) :

- **`PlexMetaForge_Setup.exe`** — Installateur NSIS (recommandé)
- **`PlexMetaForge_Portable.exe`** — Exécutable portable, sans installation

### Prérequis

- Windows 10 / 11 (64-bit)
- Plex Media Server installé localement
- [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (inclus dans Windows 11, installé automatiquement par le setup)

---

## Configuration

### 1. Token Plex

Ouvre l'app → ⚙ **Paramètres** → colle ton `X-Plex-Token`.

**Comment trouver ton token :**

```powershell
(Get-Content "$env:LOCALAPPDATA\Plex Media Server\Preferences.xml") -match 'PlexOnlineToken="([^"]+)"'
```

Ou via Plex Web : ouvre un film → `⋮` → **Voir info XML** → copie `X-Plex-Token=XXXXX` dans l'URL.

### 2. Test de connexion

Clic sur **🔌 Tester la connexion Plex** dans Paramètres → doit afficher `✓ Connecté`.

---

## Templates de plugins

| Template | Type Plex | Source | Clé API |
|----------|-----------|--------|---------|
| 📄 Vierge | `Agent.Movies` | — | Aucune |
| 🎬 Films | `Agent.Movies` | TMDB | [TMDB](https://www.themoviedb.org/settings/api) |
| 📺 Séries TV | `Agent.TV_Shows` | TMDB + épisodes | [TMDB](https://www.themoviedb.org/settings/api) |
| 🎵 Musique | `Agent.Artist` + `Agent.Album` | Last.fm | [Last.fm](https://www.last.fm/api/account/create) |
| ⛩️ Anime/Manga | `Agent.TV_Shows` | AniList | **Aucune** |
| 🌐 Universel | Films + Séries + Anime + Musique | TMDB + AniList + Last.fm | [TMDB](https://www.themoviedb.org/settings/api) |

---

## Catalogue — Plugins disponibles

| Plugin | Catégorie | Étoiles |
|--------|-----------|---------|
| Hama | Métadonnées anime | ⭐ 2.1k |
| Sub-Zero | Sous-titres | ⭐ 1.4k |
| WebTools | Utilitaires | ⭐ 950 |
| IPTV | IPTV/Live TV | ⭐ 680 |
| Absolute Series Scanner | Scanner | ⭐ 850 |
| Fanart.tv | Artwork HD | ⭐ 420 |
| CinemaVision | Expérience cinéma | ⭐ 490 |
| Audnexus | Audiobooks | ⭐ 380 |
| … et 20 autres | | |

---

## Stack technique

- **Backend** : Tauri v2 + Rust (rusqlite, reqwest, walkdir, zip)
- **Frontend** : Next.js 15 + React 19 + TypeScript strict + Tailwind CSS 3
- **Bundle** : NSIS (installateur) + binaire portable
- **DB** : SQLite WAL avec `busy_timeout=5000ms`

---

## Sécurité

- Le token Plex est stocké dans `%APPDATA%\PlexMetaForge\settings.json` (hors du repo)
- Aucun secret n'est jamais commité dans le code
- Les plugins du catalogue proviennent exclusivement de dépôts GitHub open-source vérifiés
- Sauvegarde automatique avant toute modification/suppression de plugin

---

## Build depuis les sources

```bash
git clone https://github.com/Heiphaistos/PlexMetaForge.git
cd PlexMetaForge
npm install
npx tauri dev        # mode développement
npx tauri build      # build release → src-tauri/target/release/
node scripts/post-build.js  # → dist/PlexMetaForge_Setup.exe + dist/PlexMetaForge_Portable.exe
```

**Prérequis build :** Node.js 18+, Rust 1.70+, Visual Studio Build Tools (MSVC)

---

## Licence

MIT — voir [LICENSE](LICENSE)

---

*Généré avec PlexMetaForge — heiphaistos.org*
