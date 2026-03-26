//! HTML parsing to KawatTree structure.
//! Converts HTML strings to KawatNode/KawatTree using the scraper crate.

use crate::tree::{KawatNode, KawatTree};
use scraper::{ElementRef, Html, Selector};
use thiserror::Error;

/// Errors that can occur during HTML parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid HTML: {0}")]
    InvalidHtml(String),
    #[error("Scraper error: {0}")]
    ScraperError(String),
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

/// Parse HTML string to KawatTree structure.
///
/// This function uses the scraper crate to parse HTML and converts it to our
/// custom KawatTree representation, preserving text/tail distinction.
pub fn parse_html_to_tree(html: &str) -> Result<KawatTree, ParseError> {
    let document = Html::parse_document(html);

    // Find the root element (should be html)
    let html_selector =
        Selector::parse("html").map_err(|e| ParseError::ScraperError(e.to_string()))?;

    if let Some(html_element) = document.select(&html_selector).next() {
        let root_node = convert_element_to_node(html_element, None)?;
        Ok(KawatTree::new(root_node))
    } else {
        // If no html element found, try to find body
        let body_selector =
            Selector::parse("body").map_err(|e| ParseError::ScraperError(e.to_string()))?;

        if let Some(body_element) = document.select(&body_selector).next() {
            // Found body, wrap it in html
            let mut root = KawatNode::new("html");
            let body = convert_element_to_node(body_element, Some("html".to_string()))?;
            root.add_child(body);
            Ok(KawatTree::new(root))
        } else {
            // No html or body found, create both and add all content to body
            let mut root = KawatNode::new("html");
            let mut body = KawatNode::new("body");

            // Parse all direct children as body children
            for child in document.root_element().children() {
                if let Some(element) = ElementRef::wrap(child) {
                    let child_node = convert_element_to_node(element, Some("body".to_string()))?;
                    body.add_child(child_node);
                }
            }

            root.add_child(body);
            Ok(KawatTree::new(root))
        }
    }
}

/// Convert a scraper ElementRef to a KawatNode.
fn convert_element_to_node(
    element: ElementRef,
    parent_tag: Option<String>,
) -> Result<KawatNode, ParseError> {
    let tag_name = element.value().name().to_lowercase();
    let mut node = KawatNode {
        tag: tag_name.clone(),
        text: None,
        tail: None,
        attributes: std::collections::HashMap::new(),
        children: Vec::new(),
        parent_tag,
    };

    // Extract attributes
    for (name, value) in element.value().attrs() {
        node.set_attribute(name, value);
    }

    // Process children and text content
    let mut accumulated_text = String::new();
    let mut text_before_first_child = true;

    for child in element.children() {
        match child.value() {
            scraper::Node::Text(text) => {
                // Accumulate text content
                accumulated_text.push_str(text);
            }
            scraper::Node::Element(_) => {
                // Convert child element
                let child_element = ElementRef::wrap(child).ok_or_else(|| {
                    ParseError::ConversionError("Failed to wrap element".to_string())
                })?;
                let mut child_node =
                    convert_element_to_node(child_element, Some(tag_name.clone()))?;

                // If this is the first child and we have accumulated text, it's the node's text
                if text_before_first_child && !accumulated_text.is_empty() {
                    node.text = Some(accumulated_text.clone());
                    accumulated_text.clear();
                } else if !text_before_first_child && !accumulated_text.is_empty() {
                    // Text after a child becomes that child's tail
                    child_node.tail = Some(accumulated_text.clone());
                    accumulated_text.clear();
                }

                node.add_child(child_node);
                text_before_first_child = false;
            }
            scraper::Node::Comment(_) => {
                // Skip comments
            }
            _ => {
                // Skip other node types
            }
        }
    }

    // Any remaining text after all children becomes the last child's tail
    // or the node's tail if there are no children
    if !accumulated_text.is_empty() {
        if node.children.is_empty() {
            // No children, so this is the node's text
            if node.text.is_none() {
                node.text = Some(accumulated_text);
            }
        } else {
            // Has children, so this is the last child's tail
            if let Some(last_child) = node.children.last_mut() {
                last_child.tail = Some(accumulated_text);
            }
        }
    }

    Ok(node)
}

/// Convert a KawatTree back to HTML string.
///
/// This is useful for debugging and testing.
pub fn tree_to_html(tree: &KawatTree) -> String {
    node_to_html(&tree.root)
}

