//! Output format conversion.
//!
//! Mirrors trafilatura's `xml.py` and `core.py:determine_returnstring()`.
//! Supports: TXT, Markdown, JSON, XML, XML-TEI, CSV, HTML.

use std::str::FromStr;

pub mod csv_output;
pub mod html_output;
pub mod json_output;
pub mod markdown;
pub mod txt;
pub mod xml_output;
pub mod xml_tei;

/// Supported output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "txt" => Ok(Self::Txt),
            "markdown" | "md" => Ok(Self::Markdown),
            "json" => Ok(Self::Json),
            "xml" => Ok(Self::Xml),
            "xmltei" => Ok(Self::XmlTei),
            "csv" => Ok(Self::Csv),
            "html" => Ok(Self::Html),
            _ => Err(format!("Unknown format: {s}")),
        }
    }
}
