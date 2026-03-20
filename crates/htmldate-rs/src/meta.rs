//! Tier 1: Extract dates from HTML header metadata.
//!
//! Checks JSON-LD, Open Graph, and standard meta tags.

use chrono::NaiveDate;
use scraper::{Html, Selector};

use crate::settings::DateOptions;
use crate::validators::try_parse_and_validate;

/// Extract date from JSON-LD script blocks.
fn extract_json_ld(document: &Html, options: &DateOptions) -> Option<NaiveDate> {
    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).ok()?;

    for element in document.select(&selector) {
        let text = element.text().collect::<String>();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            // Try datePublished first (original), then dateModified
            let fields = if options.original_date {
                &["datePublished", "dateCreated", "dateModified", "uploadDate"]
            } else {
                &["dateModified", "datePublished", "dateCreated", "uploadDate"]
            };

            for field in fields {
                if let Some(date_str) = json.get(field).and_then(|v| v.as_str()) {
                    if let Some(date) = try_parse_and_validate(date_str, options) {
                        return Some(date);
                    }
                }
                // Handle nested @graph arrays
                if let Some(graph) = json.get("@graph").and_then(|v| v.as_array()) {
                    for item in graph {
                        if let Some(date_str) = item.get(field).and_then(|v| v.as_str()) {
                            if let Some(date) = try_parse_and_validate(date_str, options) {
                                return Some(date);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Extract date from Open Graph and standard meta tags.
fn extract_meta_tags(document: &Html, options: &DateOptions) -> Option<NaiveDate> {
    // Ordered by signal strength
    let meta_selectors: &[(&str, &str)] = if options.original_date {
        &[
            ("meta[property='article:published_time']", "content"),
            ("meta[property='og:article:published_time']", "content"),
            ("meta[name='date']", "content"),
            ("meta[name='dcterms.date']", "content"),
            ("meta[name='DC.date.issued']", "content"),
            ("meta[name='dc.date']", "content"),
            ("meta[name='sailthru.date']", "content"),
            ("meta[name='article.published']", "content"),
            ("meta[name='published-date']", "content"),
            ("meta[name='publication_date']", "content"),
            ("meta[itemprop='datePublished']", "content"),
            ("meta[http-equiv='date']", "content"),
            ("meta[property='article:modified_time']", "content"),
            ("meta[name='last-modified']", "content"),
            ("meta[itemprop='dateModified']", "content"),
        ]
    } else {
        &[
            ("meta[property='article:modified_time']", "content"),
            ("meta[name='last-modified']", "content"),
            ("meta[itemprop='dateModified']", "content"),
            ("meta[property='article:published_time']", "content"),
            ("meta[property='og:article:published_time']", "content"),
            ("meta[name='date']", "content"),
            ("meta[itemprop='datePublished']", "content"),
        ]
    };

    for (sel_str, attr) in meta_selectors {
        if let Ok(selector) = Selector::parse(sel_str) {
            for element in document.select(&selector) {
                if let Some(value) = element.value().attr(attr) {
                    if let Some(date) = try_parse_and_validate(value, options) {
                        return Some(date);
                    }
                }
            }
        }
    }
    None
}

/// Main entry point for Tier 1 metadata extraction.
pub fn extract_from_metadata(document: &Html, options: &DateOptions) -> Option<NaiveDate> {
    // JSON-LD is highest signal
    if let Some(date) = extract_json_ld(document, options) {
        return Some(date);
    }

    // Then Open Graph / meta tags
    extract_meta_tags(document, options)
}
