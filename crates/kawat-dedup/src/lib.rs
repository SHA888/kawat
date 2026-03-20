//! Content deduplication.
//!
//! Mirrors trafilatura's `deduplication.py`:
//! - Simhash for near-duplicate detection
//! - LRU cache at sentence, paragraph, and document level
//! - SHA-1 based content fingerprinting

pub mod fingerprint;
pub mod lru_cache;
pub mod simhash;

pub use fingerprint::content_fingerprint;
pub use lru_cache::DedupCache;

/// Generate a content fingerprint (SHA-1 based, Base64 encoded).
/// Mirrors trafilatura deduplication.py:content_fingerprint.
pub fn content_fingerprint_str(content: &str) -> String {
    fingerprint::content_fingerprint(content)
}
