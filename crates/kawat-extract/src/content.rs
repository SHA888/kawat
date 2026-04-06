//! content extraction module.
// TODO: port from trafilatura main_extractor.py

use crate::tree::{KawatNode, KawatTree};
use kawat_xpath::CompiledXpaths;
use std::error::Error;
use std::fmt;

/// Error types for content extraction.
#[derive(Debug)]
pub enum ExtractionError {
    /// No suitable content area found.
    NoContentFound,
    /// XPath evaluation error.
    XPathError(String),
    /// Tree parsing error.
    TreeError(String),
}

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtractionError::NoContentFound => write!(f, "No suitable content area found"),
            ExtractionError::XPathError(msg) => write!(f, "XPath error: {}", msg),
            ExtractionError::TreeError(msg) => write!(f, "Tree error: {}", msg),
        }
    }
}

impl Error for ExtractionError {}

/// Escape markdown special characters in text content.
fn escape_markdown(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('`', "\\`")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

/// Internal extraction function: iterate BODY_XPATH, first match wins.
///
/// This mirrors trafilatura's `_extract()` function which tries each BODY_XPATH
/// expression in order and returns the first successful match.
pub fn _extract(tree: &KawatTree) -> Result<Option<KawatNode>, ExtractionError> {
    // TODO: Implement proper XPath evaluation on KawatTree
    // For now, we'll use a simple fallback: return the body element
    // The loop structure is preserved for future XPath implementation
    for _xpath_expr in CompiledXpaths::BODY {
        // Try each XPath expression (placeholder implementation)
        // When XPath is implemented, this would evaluate each expression
        // and return the first successful match
        if let Some(body) = tree.get_body() {
            return Ok(Some(KawatNode {
                tag: body.tag.clone(),
                text: body.text.clone(),
                tail: body.tail.clone(),
                attributes: body.attributes.clone(),
                children: body.children.clone(),
                parent_tag: body.parent_tag.clone(),
            }));
        }
    }

    // No content found
    Ok(None)
}

/// Calculate link density for a node (ratio of link text to total text).
fn calculate_link_density(node: &KawatNode) -> f32 {
    let total_text = node.text_content();
    if total_text.trim().is_empty() {
        return 0.0;
    }

    let mut link_text = String::new();
    for link in node.find_by_tag("a") {
        link_text.push_str(&link.text_content());
    }

    if link_text.trim().is_empty() {
        0.0
    } else {
        link_text.len() as f32 / total_text.len() as f32
    }
}

/// Prune unwanted sections using OVERALL_DISCARD + link density passes.
///
/// This mirrors trafilatura's `prune_unwanted_sections()` function which removes
/// navigation, footers, ads, and other unwanted content.
pub fn prune_unwanted_sections(node: &mut KawatNode) -> Result<(), ExtractionError> {
    // TODO: Implement XPath-based removal using OVERALL_DISCARD expressions
    // For now, we'll implement a simple placeholder

    // Remove elements with common unwanted class names
    let unwanted_classes = [
        "footer",
        "header",
        "nav",
        "sidebar",
        "menu",
        "advertisement",
        "ad",
        "banner",
        "social",
        "share",
        "comment",
        "related",
    ];

    // This is a simplified version - full implementation would use XPath
    _remove_nodes_by_classes(node, &unwanted_classes);

    // Additional pruning based on link density (simplified)
    _prune_by_link_density(node);

    Ok(())
}

/// Helper function to remove nodes with high link density.
fn _prune_by_link_density(node: &mut KawatNode) {
    let link_density = calculate_link_density(node);

    // Remove nodes with high link density (> 0.8 indicates navigation/link-heavy content)
    if link_density > 0.8 && node.text_content().len() > 100 {
        node.children.clear();
        node.text = None;
        node.tail = None;
        return;
    }

    // Recursively process children
    for child in &mut node.children {
        _prune_by_link_density(child);
    }
}

/// Helper function to remove nodes by class names (simplified implementation).
fn _remove_nodes_by_classes(node: &mut KawatNode, unwanted_classes: &[&str]) {
    // First, check if this node itself should be removed
    let has_unwanted_class = node
        .get_attribute("class")
        .map(|class| {
            unwanted_classes.iter().any(|unwanted| {
                class.contains(unwanted) || class.split_whitespace().any(|c| c == *unwanted)
            })
        })
        .unwrap_or(false);

    if has_unwanted_class {
        // Clear this node's children and text to effectively remove it
        node.children.clear();
        node.text = None;
        node.tail = None;
        return;
    }

    // Process children recursively
    node.children.retain(|child| {
        let child_has_unwanted_class = child
            .get_attribute("class")
            .map(|class| {
                unwanted_classes.iter().any(|unwanted| {
                    class.contains(unwanted) || class.split_whitespace().any(|c| c == *unwanted)
                })
            })
            .unwrap_or(false);
        !child_has_unwanted_class
    });

    // Recursively process remaining children
    for child in &mut node.children {
        _remove_nodes_by_classes(child, unwanted_classes);
    }
}

/// Dispatcher for handling text elements by tag type.
///
/// This mirrors trafilatura's `handle_textelem()` function which dispatches
/// to specific handlers based on the element tag.
pub fn handle_textelem(node: &KawatNode) -> Result<String, ExtractionError> {
    match node.tag.as_str() {
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => handle_titles(node),
        "p" => handle_paragraphs(node),
        "b" | "strong" | "i" | "em" | "u" | "span" => handle_formatting(node),
        "ul" | "ol" | "dl" => handle_lists(node),
        "blockquote" | "quote" => handle_quotes(node),
        "pre" | "code" => handle_code_blocks(node),
        "table" => handle_table(node),
        "img" => handle_image(node),
        _ => handle_other_elements(node),
    }
}

/// Handle title elements (h1-h6).
pub fn handle_titles(node: &KawatNode) -> Result<String, ExtractionError> {
    let text = node.text_content();
    if !text.trim().is_empty() {
        // Add appropriate markdown heading based on tag level
        let level = match node.tag.as_str() {
            "h1" => 1,
            "h2" => 2,
            "h3" => 3,
            "h4" => 4,
            "h5" => 5,
            "h6" => 6,
            _ => 1,
        };
        let escaped_text = escape_markdown(text.trim());
        Ok(format!("{} {}\n\n", "#".repeat(level), escaped_text))
    } else {
        Ok(String::new())
    }
}

/// Handle paragraph elements (most complex handler).
pub fn handle_paragraphs(node: &KawatNode) -> Result<String, ExtractionError> {
    let text = node.text_content();
    if !text.trim().is_empty() {
        // Process nested elements within paragraphs
        let mut processed_text = String::new();

        // Handle nested formatting elements properly
        for child in &node.children {
            let child_content = handle_textelem(child)?;
            processed_text.push_str(&child_content);
        }

        // If no children processed, use direct text content
        if processed_text.trim().is_empty() {
            processed_text = text.trim().to_string();
        }

        processed_text.push_str("\n\n");

        Ok(processed_text)
    } else {
        Ok(String::new())
    }
}

/// Handle formatting elements (b, strong, i, em, u, span).
pub fn handle_formatting(node: &KawatNode) -> Result<String, ExtractionError> {
    let text = node.text_content();
    if !text.trim().is_empty() {
        let escaped_text = escape_markdown(text.trim());
        // Apply markdown formatting based on tag
        let formatted = match node.tag.as_str() {
            "b" | "strong" => format!("**{}**", escaped_text),
            "i" | "em" => format!("*{}*", escaped_text),
            "u" => format!("__{}__", escaped_text),
            "span" => escaped_text,
            _ => escaped_text,
        };
        Ok(formatted)
    } else {
        Ok(String::new())
    }
}

/// Handle list elements (ul, ol, dl).
pub fn handle_lists(node: &KawatNode) -> Result<String, ExtractionError> {
    let mut result = String::new();
    let is_ordered = node.tag.as_str() == "ol";

    for (i, child) in node.children.iter().enumerate() {
        if child.tag == "li" {
            let prefix = if is_ordered {
                format!("{}.", i + 1)
            } else {
                "•".to_string()
            };

            // Handle nested content within list items
            let mut item_content = String::new();

            // Add direct text content first
            if let Some(text) = &child.text {
                item_content.push_str(text.trim());
            }

            for grandchild in &child.children {
                if grandchild.tag == "ul" || grandchild.tag == "ol" {
                    // Handle nested lists with proper indentation
                    let nested_list = handle_lists(grandchild)?;
                    let indented_nested = nested_list
                        .lines()
                        .map(|line| format!("  {}", line))
                        .collect::<Vec<_>>()
                        .join("\n");
                    item_content.push_str(&format!("\n{}", indented_nested));
                } else {
                    let content = handle_textelem(grandchild)?;
                    item_content.push_str(&content);
                }
            }

            // If no content processed, use text content directly
            if item_content.trim().is_empty() {
                item_content = child.text_content().trim().to_string();
            }

            if !item_content.trim().is_empty() {
                result.push_str(&format!("{} {}\n", prefix, item_content.trim()));
            }
        }
    }

    result.push('\n');
    Ok(result)
}

/// Handle quote elements.
pub fn handle_quotes(node: &KawatNode) -> Result<String, ExtractionError> {
    let text = node.text_content();
    if !text.trim().is_empty() {
        let escaped_text = escape_markdown(text.trim());
        Ok(format!("> {}\n\n", escaped_text))
    } else {
        Ok(String::new())
    }
}

/// Handle code block elements.
pub fn handle_code_blocks(node: &KawatNode) -> Result<String, ExtractionError> {
    let text = node.text_content();
    if !text.trim().is_empty() {
        if node.tag == "pre" {
            Ok(format!("```\n{}\n```\n\n", text.trim()))
        } else if node.tag == "code" {
            Ok(format!("`{}`", text.trim()))
        } else {
            Ok(text.trim().to_string())
        }
    } else {
        Ok(String::new())
    }
}

/// Handle table elements (includes cell type detection, nested content).
pub fn handle_table(node: &KawatNode) -> Result<String, ExtractionError> {
    let mut result = String::new();
    let mut header_found = false;

    for child in &node.children {
        if child.tag.as_str() == "tr" {
            let mut row_text = String::new();
            let mut cell_count = 0;
            for cell in &child.children {
                if cell.tag == "th" || cell.tag == "td" {
                    let cell_text = cell.text_content();
                    let escaped_cell = escape_markdown(cell_text.trim());
                    row_text.push_str(&format!("| {} ", escaped_cell));
                    cell_count += 1;
                }
            }
            row_text.push_str("|\n");
            result.push_str(&row_text);

            // Add header separator after first row (typically headers)
            if !header_found && child.children.iter().any(|c| c.tag == "th") {
                let separator = "|---".repeat(cell_count) + "|";
                result.push_str(&format!("{}\n", separator));
                header_found = true;
            }
        }
    }

    if !result.is_empty() {
        result.push('\n');
    }

    Ok(result)
}

/// Handle image elements.
pub fn handle_image(node: &KawatNode) -> Result<String, ExtractionError> {
    if let Some(src) = node.get_attribute("src") {
        let alt = node.get_attribute("alt").unwrap_or("");
        let escaped_alt = escape_markdown(alt);
        let escaped_src = escape_markdown(src);
        Ok(format!("![{}]({})\n", escaped_alt, escaped_src))
    } else {
        Ok(String::new())
    }
}

/// Handle other elements not covered by specific handlers.
pub fn handle_other_elements(node: &KawatNode) -> Result<String, ExtractionError> {
    let text = node.text_content();
    if !text.trim().is_empty() {
        // For unknown elements, just return the text content
        let escaped_text = escape_markdown(text.trim());
        Ok(format!("{}\n", escaped_text))
    } else {
        Ok(String::new())
    }
}

/// Recover wild text if result too short.
///
/// This mirrors trafilatura's `recover_wild_text()` function which attempts
/// to find additional text content when the main extraction yields too little.
pub fn recover_wild_text(
    _tree: &KawatTree,
    current_result: &str,
) -> Result<String, ExtractionError> {
    // TODO: Implement wild text recovery
    // For now, just return the current result
    Ok(current_result.to_string())
}

/// Main content extraction wrapper.
///
/// This mirrors trafilatura's `extract_content()` function which orchestrates
/// the entire content extraction process.
pub fn extract_content(tree: &KawatTree) -> Result<String, ExtractionError> {
    // Step 1: Extract main content area
    let content_node = _extract(tree)?;

    let mut content_node = match content_node {
        Some(node) => node,
        None => return Err(ExtractionError::NoContentFound),
    };

    // Step 2: Prune unwanted sections
    prune_unwanted_sections(&mut content_node)?;

    // Step 3: Process text elements
    let mut result = String::new();
    for child in &content_node.children {
        let processed = handle_textelem(child)?;
        result.push_str(&processed);
    }

    // Step 4: Recover wild text if needed
    if result.len() < 200 {
        // Arbitrary threshold
        result = recover_wild_text(tree, &result)?;
    }

    Ok(result)
}

/// Last-resort extraction (JSON-LD→article→p→body).
///
/// This mirrors trafilatura's `baseline()` function which provides fallback
/// extraction when the main extraction fails.
pub fn baseline(tree: &KawatTree) -> Result<String, ExtractionError> {
    // TODO: Implement baseline extraction with fallback chain
    // For now, just extract all paragraphs
    let mut result = String::new();

    for paragraph in tree.find_by_tag("p") {
        let text = paragraph.text_content();
        if !text.trim().is_empty() {
            result.push_str(&format!("{}\n\n", text.trim()));
        }
    }

    if result.is_empty() {
        // Fallback to body text
        if let Some(body) = tree.get_body() {
            let text = body.text_content();
            if !text.trim().is_empty() {
                result.push_str(&text);
            }
        }
    }

    Ok(result)
}

/// Convert HTML to plain text.
///
/// This mirrors trafilatura's `html2txt()` function which provides
/// a simple HTML-to-text conversion.
pub fn html2txt(tree: &KawatTree) -> Result<String, ExtractionError> {
    let mut result = String::new();

    // Simple text extraction - traverse all nodes and collect text
    for node in tree.iter_descendants() {
        if let Some(text) = &node.text {
            result.push_str(text);
        }
        if let Some(tail) = &node.tail {
            result.push_str(tail);
        }
    }

    // Clean up whitespace but preserve paragraph structure
    result = result.replace("\n\n\n", "\n\n"); // Remove excessive newlines
    result = result.trim().to_string();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::KawatNode;

    fn create_test_tree() -> KawatTree {
        let mut root = KawatNode::new("html");
        let mut body = KawatNode::new("body");

        // Add a title
        let title = KawatNode::with_text("h1", "Test Article Title");
        body.add_child(title);

        // Add a paragraph
        let paragraph = KawatNode::with_text("p", "This is a test paragraph with some content.");
        body.add_child(paragraph);

        // Add a list
        let mut list = KawatNode::new("ul");
        let item1 = KawatNode::with_text("li", "First item");
        let item2 = KawatNode::with_text("li", "Second item");
        list.add_child(item1);
        list.add_child(item2);
        body.add_child(list);

        root.add_child(body);
        KawatTree::new(root)
    }

    #[test]
    fn test_extract_content() {
        let tree = create_test_tree();
        let result = extract_content(&tree);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(content.contains("Test Article Title"));
        assert!(content.contains("This is a test paragraph"));
        assert!(content.contains("First item"));
        assert!(content.contains("Second item"));
    }

    #[test]
    fn test_handle_titles() {
        let h1 = KawatNode::with_text("h1", "Main Title");
        let h2 = KawatNode::with_text("h2", "Subtitle");

        let result1 = handle_titles(&h1).unwrap();
        assert_eq!(result1, "# Main Title\n\n");

        let result2 = handle_titles(&h2).unwrap();
        assert_eq!(result2, "## Subtitle\n\n");
    }

    #[test]
    fn test_handle_paragraphs() {
        let p = KawatNode::with_text("p", "This is a paragraph.");
        let result = handle_paragraphs(&p).unwrap();
        assert_eq!(result, "This is a paragraph.\n\n");
    }

    #[test]
    fn test_handle_formatting() {
        let bold = KawatNode::with_text("strong", "Bold text");
        let italic = KawatNode::with_text("em", "Italic text");

        let result_bold = handle_formatting(&bold).unwrap();
        assert_eq!(result_bold, "**Bold text**");

        let result_italic = handle_formatting(&italic).unwrap();
        assert_eq!(result_italic, "*Italic text*");
    }

    #[test]
    fn test_handle_lists() {
        let mut ul = KawatNode::new("ul");
        let item1 = KawatNode::with_text("li", "First");
        let item2 = KawatNode::with_text("li", "Second");
        ul.add_child(item1);
        ul.add_child(item2);

        let result = handle_lists(&ul).unwrap();
        assert!(result.contains("• First"));
        assert!(result.contains("• Second"));
    }

    #[test]
    fn test_handle_quotes() {
        let quote = KawatNode::with_text("blockquote", "This is a quote.");
        let result = handle_quotes(&quote).unwrap();
        assert_eq!(result, "> This is a quote.\n\n");
    }

    #[test]
    fn test_handle_code_blocks() {
        let pre = KawatNode::with_text("pre", "code block");
        let code = KawatNode::with_text("code", "inline code");

        let result_pre = handle_code_blocks(&pre).unwrap();
        assert!(result_pre.contains("```"));
        assert!(result_pre.contains("code block"));

        let result_code = handle_code_blocks(&code).unwrap();
        assert_eq!(result_code, "`inline code`");
    }

    #[test]
    fn test_handle_image() {
        let mut img = KawatNode::new("img");
        img.set_attribute("src", "https://example.com/image.jpg");
        img.set_attribute("alt", "Example image");

        let result = handle_image(&img).unwrap();
        assert_eq!(result, "![Example image](https://example.com/image.jpg)\n");
    }

    #[test]
    fn test_baseline() {
        let tree = create_test_tree();
        let result = baseline(&tree).unwrap();

        assert!(result.contains("This is a test paragraph"));
    }

    #[test]
    fn test_html2txt() {
        let tree = create_test_tree();
        let result = html2txt(&tree).unwrap();

        assert!(result.contains("Test Article Title"));
        assert!(result.contains("This is a test paragraph"));
        assert!(result.contains("First item"));
        assert!(result.contains("Second item"));
    }

    #[test]
    fn test_escape_markdown() {
        let text_with_special_chars = "Text with *bold* and _italic_ and `code`";
        let escaped = escape_markdown(text_with_special_chars);

        assert!(escaped.contains("\\*bold\\*"));
        assert!(escaped.contains("\\_italic\\_"));
        assert!(escaped.contains("\\`code\\`"));
    }

    #[test]
    fn test_link_density() {
        let mut div = KawatNode::new("div");
        div.text = Some("Normal text ".to_string());

        let mut link = KawatNode::new("a");
        link.text = Some("link text".to_string());
        div.add_child(link);

        let density = calculate_link_density(&div);
        assert!(density > 0.0);
        assert!(density < 1.0);
    }

    #[test]
    fn test_table_with_headers() {
        let mut table = KawatNode::new("table");
        let mut header_row = KawatNode::new("tr");

        let th1 = KawatNode::with_text("th", "Header 1");
        let th2 = KawatNode::with_text("th", "Header 2");
        header_row.add_child(th1);
        header_row.add_child(th2);

        let mut data_row = KawatNode::new("tr");
        let td1 = KawatNode::with_text("td", "Data 1");
        let td2 = KawatNode::with_text("td", "Data 2");
        data_row.add_child(td1);
        data_row.add_child(td2);

        table.add_child(header_row);
        table.add_child(data_row);

        let result = handle_table(&table).unwrap();
        assert!(result.contains("| Header 1 | Header 2 |"));
        assert!(result.contains("|---|---|"));
        assert!(result.contains("| Data 1 | Data 2 |"));
    }

    #[test]
    fn test_nested_lists() {
        let mut ul = KawatNode::new("ul");
        let mut li = KawatNode::new("li");
        li.text = Some("Parent item".to_string());

        let mut nested_ul = KawatNode::new("ul");
        let nested_li = KawatNode::with_text("li", "Nested item");
        nested_ul.add_child(nested_li);

        li.add_child(nested_ul);
        ul.add_child(li);

        let result = handle_lists(&ul).unwrap();
        assert!(result.contains("• Parent item"));
        assert!(result.contains("• Nested item"));
    }

    #[test]
    fn test_code_block_fix() {
        let pre = KawatNode::with_text("pre", "console.log('hello');");
        let result = handle_code_blocks(&pre).unwrap();

        assert!(result.contains("```"));
        assert!(result.contains("console.log('hello');"));
        assert!(result.ends_with("\n\n"));
        // Should have closing backticks
        assert!(result.matches("```").count() == 2);
    }
}