/// Escape HTML entities in text content.
fn escape_html_entities(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Convert a KawatNode to HTML string.
fn node_to_html(node: &KawatNode) -> String {
    let mut result = String::new();

    // Opening tag
    result.push('<');
    result.push_str(&node.tag);

    // Attributes
    if !node.attributes.is_empty() {
        let mut attrs: Vec<_> = node.attributes.iter().collect();
        attrs.sort_by(|a, b| a.0.cmp(b.0));
        for (name, value) in attrs {
            result.push(' ');
            result.push_str(name);
            result.push_str("=\"");
            // Escape HTML entities in attribute values
            let escaped = escape_html_entities(value);
            result.push_str(&escaped);
            result.push('"');
        }
    }

    // Self-closing tags
    if node.children.is_empty()
        && node.text.is_none()
        && node.tail.is_none()
        && matches!(
            node.tag.as_str(),
            "br" | "hr" | "img" | "input" | "meta" | "link"
        )
    {
        result.push_str("/>");
        return result;
    }

    result.push('>');

    // Text content (escaped)
    if let Some(text) = &node.text {
        result.push_str(&escape_html_entities(text));
    }

    // Children
    for child in &node.children {
        result.push_str(&node_to_html(child));
    }

    // Closing tag
    result.push_str("</");
    result.push_str(&node.tag);
    result.push('>');

    // Tail text (escaped)
    if let Some(tail) = &node.tail {
        result.push_str(&escape_html_entities(tail));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let html = r#"<html><body><p>Hello World</p></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();

        assert_eq!(tree.root.tag, "html");

        // Find the p tag in the tree
        let p_tags = tree.find_by_tag("p");
        assert_eq!(p_tags.len(), 1);
        assert_eq!(p_tags[0].text, Some("Hello World".to_string()));
    }

    #[test]
    fn test_parse_html_with_attributes() {
        let html =
            r#"<html><body><a href="https://example.com" class="link">Click me</a></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();

        let a_tags = tree.find_by_tag("a");
        let a_tag = a_tags.first().unwrap();
        assert_eq!(a_tag.get_attribute("href"), Some("https://example.com"));
        assert_eq!(a_tag.get_attribute("class"), Some("link"));
    }

    #[test]
    fn test_parse_nested_elements() {
        let html = r#"<html><body><div>Before<p>Inside</p>After</div></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();

        let divs = tree.find_by_tag("div");
        let div = divs.first().unwrap();
        assert_eq!(div.text, Some("Before".to_string()));
        assert_eq!(div.children.len(), 1);
        assert_eq!(div.children[0].tag, "p");
        assert_eq!(div.children[0].text, Some("Inside".to_string()));
    }

    #[test]
    fn test_parse_self_closing_tags() {
        let html = r#"<html><body><br/><img src="test.jpg"/></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();

        let br_tags = tree.find_by_tag("br");
        let br = br_tags.first().unwrap();
        assert!(br.is_empty());

        let img_tags = tree.find_by_tag("img");
        let img = img_tags.first().unwrap();
        assert!(img.is_empty());
        assert_eq!(img.get_attribute("src"), Some("test.jpg"));
    }

    #[test]
    fn test_parse_mixed_content() {
        let html = r#"<html><body><p>Start<strong>Bold</strong>End</p></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();

        let p_tags = tree.find_by_tag("p");
        let p = p_tags.first().unwrap();
        assert_eq!(p.text, Some("Start".to_string()));
        assert_eq!(p.children.len(), 1);
        assert_eq!(p.children[0].tag, "strong");
        assert_eq!(p.children[0].text, Some("Bold".to_string()));
    }

    #[test]
    fn test_tree_to_html_roundtrip() {
        let html = r#"<html><body><div class="test">Before<p>Inside</p>After</div></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();
        let result = tree_to_html(&tree);

        // The result should be valid HTML (not necessarily identical due to formatting)
        assert!(result.contains("<html>"));
        assert!(result.contains("<body>"));
        assert!(result.contains("<div"));
        assert!(result.contains(r#"class="test""#));
        assert!(result.contains("Before"));
        assert!(result.contains("<p>"));
        assert!(result.contains("Inside"));
        assert!(result.contains("After"));
        assert!(result.contains("</div>"));
        assert!(result.contains("</body>"));
        assert!(result.contains("</html>"));
    }

    #[test]
    fn test_parse_without_html_tags() {
        let html = r#"<body><p>Direct content</p></body>"#;
        let tree = parse_html_to_tree(html).unwrap();

        assert_eq!(tree.root.tag, "html");

        let p_tags = tree.find_by_tag("p");
        assert_eq!(p_tags.len(), 1);
        assert_eq!(p_tags[0].text, Some("Direct content".to_string()));
    }

    #[test]
    fn test_parse_with_comments() {
        let html = r#"<html><body><!-- comment --><p>Content</p></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();

        // Comments should be ignored
        let p_tags = tree.find_by_tag("p");
        assert_eq!(p_tags.len(), 1);
        assert_eq!(p_tags[0].text, Some("Content".to_string()));
    }

    #[test]
    fn test_text_content_preservation() {
        let html = r#"<html><body><p>Paragraph 1</p><p>Paragraph 2</p></body></html>"#;
        let tree = parse_html_to_tree(html).unwrap();

        assert_eq!(tree.root.text_content(), "Paragraph 1Paragraph 2");
    }
}
