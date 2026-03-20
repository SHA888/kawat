//! Readability-based content extraction.
//!
//! Wraps `dom_smoothie` to provide the readability fallback
//! used in trafilatura's `external.py:try_readability()`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadabilityError {
    #[error("readability extraction failed: {0}")]
    ExtractionFailed(String),
}

/// Extract main content using the readability algorithm.
/// Returns (html_content, text_content, text_length).
pub fn try_readability(_html: &str) -> Result<(String, String, usize), ReadabilityError> {
    // dom_smoothie integration
    // Mirrors trafilatura external.py:try_readability
    // defaults: min_text_length=25, retry_length=250
    todo!("wrap dom_smoothie")
}
