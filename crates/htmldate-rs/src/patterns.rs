use once_cell::sync::Lazy;
use regex::Regex;

/// ISO date pattern: 2024-01-15
pub static ISO_DATE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(\d{4})-(\d{2})-(\d{2})\b").expect("valid regex"));

/// Compact date pattern: 20240115
#[allow(dead_code)]
pub static COMPACT_DATE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(\d{4})(\d{2})(\d{2})\b").expect("valid regex"));

/// Slash-separated: 2024/01/15 or 01/15/2024
#[allow(dead_code)]
pub static SLASH_DATE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(\d{1,4})/(\d{1,2})/(\d{2,4})\b").expect("valid regex"));

/// Dot-separated: 15.01.2024
#[allow(dead_code)]
pub static DOT_DATE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b(\d{1,2})\.(\d{1,2})\.(\d{4})\b").expect("valid regex"));

/// English named month patterns: January 15, 2024 / 15 January 2024
pub static NAMED_MONTH_EN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(January|February|March|April|May|June|July|August|September|October|November|December|Jan|Feb|Mar|Apr|Jun|Jul|Aug|Sep|Sept|Oct|Nov|Dec)\.?\s+(\d{1,2}),?\s+(\d{4})\b"
    ).expect("valid regex")
});

pub static DAY_MONTH_YEAR_EN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)\b(\d{1,2})\s+(January|February|March|April|May|June|July|August|September|October|November|December|Jan|Feb|Mar|Apr|Jun|Jul|Aug|Sep|Sept|Oct|Nov|Dec)\.?\s+(\d{4})\b"
    ).expect("valid regex")
});

/// Context patterns: "Published on ...", "Updated: ...", etc.
#[allow(dead_code)]
pub static DATE_CONTEXT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)(published|posted|updated|modified|created|date|datum|fecha|tanggal|terbit)\s*[:=\-–—]?\s*"
    ).expect("valid regex")
});
