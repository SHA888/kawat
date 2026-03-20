use chrono::NaiveDate;

use crate::settings::DateOptions;

/// Date format strings to try when parsing date strings, ordered by prevalence.
pub static DATE_FORMATS: &[&str] = &[
    // ISO 8601
    "%Y-%m-%d",
    "%Y-%m-%dT%H:%M:%S",
    "%Y-%m-%dT%H:%M:%SZ",
    "%Y-%m-%dT%H:%M:%S%z",
    "%Y-%m-%dT%H:%M:%S%.f%z",
    "%Y-%m-%dT%H:%M:%S%.fZ",
    // Common web formats
    "%Y/%m/%d",
    "%d/%m/%Y",
    "%m/%d/%Y",
    "%d.%m.%Y",
    "%Y.%m.%d",
    // Named months
    "%B %d, %Y",  // January 15, 2024
    "%b %d, %Y",  // Jan 15, 2024
    "%d %B %Y",   // 15 January 2024
    "%d %b %Y",   // 15 Jan 2024
    "%B %d %Y",   // January 15 2024
    "%b. %d, %Y", // Jan. 15, 2024
    // Compact
    "%Y%m%d",
];

/// Try to parse a date string using the curated format list.
pub fn parse_date_string(input: &str) -> Option<NaiveDate> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Try direct NaiveDate parsing first
    for fmt in DATE_FORMATS {
        if let Ok(date) = NaiveDate::parse_from_str(trimmed, fmt) {
            return Some(date);
        }
    }

    // Try parsing as DateTime then extracting the date
    for fmt in DATE_FORMATS {
        if let Ok(dt) = chrono::DateTime::parse_from_str(trimmed, fmt) {
            return Some(dt.date_naive());
        }
    }

    // Try extracting just a YYYY-MM-DD substring via regex (fallback)
    use crate::patterns;
    if let Some(cap) = patterns::ISO_DATE.captures(trimmed) {
        if let Some(matched) = cap.get(0) {
            return parse_date_string(matched.as_str());
        }
    }

    None
}

/// Validate that a date is plausible for web content.
pub fn is_valid_date(date: NaiveDate, options: &DateOptions) -> bool {
    let today = chrono::Local::now().date_naive();
    let max = options.max_date.unwrap_or(today);

    // Not in the future (2-day tolerance for timezone differences)
    if date > max + chrono::Duration::days(2) {
        return false;
    }

    // Not before minimum date
    if date < options.min_date {
        return false;
    }

    true
}

/// Parse and validate a date string in one step.
pub fn try_parse_and_validate(input: &str, options: &DateOptions) -> Option<NaiveDate> {
    parse_date_string(input).filter(|d| is_valid_date(*d, options))
}
