use rusqlite::{Connection, OpenFlags, params, types::Value};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{PlexMetaForgeError, Result};

// ─── Structs ──────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaItem {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub summary: Option<String>,
    pub thumb: Option<String>,
    pub media_type: Option<String>,
    pub section_id: Option<i64>,
    pub duration: Option<i64>,
    pub rating: Option<f64>,
    pub tagline: Option<String>,
    pub studio: Option<String>,
    pub originally_available_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlexSection {
    pub id: i64,
    pub name: String,
    pub section_type: String,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseStats {
    pub total_items: i64,
    pub movies: i64,
    pub shows: i64,
    pub episodes: i64,
    pub artists: i64,
    pub albums: i64,
    pub tracks: i64,
    pub sections: i64,
    pub items_with_thumb: i64,
    pub items_without_summary: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchUpdateResult {
    pub updated: usize,
    pub errors: Vec<String>,
}

// ─── Helpers types ────────────────────────────────────────────

/// Lit une valeur rusqlite (TEXT ou INTEGER) et la convertit en String
fn val_to_string(v: Value) -> String {
    match v {
        Value::Text(s) => s,
        Value::Integer(i) => section_type_from_int(i),
        Value::Real(f) => f.to_string(),
        _ => String::new(),
    }
}

/// Plex stocke section_type comme entier dans certaines versions
fn section_type_from_int(i: i64) -> String {
    match i {
        1 => "movie".to_string(),
        2 => "show".to_string(),
        3 => "show".to_string(),
        8 => "artist".to_string(),
        13 => "photo".to_string(),
        _ => format!("type_{}", i),
    }
}

/// Plex stocke originally_available_at comme date julienne (REAL) ou texte
fn val_to_date_string(v: Value) -> Option<String> {
    match v {
        Value::Null => None,
        Value::Text(s) if s.is_empty() => None,
        Value::Text(s) => Some(s),
        Value::Integer(i) if i == 0 => None,
        Value::Integer(i) => {
            // Julian day number → approximation YYYY-MM-DD
            julian_to_date(i as f64)
        }
        Value::Real(f) if f == 0.0 => None,
        Value::Real(f) => julian_to_date(f),
        _ => None,
    }
}

/// Convertit un numéro de jour julien en "YYYY-MM-DD"
fn julian_to_date(jd: f64) -> Option<String> {
    // Julian Day Number → Gregorian calendar
    let jd = jd as i64;
    if jd < 1000000 {
        // Probably a Unix timestamp in seconds
        let secs = jd;
        let days_since_epoch = secs / 86400;
        let epoch_jd: i64 = 2440588; // JD for 1970-01-01
        return julian_jd_to_date(epoch_jd + days_since_epoch);
    }
    julian_jd_to_date(jd)
}

fn julian_jd_to_date(jd: i64) -> Option<String> {
    // Algorithm to convert JD to calendar date
    let l = jd + 68569;
    let n = 4 * l / 146097;
    let l = l - (146097 * n + 3) / 4;
    let i = 4000 * (l + 1) / 1461001;
    let l = l - 1461 * i / 4 + 31;
    let j = 80 * l / 2447;
    let day = l - 2447 * j / 80;
    let l = j / 11;
    let month = j + 2 - 12 * l;
    let year = 100 * (n - 49) + i + l;
    if year < 1800 || year > 2100 {
        return None;
    }
    Some(format!("{:04}-{:02}-{:02}", year, month, day))
}

// ─── Database ─────────────────────────────────────────────────

pub struct PlexDatabase {
    conn: Connection,
}

impl PlexDatabase {
    pub fn open(db_path: &PathBuf) -> Result<Self> {
        if !db_path.exists() {
            return Err(PlexMetaForgeError::PlexApi(format!(
                "Base de données introuvable : {}",
                db_path.display()
            )));
        }

        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;

        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;",
        )?;

        Ok(Self { conn })
    }

    // ─── Stats ──────────────────────────────────────────────

    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let count = |q: &str| -> Result<i64> {
            self.conn.query_row(q, [], |r| r.get(0)).map_err(PlexMetaForgeError::Sqlite)
        };

        Ok(DatabaseStats {
            total_items:           count("SELECT COUNT(*) FROM metadata_items")?,
            movies:                count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 1")?,
            shows:                 count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 2")?,
            episodes:              count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 4")?,
            artists:               count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 8")?,
            albums:                count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 9")?,
            tracks:                count("SELECT COUNT(*) FROM metadata_items WHERE metadata_type = 10")?,
            sections:              count("SELECT COUNT(*) FROM library_sections")?,
            items_with_thumb:      count("SELECT COUNT(*) FROM metadata_items WHERE user_thumb_url IS NOT NULL AND user_thumb_url != ''")?,
            items_without_summary: count("SELECT COUNT(*) FROM metadata_items WHERE (summary IS NULL OR summary = '') AND metadata_type IN (1,2,4)")?,
        })
    }

    // ─── Sections ───────────────────────────────────────────

    pub fn get_sections(&self) -> Result<Vec<PlexSection>> {
        // section_type peut être TEXT ('movie') ou INTEGER (1) selon la version Plex
        // On lit comme Value pour gérer les deux cas
        let sql = "SELECT ls.id, ls.name, ls.section_type, lsp.root_path
                   FROM library_sections ls
                   LEFT JOIN library_section_locations lsp ON lsp.library_section_id = ls.id
                   GROUP BY ls.id ORDER BY ls.name";
        let sql_fallback = "SELECT id, name, section_type, NULL FROM library_sections ORDER BY name";

        let mut stmt = self.conn.prepare(sql)
            .or_else(|_| self.conn.prepare(sql_fallback))?;

        let rows = stmt.query_map([], |row| {
            let section_type_val: Value = row.get(2)?;
            Ok(PlexSection {
                id:           row.get(0)?,
                name:         row.get(1)?,
                section_type: val_to_string(section_type_val),
                location:     row.get(3)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    // ─── Search ─────────────────────────────────────────────

    pub fn search_metadata_items(&self, query: &str) -> Result<Vec<MediaItem>> {
        let pattern = format!("%{}%", query);
        // originally_available_at : stocké en REAL (Julian date) ou TEXT selon la version
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.title LIKE ?1
               AND mi.metadata_type IN (1, 2, 4, 8, 9, 10)
             ORDER BY mi.title
             LIMIT 100"
        )?;

        let items = stmt.query_map([&pattern], |row| {
            let date_val: Value = row.get(11)?;
            Ok(MediaItem {
                id:                      row.get(0)?,
                title:                   row.get(1)?,
                year:                    row.get(2)?,
                summary:                 row.get(3)?,
                thumb:                   row.get(4)?,
                media_type:              row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:              row.get(6)?,
                duration:                row.get(7)?,
                rating:                  row.get(8)?,
                tagline:                 row.get(9)?,
                studio:                  row.get(10)?,
                originally_available_at: val_to_date_string(date_val),
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(items)
    }

    pub fn get_items_by_section(&self, section_id: i64, limit: i64) -> Result<Vec<MediaItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.library_section_id = ?1
               AND mi.metadata_type IN (1, 2, 8, 9)
             ORDER BY mi.title
             LIMIT ?2"
        )?;

        let items = stmt.query_map(params![section_id, limit], |row| {
            let date_val: Value = row.get(11)?;
            Ok(MediaItem {
                id:                      row.get(0)?,
                title:                   row.get(1)?,
                year:                    row.get(2)?,
                summary:                 row.get(3)?,
                thumb:                   row.get(4)?,
                media_type:              row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:              row.get(6)?,
                duration:                row.get(7)?,
                rating:                  row.get(8)?,
                tagline:                 row.get(9)?,
                studio:                  row.get(10)?,
                originally_available_at: val_to_date_string(date_val),
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(items)
    }

    pub fn get_media_item_by_title(&self, title: &str) -> Result<Option<MediaItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.title = ?1
             LIMIT 1"
        )?;

        let result = stmt.query_row([title], |row| {
            let date_val: Value = row.get(11)?;
            Ok(MediaItem {
                id:                      row.get(0)?,
                title:                   row.get(1)?,
                year:                    row.get(2)?,
                summary:                 row.get(3)?,
                thumb:                   row.get(4)?,
                media_type:              row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:              row.get(6)?,
                duration:                row.get(7)?,
                rating:                  row.get(8)?,
                tagline:                 row.get(9)?,
                studio:                  row.get(10)?,
                originally_available_at: val_to_date_string(date_val),
            })
        });

        match result {
            Ok(item) => Ok(Some(item)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(PlexMetaForgeError::Sqlite(e)),
        }
    }

    pub fn get_incomplete_items(&self, limit: i64) -> Result<Vec<MediaItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT mi.id, mi.title, mi.year, mi.summary, mi.user_thumb_url,
                    mi.metadata_type, mi.library_section_id, mi.duration,
                    mi.rating, mi.tagline, mi.studio, mi.originally_available_at
             FROM metadata_items mi
             WHERE mi.metadata_type IN (1, 2)
               AND (mi.summary IS NULL OR mi.summary = ''
                    OR mi.user_thumb_url IS NULL OR mi.user_thumb_url = '')
             ORDER BY mi.title
             LIMIT ?1"
        )?;

        let items = stmt.query_map([limit], |row| {
            let date_val: Value = row.get(11)?;
            Ok(MediaItem {
                id:                      row.get(0)?,
                title:                   row.get(1)?,
                year:                    row.get(2)?,
                summary:                 row.get(3)?,
                thumb:                   row.get(4)?,
                media_type:              row.get::<_, Option<i64>>(5)?.map(media_type_label),
                section_id:              row.get(6)?,
                duration:                row.get(7)?,
                rating:                  row.get(8)?,
                tagline:                 row.get(9)?,
                studio:                  row.get(10)?,
                originally_available_at: val_to_date_string(date_val),
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(items)
    }

    // ─── Write ──────────────────────────────────────────────

    pub fn update_metadata_full(
        &self,
        id: i64,
        title: &str,
        year: Option<i32>,
        summary: &str,
        tagline: Option<&str>,
        studio: Option<&str>,
        rating: Option<f64>,
    ) -> Result<usize> {
        let n = self.conn.execute(
            "UPDATE metadata_items SET title=?1, year=?2, summary=?3, tagline=?4, studio=?5, rating=?6 WHERE id=?7",
            params![title, year, summary, tagline, studio, rating, id],
        )?;
        Ok(n)
    }

    pub fn batch_clear_locks(&self) -> Result<BatchUpdateResult> {
        let n = self.conn.execute(
            "UPDATE metadata_items SET refreshed_at = NULL WHERE refreshed_at IS NOT NULL",
            [],
        )?;
        Ok(BatchUpdateResult { updated: n, errors: vec![] })
    }

    #[allow(dead_code)]
    pub fn update_thumb(&self, id: i64, thumb_url: &str) -> Result<usize> {
        let n = self.conn.execute(
            "UPDATE metadata_items SET user_thumb_url = ?1 WHERE id = ?2",
            params![thumb_url, id],
        )?;
        Ok(n)
    }
}

fn media_type_label(t: i64) -> String {
    match t {
        1  => "Film".to_string(),
        2  => "Série".to_string(),
        3  => "Saison".to_string(),
        4  => "Épisode".to_string(),
        8  => "Artiste".to_string(),
        9  => "Album".to_string(),
        10 => "Piste".to_string(),
        13 => "Photo".to_string(),
        _  => format!("Type {}", t),
    }
}
