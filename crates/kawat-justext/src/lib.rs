//! Pure Rust port of the jusText algorithm.
//!
//! jusText classifies paragraphs in HTML as boilerplate or content
//! based on text length, link density, and stopword frequency.
//!
//! Reference: Pomikálek, J. (2011). Removing Boilerplate and Duplicate Content from Web Corpora.
//!
//! Trafilatura uses custom parameters:
//! `classify_paragraphs(paragraphs, stoplist, 50, 150, 0.1, 0.2, 0.25, True)`
//! `revise_paragraph_classification(paragraphs, 150)`

pub mod classify;
pub mod paragraph;
pub mod stoplists;

// TODO: uncomment when functions are implemented
// pub use classify::classify_paragraphs;
// pub use paragraph::Paragraph;

/// Parameters for justext classification.
/// Defaults match trafilatura's custom_justext() in external.py.
#[derive(Debug, Clone)]
pub struct JusTextParams {
    pub length_low: usize,           // 50
    pub length_high: usize,          // 150
    pub stopwords_low: f64,          // 0.1
    pub stopwords_high: f64,         // 0.2
    pub max_link_density: f64,       // 0.25
    pub no_headings: bool,           // true
    pub max_heading_distance: usize, // 150 (for revise step)
}

impl Default for JusTextParams {
    fn default() -> Self {
        Self {
            length_low: 50,
            length_high: 150,
            stopwords_low: 0.1,
            stopwords_high: 0.2,
            max_link_density: 0.25,
            no_headings: true,
            max_heading_distance: 150,
        }
    }
}
