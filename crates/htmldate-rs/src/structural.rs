//! Tier 2: Extract dates from structural HTML elements.
//!
//! Checks `<time>`, `<abbr>`, and elements with date-related class/id attributes.

use chrono::NaiveDate;
use scraper::{Html, Selector};

use crate::settings::DateOptions;
use crate::validators::try_parse_and_validate;

/// CSS selectors for date-bearing structural elements, ordered by specificity.
static STRUCTURAL_SELECTORS: &[&str] = &[
    // <time datetime="...">
    "time[datetime]",
    // <abbr> with date-related classes
    r#"abbr[class="published"]"#,
    r#"abbr[class="date"]"#,
    // itemprop date elements
    "[itemprop='datePublished']",
    "[itemprop='dateModified']",
    "[itemprop='dateCreated']",
];

/// Class/id substrings that indicate date-bearing elements.
static DATE_CLASSES: &[&str] = &[
    "entry-date",
    "post-date",
    "published",
    "article-date",
    "article_date",
    "post-meta",
    "posted-on",
    "publication-date",
    "publish-date",
    "publishing-date",
    "date-published",
    "dateline",
    "byline",
    "time",
    "fecha",   // Spanish
    "datum",   // German/Dutch
    "tanggal", // Indonesian
];

/// Extract date from `<time datetime="">` and similar structural elements.
fn extract_time_elements(document: &Html, options: &DateOptions) -> Option<NaiveDate> {
    for sel_str in STRUCTURAL_SELECTORS {
        if let Ok(selector) = Selector::parse(sel_str) {
            for element in document.select(&selector) {
                // Try datetime attribute first
                if let Some(dt) = element.value().attr("datetime") {
                    if let Some(date) = try_parse_and_validate(dt, options) {
                        return Some(date);
                    }
                }
                // Then title attribute (used by <abbr>)
                if let Some(title) = element.value().attr("title") {
                    if let Some(date) = try_parse_and_validate(title, options) {
                        return Some(date);
                    }
                }
                // Then text content
                let text: String = element.text().collect();
                if let Some(date) = try_parse_and_validate(&text, options) {
                    return Some(date);
                }
            }
        }
    }
    None
}

/// Extract date from elements with date-related class or id attributes.
fn extract_from_date_classes(document: &Html, options: &DateOptions) -> Option<NaiveDate> {
    for class_name in DATE_CLASSES {
        // Try class contains
        let css = format!(r#"[class*="{class_name}"], [id*="{class_name}"]"#);
        if let Ok(selector) = Selector::parse(&css) {
            for element in document.select(&selector) {
                let text: String = element.text().collect();
                let trimmed = text.trim();
                if !trimmed.is_empty() && trimmed.len() < 100 {
                    if let Some(date) = try_parse_and_validate(trimmed, options) {
                        return Some(date);
                    }
                }
            }
        }
    }
    None
}

/// Main entry point for Tier 2 structural extraction.
pub fn extract_from_structure(document: &Html, options: &DateOptions) -> Option<NaiveDate> {
    if let Some(date) = extract_time_elements(document, options) {
        return Some(date);
    }
    extract_from_date_classes(document, options)
}
