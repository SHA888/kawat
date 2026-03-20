//! # htmldate-rs
//!
//! Fast and robust date extraction from web pages.
//!
//! Inspired by Python [htmldate](https://github.com/adbar/htmldate) by Adrien Barbaresi.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use htmldate_rs::{find_date, DateOptions};
//!
//! let html = r#"<html><head>
//!   <meta property="article:published_time" content="2024-01-15T10:00:00Z">
//! </head><body>Article text</body></html>"#;
//!
//! let date = find_date(html, &DateOptions::default());
//! assert_eq!(date, Some("2024-01-15".to_string()));
//! ```
//!
//! ## Extraction strategy
//!
//! Three-tier heuristic cascade:
//! 1. **Header metadata**: JSON-LD, Open Graph, standard meta tags
//! 2. **Structural markers**: `<time>`, `<abbr>`, date-related class/id attributes
//! 3. **Text heuristics**: regex-based date extraction from bare text content

mod error;
mod meta;
mod patterns;
mod settings;
mod structural;
mod text;
mod validators;

pub use error::HtmlDateError;
pub use settings::DateOptions;

use chrono::NaiveDate;

/// Find the publication date of an HTML document.
///
/// Returns the date as an ISO 8601 string (YYYY-MM-DD) or `None` if no valid date is found.
pub fn find_date(html: &str, options: &DateOptions) -> Option<String> {
    find_date_parsed(html, options).map(|d| d.format(&options.output_format).to_string())
}

/// Find the publication date and return it as a `NaiveDate`.
pub fn find_date_parsed(html: &str, options: &DateOptions) -> Option<NaiveDate> {
    let document = scraper::Html::parse_document(html);

    // Tier 1: header metadata (JSON-LD, Open Graph, meta tags)
    if let Some(date) = meta::extract_from_metadata(&document, options) {
        return Some(date);
    }

    // Tier 2: structural HTML markers (<time>, <abbr>, class/id patterns)
    if let Some(date) = structural::extract_from_structure(&document, options) {
        return Some(date);
    }

    // Tier 3: text heuristics (fast mode: precise patterns; extensive: full scan)
    if options.extensive_search {
        if let Some(date) = text::extract_from_text(&document, options) {
            return Some(date);
        }
    }

    None
}
