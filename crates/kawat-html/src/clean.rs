//! Tree cleaning: remove unwanted elements.
use crate::tags::{CUT_EMPTY_ELEMS, MANUALLY_CLEANED, MANUALLY_STRIPPED};
use regex::Regex;
use tracing::{debug, trace};

/// Remove unwanted elements from HTML tree.
///
/// # Arguments
/// * `html` - Input HTML string
/// * `preserve_images` - Whether to preserve img tags when stripping
///
/// # Returns
/// Cleaned HTML string
pub fn tree_cleaning(
    html: &str,
    preserve_images: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    trace!("Starting tree cleaning on {} bytes", html.len());

    let mut result = html.to_string();

    // Remove MANUALLY_CLEANED elements entirely
    for tag in MANUALLY_CLEANED {
        // Special handling for figure tags with tables
        if *tag == "figure" && result.contains("<table>") {
            // Convert figure to div before removing
            result = result
                .replace("<figure>", "<div>")
                .replace("</figure>", "</div>");
            debug!("Converted figure with table to div");
            continue;
        }

        // Simple regex to remove entire element with content
        let escaped_tag = regex::escape(tag);
        let regex = Regex::new(&format!(r#"<{escaped_tag}[^>]*>.*?</{escaped_tag}>"#))?;
        result = regex.replace_all(&result, "").to_string();
        debug!("Removed elements: {}", tag);
    }

    // Strip MANUALLY_STRIPPED tags (keep children)
    for tag in MANUALLY_STRIPPED {
        // Special handling for img tags
        if *tag == "img" && preserve_images {
            continue;
        }

        // Remove opening tag but keep content
        let escaped_tag = regex::escape(tag);
        let open_regex = Regex::new(&format!(r#"<{escaped_tag}[^>]*>"#))?;
        result = open_regex.replace_all(&result, "").to_string();

        // Remove closing tag
        let close_regex = Regex::new(&format!(r"</{escaped_tag}>"))?;
        result = close_regex.replace_all(&result, "").to_string();

        debug!("Stripped tag: {}", tag);
    }

    // Remove empty elements from CUT_EMPTY_ELEMS
    for tag in CUT_EMPTY_ELEMS {
        // Match empty tags: <tag></tag>, <tag />, <tag>   </tag>
        let escaped_tag = regex::escape(tag);
        let empty_regex = Regex::new(&format!(
            r#"<{escaped_tag}(?:\s*/>|>\s*</\s*{escaped_tag}>|>\s*\s*</\s*{escaped_tag}>)"#
        ))?;
        result = empty_regex.replace_all(&result, "").to_string();
        debug!("Removed empty {} elements", tag);
    }

    trace!("Tree cleaning completed, output length: {}", result.len());

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_cleaned_elements() {
        let html = r#"<div><script>alert('x')</script><p>Hello</p></div>"#;
        let result = tree_cleaning(html, false).unwrap();
        assert!(!result.contains("<script>"));
        assert!(result.contains("<p>Hello</p>"));
    }

    #[test]
    fn test_strip_tags_keep_content() {
        let html = r#"<div><abbr>HTML</abbr><p>Text</p></div>"#;
        let result = tree_cleaning(html, false).unwrap();
        assert!(!result.contains("<abbr>"));
        assert!(result.contains("HTML"));
        assert!(result.contains("<p>Text</p>"));
    }

    #[test]
    fn test_preserve_images() {
        let html = r#"<div><img src="test.jpg"/><p>Text</p></div>"#;
        let result = tree_cleaning(html, true).unwrap();
        assert!(result.contains("<img"));
        let result = tree_cleaning(html, false).unwrap();
        assert!(!result.contains("<img>"));
        assert!(result.contains("Text"));
    }

    #[test]
    fn test_figure_table_conversion() {
        let html = r#"<figure><table><tr><td>Cell</td></tr></table></figure>"#;
        let result = tree_cleaning(html, false).unwrap();
        println!("Input: {html}");
        println!("Output: {result}");
        assert!(result.contains("<div>"));
        assert!(!result.contains("<figure>"));
    }

    #[test]
    fn test_remove_empty_elements() {
        let html = r#"<div><p></p><span> </span><div>Text</div></div>"#;
        let result = tree_cleaning(html, false).unwrap();
        assert!(!result.contains("<p></p>"));
        assert!(!result.contains("<span> </span>"));
        assert!(result.contains("<div>Text</div>"));
    }
}
