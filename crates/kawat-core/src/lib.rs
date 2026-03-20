//! Core extraction orchestrator.
//!
//! Implements the full trafilatura extraction cascade:
//!
//! ```text
//! HTML → parse → metadata → clean → convert tags → extract comments
//!   → trafilatura_sequence:
//!       extract_content (BODY_XPATH)
//!       → if not fast: compare_extraction (readability + justext)
//!       → if still short: baseline
//!   → size checks → dedup → language filter → output format
//! ```

pub mod cascade;
pub mod compare;
pub mod config;
pub mod document;

pub use config::ExtractorOptions;
pub use document::Document;
pub use kawat_output::OutputFormat;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExtractionError {
    #[error("HTML parsing failed")]
    ParseError,
    #[error("document too short (len={0}, min={1})")]
    TooShort(usize, usize),
    #[error("duplicate document")]
    Duplicate,
    #[error("wrong language: expected {expected}, got {got:?}")]
    WrongLanguage {
        expected: String,
        got: Option<String>,
    },
    #[error("blacklisted URL: {0}")]
    BlacklistedUrl(String),
    #[error("missing required metadata")]
    MissingMetadata,
}

/// Extract content from an HTML document.
///
/// This is the main entry point, equivalent to trafilatura's `bare_extraction()`.
pub fn bare_extraction(
    html: &str,
    options: &ExtractorOptions,
) -> Result<Document, ExtractionError> {
    cascade::run(html, options)
}

/// Extract and format content, equivalent to trafilatura's `extract()`.
pub fn extract(html: &str, options: &ExtractorOptions) -> Result<String, ExtractionError> {
    let doc = bare_extraction(html, options)?;
    Ok(doc.to_formatted_string(options))
}
