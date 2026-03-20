//! Main content extractor.
//!
//! Mirrors trafilatura's `main_extractor.py`:
//! - Iterate BODY_XPATH expressions (first match wins)
//! - Prune unwanted sections (OVERALL_DISCARD_XPATH + link density)
//! - Handle text elements by type (titles, paragraphs, tables, lists, quotes, code)
//! - Recover wild text if result too short

pub mod comments;
pub mod content;
pub mod handlers;
pub mod wild;

// TODO: uncomment when functions are implemented
// pub use content::extract_content;
// pub use comments::extract_comments;
