use std::path::PathBuf;
use crate::error::Result;
use super::MetadataPayload;

pub fn write_nfo(media_path: &PathBuf, payload: &MetadataPayload) -> Result<()> {
    let folder_name = media_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| payload.title.clone());

    let nfo_path = media_path.join(format!("{}.nfo", folder_name));
    std::fs::write(&nfo_path, build_xml(payload))?;
    Ok(())
}

fn build_xml(p: &MetadataPayload) -> String {
    let mut lines = vec![
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.to_string(),
        "<movie>".to_string(),
        format!("  <title>{}</title>", xml_escape(&p.title)),
    ];

    if let Some(year) = p.year {
        lines.push(format!("  <year>{}</year>", year));
    }
    if let Some(ref plot) = p.plot {
        if !plot.is_empty() {
            lines.push(format!("  <plot>{}</plot>", xml_escape(plot)));
        }
    }
    if let Some(ref tagline) = p.tagline {
        if !tagline.is_empty() {
            lines.push(format!("  <tagline>{}</tagline>", xml_escape(tagline)));
        }
    }
    if let Some(ref studio) = p.studio {
        if !studio.is_empty() {
            lines.push(format!("  <studio>{}</studio>", xml_escape(studio)));
        }
    }
    if let Some(rating) = p.rating {
        lines.push(format!("  <rating>{}</rating>", rating));
    }
    if let Some(ref url) = p.poster_url {
        if !url.is_empty() {
            lines.push(format!(
                r#"  <thumb aspect="poster">{}</thumb>"#,
                xml_escape(url)
            ));
        }
    }
    if let Some(ref url) = p.fanart_url {
        if !url.is_empty() {
            lines.push(format!(
                "  <fanart><thumb>{}</thumb></fanart>",
                xml_escape(url)
            ));
        }
    }
    if let Some(ref id) = p.tmdb_id {
        if !id.is_empty() {
            lines.push(format!(
                r#"  <uniqueid type="tmdb" default="true">{}</uniqueid>"#,
                xml_escape(id)
            ));
        }
    }
    if let Some(ref id) = p.imdb_id {
        if !id.is_empty() {
            lines.push(format!(
                r#"  <uniqueid type="imdb">{}</uniqueid>"#,
                xml_escape(id)
            ));
        }
    }

    lines.push("</movie>".to_string());
    lines.join("\n")
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
