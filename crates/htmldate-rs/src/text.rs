//! Tier 3: Extract dates from bare text content using regex heuristics.

use chrono::NaiveDate;
use scraper::Html;

use crate::patterns;
use crate::settings::DateOptions;
use crate::validators::{is_valid_date, parse_date_string};

/// Collect all date candidates from the document text and pick the best one.
pub fn extract_from_text(document: &Html, options: &DateOptions) -> Option<NaiveDate> {
    let body_selector = scraper::Selector::parse("body").ok()?;
    let body = document.select(&body_selector).next()?;
    let text: String = body.text().collect();

    let mut candidates: Vec<NaiveDate> = Vec::new();

    // ISO dates
    for cap in patterns::ISO_DATE.captures_iter(&text) {
        if let Some(date) = parse_date_string(cap.get(0).map_or("", |m| m.as_str())) {
            if is_valid_date(date, options) {
                candidates.push(date);
            }
        }
    }

    // Named month patterns (English)
    for cap in patterns::NAMED_MONTH_EN.captures_iter(&text) {
        if let Some(date) = parse_date_string(cap.get(0).map_or("", |m| m.as_str())) {
            if is_valid_date(date, options) {
                candidates.push(date);
            }
        }
    }

    for cap in patterns::DAY_MONTH_YEAR_EN.captures_iter(&text) {
        if let Some(date) = parse_date_string(cap.get(0).map_or("", |m| m.as_str())) {
            if is_valid_date(date, options) {
                candidates.push(date);
            }
        }
    }

    // Disambiguation: prefer the first (top-of-page) date
    // This is a simplification of htmldate's full disambiguation algorithm
    candidates.first().copied()
}
