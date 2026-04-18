//! Output format conversion.
//!
//! Mirrors trafilatura's `xml.py` and `core.py:determine_returnstring()`.
//! Supports: TXT, Markdown, JSON, XML, XML-TEI, CSV, HTML.

use std::str::FromStr;
use thiserror::Error;

/// Error type for output format operations.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OutputError {
    /// Unknown output format string.
    #[error("unknown format: {0}")]
    UnknownFormat(String),
}

pub mod csv_output;
pub mod html_output;
pub mod json_output;
pub mod markdown;
pub mod txt;
pub mod xml_output;
pub mod xml_tei;

pub use txt::{to_txt, to_txt_body_only};

/// Supported output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum OutputFormat {
    Txt,
    Markdown,
    Json,
    Xml,
    XmlTei,
    Csv,
    Html,
}

impl FromStr for OutputFormat {
    type Err = OutputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "txt" => Ok(Self::Txt),
            "markdown" | "md" => Ok(Self::Markdown),
            "json" => Ok(Self::Json),
            "xml" => Ok(Self::Xml),
            "xmltei" => Ok(Self::XmlTei),
            "csv" => Ok(Self::Csv),
            "html" => Ok(Self::Html),
            _ => Err(OutputError::UnknownFormat(s.to_string())),
        }
    }
}
