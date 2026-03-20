//! Metadata extraction from web pages.
//!
//! Mirrors trafilatura's `metadata.py` + `json_metadata.py`:
//! - Title (from XPath patterns, <title>, OG, JSON-LD)
//! - Author (from meta, JSON-LD, byline patterns)
//! - Date (via htmldate-rs)
//! - URL (canonical, OG)
//! - Sitename, description, categories, tags, license, image

pub mod author;
pub mod json_ld;
pub mod opengraph;
pub mod title;

use serde::{Deserialize, Serialize};

/// Extracted document metadata.
/// Mirrors trafilatura settings.py Document class (20 slots).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub url: Option<String>,
    pub hostname: Option<String>,
    pub description: Option<String>,
    pub sitename: Option<String>,
    pub date: Option<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub fingerprint: Option<String>,
    pub id: Option<String>,
    pub license: Option<String>,
    pub language: Option<String>,
    pub image: Option<String>,
    pub pagetype: Option<String>,
}

/// Extract all metadata from an HTML document.
pub fn extract_metadata(
    _html: &str,
    _url: Option<&str>,
    _date_params: &htmldate_rs::DateOptions,
    _fast: bool,
    _author_blacklist: &std::collections::HashSet<String>,
) -> DocumentMetadata {
    // TODO: port from trafilatura metadata.py
    // 1. examine_meta() - scan meta tags
    // 2. extract_meta_json() - JSON-LD
    // 3. extract_opengraph() - OG tags
    // 4. extract_title(), extract_author()
    // 5. htmldate_rs::find_date()
    // 6. extract_url(), extract_sitename()
    // 7. extract_catstags(), extract_license()
    todo!("extract_metadata")
}
