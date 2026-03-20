//! # kawat
//!
//! A Rust library for web content extraction, inspired by
//! [trafilatura](https://github.com/adbar/trafilatura).
//!
//! Extracts main text, metadata, and comments from HTML documents
//! with a multi-algorithm fallback cascade.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use kawat::{extract, fetch_url, ExtractorOptions};
//!
//! // From URL
//! let html = fetch_url("https://example.org/article").unwrap();
//! let text = extract(&html, &ExtractorOptions::default()).unwrap();
//!
//! // With options
//! let options = ExtractorOptions {
//!     with_metadata: true,
//!     ..Default::default()
//! };
//! let text = extract(&html, &options).unwrap();
//! ```
//!
//! ## Name
//!
//! *Kawat* is Indonesian for "wire" — the same metallurgical metaphor as
//! *trafilatura* (Italian for "wire drawing"), symbolizing the refinement
//! of raw HTML into clean, structured text.

pub use htmldate_rs;
pub use kawat_core::{
    Document, ExtractionError, ExtractorOptions, OutputFormat, bare_extraction, extract,
};

/// Fetch a URL and return the HTML content.
pub fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}

/// Async version of `fetch_url`.
pub async fn fetch_url_async(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.text().await
}
