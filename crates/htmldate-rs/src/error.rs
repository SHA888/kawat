/// Errors that can occur during date extraction.
#[derive(Debug, thiserror::Error)]
pub enum HtmlDateError {
    #[error("failed to parse HTML")]
    ParseError,
    #[error("no valid date found")]
    NotFound,
    #[error("date validation failed: {0}")]
    ValidationError(String),
}
