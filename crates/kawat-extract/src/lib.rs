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
pub mod parser;
pub mod transforms;
pub mod tree;
pub mod wild;

// Export tree types for external use
pub use parser::{ParseError, parse_html_to_tree, tree_to_html};
pub use transforms::{apply_html_transformations, apply_tag_conversion, apply_tree_cleaning};
pub use tree::{KawatNode, KawatTree};

// TODO: uncomment when functions are implemented
pub use content::{ExtractionError, baseline, extract_content, html2txt};
// pub use comments::extract_comments;
