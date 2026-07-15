use crate::config::PlexPaths;
use crate::database::PlexDatabase;
use crate::error::Result;
use super::MetadataPayload;

pub fn update_metadata(paths: &PlexPaths, payload: &MetadataPayload) -> Result<usize> {
    let db = PlexDatabase::open(&paths.database_path)?;

    let item = db.get_media_item_by_title(&payload.title)?;
    let updated = match item {
        Some(item) => db.update_metadata_full(
            item.id,
            &payload.title,
            payload.year,
            payload.plot.as_deref().unwrap_or(""),
            payload.tagline.as_deref(),
            payload.studio.as_deref(),
            payload.rating,
        )?,
        None => 0,
    };

    Ok(updated)
}
