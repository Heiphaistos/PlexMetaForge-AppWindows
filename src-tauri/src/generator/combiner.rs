use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectiveConfig {
    pub name: String,
    // Agents à inclure
    pub films: bool,
    pub series: bool,
    pub anime: bool,           // Injecte la détection anime dans films+séries
    pub music_artist: bool,
    pub music_album: bool,
    // Sources activées
    pub use_tmdb: bool,
    pub use_anilist: bool,     // Gratuit, toujours dispo
    pub use_lastfm: bool,
    // Clés API
    pub tmdb_key: Option<String>,
    pub lastfm_key: Option<String>,
}

impl SelectiveConfig {
    pub fn needs_tmdb(&self) -> bool {
        self.use_tmdb && (self.films || self.series)
    }
    pub fn needs_lastfm(&self) -> bool {
        self.use_lastfm && (self.music_artist || self.music_album)
    }
    pub fn needs_anilist(&self) -> bool {
        self.anime && self.use_anilist
    }
}

pub fn generate(cfg: &SelectiveConfig) -> String {
    let name = &cfg.name;
    let mut out = String::new();

    // ── Header ──────────────────────────────────────────────
    out.push_str(&format!(r##"# ================================================================
# {name} — Plugin Universel Personnalisé pour Plex
# Généré par PlexMetaForge — Builder Avancé
# ================================================================
# Agents inclus :
"##));
    if cfg.films        { out.push_str(&format!("#   ✓ Films (Agent.Movies){}\n", if cfg.anime { " + Anime Films" } else { "" })); }
    if cfg.series       { out.push_str(&format!("#   ✓ Séries TV (Agent.TV_Shows){}\n", if cfg.anime { " + Anime/Manga" } else { "" })); }
    if cfg.music_artist { out.push_str("#   ✓ Musique — Artistes (Agent.Artist)\n"); }
    if cfg.music_album  { out.push_str("#   ✓ Musique — Albums (Agent.Album)\n"); }
    out.push_str(&format!(r##"# Sources : {}
# ================================================================

import json as _json

AGENT_VERSION = "1.0.0"
"##,
        [
            if cfg.use_tmdb { "TMDB" } else { "" },
            if cfg.use_anilist && cfg.anime { "AniList (sans clé)" } else { "" },
            if cfg.use_lastfm { "Last.fm" } else { "" },
        ].iter().filter(|s| !s.is_empty()).cloned().collect::<Vec<_>>().join(" + ")
    ));

    // ── Constantes selon sources ────────────────────────────
    if cfg.needs_tmdb() {
        out.push_str(r##"
TMDB_BASE = "https://api.themoviedb.org/3"
TMDB_ORIG = "https://image.tmdb.org/t/p/original"
TMDB_W500 = "https://image.tmdb.org/t/p/w500"
"##);
    }
    if cfg.needs_anilist() {
        out.push_str(r##"
ANILIST_URL = "https://graphql.anilist.co"
ANILIST_QUERY = (
    "query ($search: String, $type: MediaType) {"
    "  Page(perPage: 5) {"
    "    media(search: $search, type: $type, sort: SEARCH_MATCH) {"
    "      id title { romaji english french native }"
    "      startDate { year } coverImage { extraLarge large }"
    "      episodes averageScore genres description(asHtml: false)"
    "      studios { nodes { name isAnimationStudio } }"
    "      characters(sort: ROLE, perPage: 12) {"
    "        edges { node { name { full } image { large } } role }"
    "      }"
    "      staff(sort: RELEVANCE, perPage: 5) {"
    "        edges { role node { name { full } } }"
    "      }"
    "    }"
    "  }"
    "}"
)
ANIME_KEYWORDS = [
    "anime","manga","ova","ona","manhwa","manhua",
    "naruto","one piece","bleach","dragon ball","attack on titan",
    "demon slayer","sword art online","spirited away","ghibli","pokemon",
    "digimon","saint seiya","evangelion","death note","fullmetal","my hero",
    "jujutsu","chainsaw","spy x","bocchi","frieren","vinland"
]
"##);
    }
    if cfg.needs_lastfm() {
        out.push_str(r##"
LASTFM_BASE = "https://ws.audioscrobbler.com/2.0"
"##);
    }

    // ── start() + ValidatePrefs() ──────────────────────────
    out.push_str(&format!(r##"
def Start():
    HTTP.CacheTime = CACHE_1HOUR
    Log.Info("[{name}] v%s démarré" % AGENT_VERSION)

def ValidatePrefs():
"##));
    let mut checks: Vec<String> = Vec::new();
    if cfg.needs_tmdb() {
        checks.push(format!(
            r##"    if not Prefs["tmdb_api_key"]:
        return MessageContainer("Clé TMDB manquante", "Configure ta clé TMDB dans Paramètres > Agents.")"##
        ));
    }
    if cfg.needs_lastfm() {
        checks.push(format!(
            r##"    if not Prefs["lastfm_api_key"]:
        return MessageContainer("Clé Last.fm manquante", "Configure ta clé Last.fm dans Paramètres > Agents.")"##
        ));
    }
    if checks.is_empty() {
        out.push_str("    pass\n");
    } else {
        for c in &checks { out.push_str(c); out.push('\n'); }
        out.push_str("    return MessageContainer(\"OK\", \"Agent prêt.\")\n");
    }

    // ── Helpers AniList ─────────────────────────────────────
    if cfg.needs_anilist() {
        out.push_str(r##"
def _anilist(query, variables):
    body = _json.dumps({"query": query, "variables": variables})
    resp = HTTP.Request(ANILIST_URL, method="POST", data=body,
                        headers={"Content-Type": "application/json"}, sleep=0.5)
    return _json.loads(resp.content)

def _ani_title(t, lang):
    l = lang[:2] if lang else "en"
    if l == "fr" and t.get("french"):  return t["french"]
    if l == "en" and t.get("english"): return t["english"]
    return t.get("english") or t.get("romaji") or t.get("native", "")

def _is_anime(title):
    tl = title.lower()
    return any(kw in tl for kw in ANIME_KEYWORDS)

def _apply_anilist(metadata, item, lang):
    metadata.title = _ani_title(item.get("title", {}), lang)
    desc = item.get("description", "") or ""
    metadata.summary = desc.replace("<br>","").replace("<i>","").replace("</i>","")
    sc = item.get("averageScore")
    if sc: metadata.rating = float(sc) / 10.0
    sd = item.get("startDate") or {}
    if sd.get("year"): metadata.year = sd["year"]
    metadata.genres.clear()
    for g in item.get("genres", []): metadata.genres.add(g)
    for studio in item.get("studios", {}).get("nodes", []):
        if studio.get("isAnimationStudio"):
            metadata.studio = studio["name"]; break
    metadata.roles.clear()
    for edge in item.get("characters", {}).get("edges", [])[:12]:
        node = edge.get("node", {})
        r = metadata.roles.new()
        r.name = node.get("name", {}).get("full", "")
        r.role = edge.get("role", "")
        img = node.get("image", {}).get("large", "")
        if img: r.photo = img
    metadata.directors.clear()
    for edge in item.get("staff", {}).get("edges", []):
        if edge.get("role") in ("Director", "Series Director"):
            metadata.directors.new().name = edge["node"]["name"]["full"]
    metadata.posters.validate_keys([])
    cover = item.get("coverImage", {})
    for k in ("extraLarge", "large"):
        url = cover.get(k, "")
        if url:
            metadata.posters[url] = Proxy.Preview(HTTP.Request(url, sleep=0).content, sort_order=1)
            break
"##);
    }

    // ── TMDB helpers ────────────────────────────────────────
    if cfg.needs_tmdb() {
        out.push_str(r##"
def _tmdb_get(path, lang, extra=""):
    key = Prefs["tmdb_api_key"]
    url = "%s%s?api_key=%s&language=%s%s" % (TMDB_BASE, path, key, lang, extra)
    return JSON.ObjectFromURL(url, sleep=0.5)

def _tmdb_posters(metadata, images):
    metadata.posters.validate_keys([])
    for p in images.get("posters", [])[:4]:
        pu = TMDB_ORIG + p["file_path"]
        metadata.posters[pu] = Proxy.Preview(HTTP.Request(TMDB_W500 + p["file_path"], sleep=0).content, sort_order=1)

def _tmdb_backdrops(metadata, images):
    metadata.art.validate_keys([])
    for a in images.get("backdrops", [])[:3]:
        au = TMDB_ORIG + a["file_path"]
        metadata.art[au] = Proxy.Preview(HTTP.Request(TMDB_W500 + a["file_path"], sleep=0).content, sort_order=1)

def _tmdb_cast(metadata, credits):
    metadata.roles.clear()
    for actor in credits.get("cast", [])[:15]:
        r = metadata.roles.new(); r.name = actor["name"]
        r.role = actor.get("character", "")
        if actor.get("profile_path"): r.photo = TMDB_ORIG + actor["profile_path"]

def _tmdb_crew(metadata, credits):
    metadata.directors.clear(); metadata.writers.clear()
    for c in credits.get("crew", []):
        if c["job"] == "Director": metadata.directors.new().name = c["name"]
        elif c["job"] in ("Screenplay","Writer","Story"): metadata.writers.new().name = c["name"]
"##);
    }

    // ── Agent Films ─────────────────────────────────────────
    if cfg.films {
        out.push_str(&format!(r##"
class {name}Movies(Agent.Movies):
    name             = "{name} — Films"
    languages        = [Locale.Language.French, Locale.Language.English,
                        Locale.Language.Japanese, Locale.Language.NoLanguage]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        title = media.name
        year  = getattr(media, "year", None)
"##));
        if cfg.needs_anilist() {
            out.push_str(r##"        if _is_anime(title):
            try:
                resp = _anilist(ANILIST_QUERY, {"search": title, "type": "ANIME"})
                for i, item in enumerate(resp.get("data",{}).get("Page",{}).get("media",[])[:5]):
                    ttl = _ani_title(item.get("title",{}), lang)
                    yr  = (item.get("startDate") or {}).get("year")
                    results.Append(MetadataSearchResult(id="ani_%s"%item["id"], name=ttl, year=yr, score=max(60,95-i*8), lang=lang))
                if results: return
            except Exception as e: Log.Warn("[Movies] AniList: %s"%e)
"##);
        }
        if cfg.needs_tmdb() {
            out.push_str(r##"        try:
            key = Prefs["tmdb_api_key"]
            qs = "api_key=%s&query=%s&language=%s" % (key, String.Quote(title), lang)
            if year: qs += "&primary_release_year=%s" % year
            data = JSON.ObjectFromURL("%s/search/movie?%s" % (TMDB_BASE, qs), sleep=0.5)
            for i, r in enumerate(data.get("results",[])[:5]):
                ttl = r.get("title",""); yr = r.get("release_date","")[:4]
                score = 100 if ttl.lower()==title.lower() else max(60,90-i*8)
                results.Append(MetadataSearchResult(id=str(r["id"]), name=ttl, year=int(yr) if yr.isdigit() else None, score=score, lang=lang))
        except Exception as e: Log.Error("[Movies] TMDB search: %s"%e)
"##);
        }
        out.push_str(&format!(r##"
    def update(self, metadata, media, lang, force):
        mid = metadata.id
"##));
        if cfg.needs_anilist() {
            out.push_str(r##"        if mid.startswith("ani_"):
            try:
                ani_id = int(mid.replace("ani_",""))
                resp = _anilist(ANILIST_QUERY, {"search": metadata.title, "type": "ANIME"})
                items = resp.get("data",{}).get("Page",{}).get("media",[])
                target = next((x for x in items if x["id"]==ani_id), items[0] if items else None)
                if target: _apply_anilist(metadata, target, lang)
            except Exception as e: Log.Error("[Movies] AniList update: %s"%e)
            return
"##);
        }
        if cfg.needs_tmdb() {
            out.push_str(r##"        try:
            d = _tmdb_get("/movie/%s" % mid, lang, "&append_to_response=credits,images")
            metadata.title = d.get("title",""); metadata.original_title = d.get("original_title","")
            metadata.summary = d.get("overview",""); metadata.rating = float(d.get("vote_average",0))
            metadata.tagline = d.get("tagline","")
            rt = int(d.get("runtime") or 0)
            if rt: metadata.duration = rt * 60000
            if d.get("production_companies"): metadata.studio = d["production_companies"][0].get("name","")
            if d.get("release_date"):
                try:
                    from datetime import datetime
                    dt = datetime.strptime(d["release_date"],"%Y-%m-%d")
                    metadata.originally_available_at = dt.date(); metadata.year = dt.year
                except: pass
            metadata.genres.clear()
            for g in d.get("genres",[]): metadata.genres.add(g["name"])
            _tmdb_cast(metadata, d.get("credits",{}))
            _tmdb_crew(metadata, d.get("credits",{}))
            _tmdb_posters(metadata, d.get("images",{}))
            _tmdb_backdrops(metadata, d.get("images",{}))
            Log.Info("[Movies] %s (%s)" % (metadata.title, metadata.year))
        except Exception as e: Log.Error("[Movies] TMDB update: %s"%e)
"##);
        }
        out.push_str("\n");
    }

    // ── Agent Séries ────────────────────────────────────────
    if cfg.series {
        out.push_str(&format!(r##"
class {name}Shows(Agent.TV_Shows):
    name             = "{name} — Séries & Anime"
    languages        = [Locale.Language.French, Locale.Language.English,
                        Locale.Language.Japanese, Locale.Language.NoLanguage]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
        title = media.show
"##));
        if cfg.needs_anilist() {
            out.push_str(r##"        if _is_anime(title):
            for mtype in ("ANIME","MANGA"):
                try:
                    resp = _anilist(ANILIST_QUERY,{"search":title,"type":mtype})
                    for i,item in enumerate(resp.get("data",{}).get("Page",{}).get("media",[])[:5]):
                        ttl=_ani_title(item.get("title",{}),lang); yr=(item.get("startDate")or{}).get("year")
                        results.Append(MetadataSearchResult(id="ani_%s_%s"%(mtype.lower(),item["id"]),name=ttl,year=yr,score=max(60,95-i*8),lang=lang))
                except Exception as e: Log.Warn("[Shows] AniList(%s): %s"%(mtype,e))
            return
"##);
        }
        if cfg.needs_tmdb() {
            out.push_str(r##"        try:
            key = Prefs["tmdb_api_key"]
            data = JSON.ObjectFromURL("%s/search/tv?api_key=%s&query=%s&language=%s"%(TMDB_BASE,key,String.Quote(title),lang),sleep=0.5)
            for i,r in enumerate(data.get("results",[])[:5]):
                ttl=r.get("name",""); yr=r.get("first_air_date","")[:4]
                score=100 if ttl.lower()==title.lower() else max(60,90-i*8)
                results.Append(MetadataSearchResult(id=str(r["id"]),name=ttl,year=int(yr) if yr.isdigit() else None,score=score,lang=lang))
        except Exception as e: Log.Error("[Shows] TMDB search: %s"%e)
"##);
        }
        out.push_str(&format!(r##"
    def update(self, metadata, media, lang, force):
        mid = metadata.id
"##));
        if cfg.needs_anilist() {
            out.push_str(r##"        if mid.startswith("ani_"):
            ani_id = int(mid.split("_")[-1])
            try:
                resp = _anilist(ANILIST_QUERY,{"search":metadata.title,"type":"ANIME"})
                items = resp.get("data",{}).get("Page",{}).get("media",[])
                target = next((x for x in items if x["id"]==ani_id), items[0] if items else None)
                if target: _apply_anilist(metadata, target, lang)
                Log.Info("[Shows] Anime: %s" % metadata.title)
            except Exception as e: Log.Error("[Shows] AniList update: %s"%e)
            return
"##);
        }
        if cfg.needs_tmdb() {
            out.push_str(r##"        try:
            d = _tmdb_get("/tv/%s"%mid, lang, "&append_to_response=credits,images")
            metadata.title=d.get("name",""); metadata.summary=d.get("overview",""); metadata.rating=float(d.get("vote_average",0))
            if d.get("first_air_date"):
                try:
                    from datetime import datetime
                    dt=datetime.strptime(d["first_air_date"],"%Y-%m-%d")
                    metadata.originally_available_at=dt.date(); metadata.year=dt.year
                except: pass
            metadata.genres.clear()
            for g in d.get("genres",[]): metadata.genres.add(g["name"])
            if d.get("networks"): metadata.studio=d["networks"][0].get("name","")
            _tmdb_cast(metadata, d.get("credits",{}))
            metadata.directors.clear()
            for c in d.get("created_by",[]): metadata.directors.new().name=c["name"]
            _tmdb_posters(metadata, d.get("images",{}))
            _tmdb_backdrops(metadata, d.get("images",{}))
            Log.Info("[Shows] %s" % metadata.title)
        except Exception as e: Log.Error("[Shows] TMDB update: %s"%e)
"##);
        }
        out.push_str("\n");
    }

    // ── Agent Musique Artiste ───────────────────────────────
    if cfg.music_artist {
        out.push_str(&format!(r##"
class {name}Artist(Agent.Artist):
    name             = "{name} — Artiste"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
"##));
        if cfg.needs_lastfm() {
            out.push_str(r##"        key = Prefs.get("lastfm_api_key","")
        if not key:
            results.Append(MetadataSearchResult(id=media.artist,name=media.artist,score=80,lang=lang)); return
        try:
            data=JSON.ObjectFromURL("%s/?method=artist.search&artist=%s&api_key=%s&format=json"%(LASTFM_BASE,String.Quote(media.artist),key),sleep=0.5)
            for i,a in enumerate(data.get("results",{}).get("artistmatches",{}).get("artist",[])[:5]):
                results.Append(MetadataSearchResult(id=a.get("mbid",a["name"]),name=a["name"],score=max(60,100-i*10),lang=lang))
        except Exception as e: Log.Error("[Artist] search: %s"%e)
"##);
        } else {
            out.push_str(r##"        results.Append(MetadataSearchResult(id=media.artist,name=media.artist,score=80,lang=lang))
"##);
        }
        out.push_str(r##"
    def update(self, metadata, media, lang, force):
"##);
        if cfg.needs_lastfm() {
            out.push_str(r##"        key = Prefs.get("lastfm_api_key","")
        if not key: return
        try:
            data=JSON.ObjectFromURL("%s/?method=artist.getinfo&artist=%s&api_key=%s&lang=%s&format=json"%(LASTFM_BASE,String.Quote(media.artist),key,lang[:2]),sleep=0.5)
            info=data.get("artist",{})
            metadata.title=info.get("name",media.artist)
            bio=(info.get("bio",{}).get("summary","") or "")
            metadata.summary=bio.split("<a href")[0].strip()
            metadata.genres.clear()
            for tag in info.get("tags",{}).get("tag",[])[:5]: metadata.genres.add(tag["name"])
            metadata.similar.clear()
            for sim in info.get("similar",{}).get("artist",[])[:8]: metadata.similar.add(sim["name"])
            for img in reversed(info.get("image",[])):
                url=img.get("#text","")
                if url:
                    metadata.posters[url]=Proxy.Preview(HTTP.Request(url,sleep=0).content,sort_order=1); break
        except Exception as e: Log.Error("[Artist] update: %s"%e)
"##);
        } else {
            out.push_str("        metadata.title = media.artist\n");
        }
        out.push_str("\n");
    }

    // ── Agent Musique Album ──────────────────────────────────
    if cfg.music_album {
        out.push_str(&format!(r##"
class {name}Album(Agent.Album):
    name             = "{name} — Album"
    languages        = [Locale.Language.French, Locale.Language.English]
    primary_provider = True
    accepts_from     = ["com.plexapp.agents.localmedia"]

    def search(self, results, media, lang, manual):
"##));
        if cfg.needs_lastfm() {
            out.push_str(r##"        key = Prefs.get("lastfm_api_key","")
        if not key:
            results.Append(MetadataSearchResult(id=media.album,name=media.album,score=80,lang=lang)); return
        try:
            data=JSON.ObjectFromURL("%s/?method=album.search&album=%s&api_key=%s&format=json"%(LASTFM_BASE,String.Quote(media.album),key),sleep=0.5)
            for i,a in enumerate(data.get("results",{}).get("albummatches",{}).get("album",[])[:5]):
                score=100 if a["name"].lower()==media.album.lower() else max(60,90-i*10)
                results.Append(MetadataSearchResult(id=a.get("mbid",a["name"]),name=a["name"],score=score,lang=lang))
        except Exception as e: Log.Error("[Album] search: %s"%e)
"##);
        } else {
            out.push_str(r##"        results.Append(MetadataSearchResult(id=media.album,name=media.album,score=80,lang=lang))
"##);
        }
        out.push_str(r##"
    def update(self, metadata, media, lang, force):
"##);
        if cfg.needs_lastfm() {
            out.push_str(r##"        key = Prefs.get("lastfm_api_key","")
        if not key: return
        try:
            data=JSON.ObjectFromURL("%s/?method=album.getinfo&album=%s&artist=%s&api_key=%s&lang=%s&format=json"%(LASTFM_BASE,String.Quote(media.album),String.Quote(media.artist),key,lang[:2]),sleep=0.5)
            info=data.get("album",{})
            metadata.title=info.get("name",media.album)
            wiki=(info.get("wiki",{}).get("summary","") or "")
            metadata.summary=wiki.split("<a href")[0].strip()
            metadata.genres.clear()
            for tag in info.get("tags",{}).get("tag",[])[:5]: metadata.genres.add(tag["name"])
            for img in reversed(info.get("image",[])):
                url=img.get("#text","")
                if url:
                    metadata.posters[url]=Proxy.Preview(HTTP.Request(url,sleep=0).content,sort_order=1); break
        except Exception as e: Log.Error("[Album] update: %s"%e)
"##);
        } else {
            out.push_str("        metadata.title = media.album\n");
        }
    }

    out
}

pub fn prefs_json(cfg: &SelectiveConfig) -> String {
    let mut prefs = Vec::new();
    if cfg.needs_tmdb() {
        prefs.push(r#"    {"id":"tmdb_api_key","type":"text","label":"Clé API TMDB","default":"","secure":false}"#);
    }
    if cfg.needs_lastfm() {
        prefs.push(r#"    {"id":"lastfm_api_key","type":"text","label":"Clé API Last.fm","default":"","secure":false}"#);
    }
    format!("{{\n  \"prefs\": [\n{}\n  ]\n}}", prefs.join(",\n"))
}
