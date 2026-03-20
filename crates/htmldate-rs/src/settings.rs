use chrono::NaiveDate;

/// Configuration options for date extraction.
#[derive(Debug, Clone)]
pub struct DateOptions {
    /// Look for the original publication date (vs. last modified).
    pub original_date: bool,
    /// Enable extensive/slow search through bare text content.
    pub extensive_search: bool,
    /// Maximum acceptable date (YYYY-MM-DD). Dates after this are rejected.
    pub max_date: Option<NaiveDate>,
    /// Minimum acceptable date. Dates before this are rejected.
    pub min_date: NaiveDate,
    /// Output format string compatible with chrono's strftime.
    pub output_format: String,
}

impl Default for DateOptions {
    fn default() -> Self {
        Self {
            original_date: true,
            extensive_search: true,
            max_date: None, // will use today's date in validators
            min_date: NaiveDate::from_ymd_opt(1995, 1, 1).expect("valid date"),
            output_format: "%Y-%m-%d".to_string(),
        }
    }
}
