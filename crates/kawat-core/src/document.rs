//! Extracted document model.
//! Mirrors trafilatura settings.py Document class.

use kawat_metadata::DocumentMetadata;
use serde::{Deserialize, Serialize};

use crate::config::ExtractorOptions;

/// A fully extracted document with text, metadata, and comments.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Document {
    /// Extracted metadata.
    #[serde(flatten)]
    pub metadata: DocumentMetadata,
    /// Main body text (internal representation).
    pub body: String,
    /// Extracted comments (if enabled).
    pub comments: Option<String>,
    /// Raw extracted text (before formatting).
    pub raw_text: Option<String>,
    /// Formatted output string.
    #[serde(skip)]
    pub text: Option<String>,
}

impl Document {
    /// Convert to the configured output format.
    pub fn to_formatted_string(&self, _options: &ExtractorOptions) -> String {
        // TODO: dispatch to kawat_output formatters
        self.body.clone()
    }

    /// Convert to a HashMap for Python-dict-like access.
    pub fn as_map(&self) -> std::collections::HashMap<String, Option<String>> {
        let mut map = std::collections::HashMap::new();
        map.insert("title".into(), self.metadata.title.clone());
        map.insert("author".into(), self.metadata.author.clone());
        map.insert("url".into(), self.metadata.url.clone());
        map.insert("date".into(), self.metadata.date.clone());
        map.insert("body".into(), Some(self.body.clone()));
        map.insert("comments".into(), self.comments.clone());
        map
    }
}
