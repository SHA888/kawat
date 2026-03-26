//! Text node processing for different element types.
//! Handles titles, paragraphs, tables, lists, quotes, code blocks, etc.

use tracing::{debug, trace};

/// Process a text node based on its element type.
///
/// This function determines the appropriate handler based on the tag name
/// and applies trafilatura-style text extraction heuristics.
///
/// # Arguments
/// * `tag` - The HTML tag name
/// * `text` - The text content
/// * `attributes` - Element attributes (e.g., class, id)
///
/// # Returns
/// Processed text content, or None if the element should be skipped
pub fn process_node(tag: &str, text: &str, attributes: &str) -> Option<String> {
    trace!("Processing node: tag={}, text_len={}", tag, text.len());

    // Skip empty text
    if text.trim().is_empty() {
        return None;
    }

    match tag {
        "head" => handle_title(text),
        "p" => handle_paragraph(text),
        "table" => handle_table(text),
        "list" | "item" => handle_list(text),
        "quote" => handle_quote(text),
        "code" => handle_code(text, attributes),
        "hi" => handle_formatting(text),
        "ref" => handle_link(text),
        "img" => handle_image(text, attributes),
        "lb" => Some("\n".to_string()),
        _ => handle_generic(text),
    }
}

/// Handle title/heading text (from h1-h6 tags, converted to 'head').
fn handle_title(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    debug!("Handling title: {}", trimmed);
    Some(trimmed.to_string())
}

/// Handle paragraph text.
fn handle_paragraph(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Skip paragraphs that are just links or very short
    if trimmed.len() < 3 {
        return None;
    }

    debug!("Handling paragraph: {} chars", trimmed.len());
    Some(trimmed.to_string())
}

/// Handle table content.
fn handle_table(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    // For tables, preserve some structure by keeping newlines
    let processed = trimmed
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" | ");

    if processed.is_empty() {
        return None;
    }

    debug!("Handling table: {} chars", processed.len());
    Some(processed)
}

/// Handle list item text.
fn handle_list(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    debug!("Handling list item: {}", trimmed);
    Some(format!("• {trimmed}"))
}

/// Handle blockquote text.
fn handle_quote(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    debug!("Handling quote: {}", trimmed);
    Some(format!("\" {trimmed} \""))
}

/// Handle code block or inline code.
fn handle_code(text: &str, attributes: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    let is_block = attributes.contains("data-block=\"true\"");

    if is_block {
        debug!("Handling code block: {} chars", trimmed.len());
        Some(format!("\n```\n{trimmed}\n```\n"))
    } else {
        debug!("Handling inline code: {}", trimmed);
        Some(format!("`{trimmed}`"))
    }
}

/// Handle formatting (bold, italic, etc.).
fn handle_formatting(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    debug!("Handling formatting: {}", trimmed);
    Some(trimmed.to_string())
}

/// Handle link text.
fn handle_link(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    debug!("Handling link: {}", trimmed);
    Some(trimmed.to_string())
}

/// Handle image alt text.
fn handle_image(text: &str, _attributes: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    debug!("Handling image alt text: {}", trimmed);
    Some(format!("[Image: {trimmed}]"))
}

/// Handle generic/unknown element text.
fn handle_generic(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    debug!("Handling generic text: {} chars", trimmed.len());
    Some(trimmed.to_string())
}

/// Handle a text node - extract and process text content.
///
/// This is the main entry point for text node processing.
/// It extracts text from a node and applies appropriate transformations.
///
/// # Arguments
/// * `html` - The HTML content to process
/// * `tag` - The tag name of the element
/// * `attributes` - Element attributes
///
/// # Returns
/// Processed text content
pub fn handle_textnode(html: &str, tag: &str, attributes: &str) -> Option<String> {
    trace!("Handling text node: tag={}", tag);

    // Extract text content from the HTML
    // This is a simplified version - in production, would use proper HTML parsing
    let text = extract_text_content(html);

    // Process the text based on tag type
    process_node(tag, &text, attributes)
}

/// Extract plain text content from HTML.
///
/// This is a simplified text extraction that removes HTML tags.
/// In production, would use proper HTML parsing.
fn extract_text_content(html: &str) -> String {
    // Remove HTML tags using a simple regex-like approach
    let mut result = String::new();
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_title() {
        let result = handle_title("  My Title  ");
        assert_eq!(result, Some("My Title".to_string()));
    }

    #[test]
    fn test_handle_title_empty() {
        let result = handle_title("   ");
        assert_eq!(result, None);
    }

    #[test]
    fn test_handle_paragraph() {
        let result = handle_paragraph("  This is a paragraph with some text.  ");
        assert_eq!(
            result,
            Some("This is a paragraph with some text.".to_string())
        );
    }

    #[test]
    fn test_handle_paragraph_too_short() {
        let result = handle_paragraph("ab");
        assert_eq!(result, None);
    }

    #[test]
    fn test_handle_list() {
        let result = handle_list("Item 1");
        assert_eq!(result, Some("• Item 1".to_string()));
    }

    #[test]
    fn test_handle_quote() {
        let result = handle_quote("Famous quote");
        assert_eq!(result, Some("\" Famous quote \"".to_string()));
    }

    #[test]
    fn test_handle_code_block() {
        let result = handle_code("let x = 5;", "data-block=\"true\"");
        let result_str = result.unwrap();
        assert!(result_str.contains("```"));
        assert!(result_str.contains("let x = 5;"));
    }

    #[test]
    fn test_handle_code_inline() {
        let result = handle_code("variable", "data-block=\"false\"");
        assert_eq!(result, Some("`variable`".to_string()));
    }

    #[test]
    fn test_handle_image() {
        let result = handle_image("Alt text", "");
        assert_eq!(result, Some("[Image: Alt text]".to_string()));
    }

    #[test]
    fn test_process_node_title() {
        let result = process_node("head", "Title", "");
        assert_eq!(result, Some("Title".to_string()));
    }

    #[test]
    fn test_process_node_paragraph() {
        let result = process_node("p", "Paragraph text", "");
        assert_eq!(result, Some("Paragraph text".to_string()));
    }

    #[test]
    fn test_process_node_unknown() {
        let result = process_node("unknown", "Text content", "");
        assert_eq!(result, Some("Text content".to_string()));
    }

    #[test]
    fn test_extract_text_content() {
        let html = "<p>Hello <b>world</b>!</p>";
        let result = extract_text_content(html);
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_handle_textnode() {
        let html = "<p>Sample paragraph text</p>";
        let result = handle_textnode(html, "p", "");
        assert!(result.is_some());
        assert!(result.unwrap().contains("Sample paragraph text"));
    }
}
