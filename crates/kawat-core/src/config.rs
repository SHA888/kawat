//! Extraction configuration.
//! Mirrors trafilatura settings.py Extractor class (28 slots).

use kawat_output::OutputFormat;
use std::collections::HashSet;

/// Focus mode for extraction precision/recall tradeoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Balanced,
    Precision,
    Recall,
}

/// Complete extraction configuration.
/// Equivalent to trafilatura's Extractor class.
#[derive(Debug, Clone)]
pub struct ExtractorOptions {
    // Output
    pub format: OutputFormat,

    // Extraction strategy
    pub fast: bool,
    pub focus: Focus,

    // Content inclusion
    pub comments: bool,
    pub formatting: bool,
    pub links: bool,
    pub images: bool,
    pub tables: bool,

    // Deduplication
    pub dedup: bool,

    // Language
    pub target_language: Option<String>,

    // Metadata
    pub with_metadata: bool,
    pub only_with_metadata: bool,

    // Filtering
    pub url_blacklist: HashSet<String>,
    pub author_blacklist: HashSet<String>,
    pub prune_selectors: Vec<String>,

    // Date extraction
    pub date_params: htmldate_rs::DateOptions,

    // Size thresholds (from settings.cfg)
    pub min_extracted_size: usize,
    pub min_output_size: usize,
    pub min_output_comm_size: usize,
    pub min_extracted_comm_size: usize,
    pub min_duplcheck_size: usize,
    pub max_repetitions: usize,
}

impl Default for ExtractorOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::Txt,
            fast: false,
            focus: Focus::Balanced,
            comments: true,
            formatting: false,
            links: false,
            images: false,
            tables: true,
            dedup: false,
            target_language: None,
            with_metadata: false,
            only_with_metadata: false,
            url_blacklist: HashSet::new(),
            author_blacklist: HashSet::new(),
            prune_selectors: Vec::new(),
            date_params: htmldate_rs::DateOptions::default(),
            // Defaults from trafilatura settings.cfg
            min_extracted_size: 250,
            min_output_size: 1,
            min_output_comm_size: 1,
            min_extracted_comm_size: 1,
            min_duplcheck_size: 100,
            max_repetitions: 2,
        }
    }
}
