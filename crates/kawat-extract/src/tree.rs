//! Custom tree structure for HTML processing.
//! Lightweight, trafilatura-optimized tree representation.

use std::collections::HashMap;

/// A node in the Kawat tree representation.
///
/// This structure mirrors trafilatura's requirements for HTML processing:
/// - tag: element tag name
/// - text: text content before the first child
/// - tail: text content after the closing tag
/// - attributes: element attributes
/// - children: child nodes
/// - parent_tag: parent tag name for context (optional)
#[derive(Debug, Clone, PartialEq)]
pub struct KawatNode {
    pub tag: String,
    pub text: Option<String>,
    pub tail: Option<String>,
    pub attributes: HashMap<String, String>,
    pub children: Vec<KawatNode>,
    pub parent_tag: Option<String>,
}

impl KawatNode {
    /// Create a new KawatNode with the given tag.
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            text: None,
            tail: None,
            attributes: HashMap::new(),
            children: Vec::new(),
            parent_tag: None,
        }
    }

    /// Create a new KawatNode with text content.
    pub fn with_text(tag: &str, text: &str) -> Self {
        let mut node = Self::new(tag);
        node.text = Some(text.to_string());
        node
    }

    /// Add a child node to this node.
    pub fn add_child(&mut self, child: KawatNode) {
        self.children.push(child);
    }

    /// Remove a child node at the given index.
    pub fn remove_child(&mut self, index: usize) -> Option<KawatNode> {
        if index < self.children.len() {
            Some(self.children.remove(index))
        } else {
            None
        }
    }

    /// Get all text content from this node and its descendants.
    ///
    /// Note: This does NOT include tail text, as tail belongs to the context
    /// of the child element, not the parent.
    pub fn text_content(&self) -> String {
        let mut result = String::new();

        // Add this node's text
        if let Some(text) = &self.text {
            result.push_str(text);
        }

        // Add children's text (including their tails)
        for child in &self.children {
            result.push_str(&child.text_content());
            // Include child's tail as it's part of the child's content
            if let Some(tail) = &child.tail {
                result.push_str(tail);
            }
        }

        result
    }

    /// Find all nodes with the given tag name.
    pub fn find_by_tag(&self, tag: &str) -> Vec<&KawatNode> {
        let mut result = Vec::new();

        if self.tag == tag {
            result.push(self);
        }

        for child in &self.children {
            result.extend(child.find_by_tag(tag));
        }

        result
    }

    /// Find all nodes with the given tag name (mutable version).
    ///
    /// Note: For complex tree mutations, consider rebuilding the tree or using
    /// a different approach. This method is intentionally not provided due to
    /// the complexity of safe mutable tree traversal in Rust.
    ///
    /// For mutations, use the immutable version and rebuild the tree with
    /// the desired changes.
    pub fn find_by_tag_mut(&mut self, _tag: &str) -> Vec<&mut KawatNode> {
        unimplemented!(
            "Mutable traversal is not provided. Use immutable traversal and rebuild the tree."
        )
    }

    /// Get an iterator over all descendants of this node.
    pub fn iter_descendants(&self) -> DescendantsIter<'_> {
        DescendantsIter::new(self)
    }

    /// Check if this node has the given attribute.
    pub fn has_attribute(&self, attr: &str) -> bool {
        self.attributes.contains_key(attr)
    }

    /// Get the value of the given attribute.
    pub fn get_attribute(&self, attr: &str) -> Option<&str> {
        self.attributes.get(attr).map(|s| s.as_str())
    }

    /// Set an attribute value.
    pub fn set_attribute(&mut self, attr: &str, value: &str) {
        self.attributes.insert(attr.to_string(), value.to_string());
    }

    /// Remove an attribute.
    pub fn remove_attribute(&mut self, attr: &str) -> Option<String> {
        self.attributes.remove(attr)
    }

    /// Check if this node is a text node (no children, has text content).
    pub fn is_text_node(&self) -> bool {
        self.children.is_empty() && self.text.is_some()
    }

    /// Check if this node is empty (no children, no text).
    pub fn is_empty(&self) -> bool {
        self.children.is_empty() && self.text.is_none() && self.tail.is_none()
    }
}

/// Iterator over all descendants of a KawatNode.
#[derive(Debug)]
pub struct DescendantsIter<'a> {
    stack: Vec<&'a KawatNode>,
}

impl<'a> DescendantsIter<'a> {
    fn new(root: &'a KawatNode) -> Self {
        let mut stack = Vec::new();
        // Add children in reverse order to process them in correct order
        for child in root.children.iter().rev() {
            stack.push(child);
        }
        Self { stack }
    }
}

impl<'a> Iterator for DescendantsIter<'a> {
    type Item = &'a KawatNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            // Add children in reverse order to process them in correct order
            for child in node.children.iter().rev() {
                self.stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    }
}

/// The main tree structure representing an HTML document.
#[derive(Debug, Clone, PartialEq)]
pub struct KawatTree {
    pub root: KawatNode,
    pub body: Option<Box<KawatNode>>,
}

impl KawatTree {
    /// Create a new KawatTree with the given root node.
    pub fn new(root: KawatNode) -> Self {
        Self { root, body: None }
    }

    /// Get the body element if it exists.
    pub fn get_body(&self) -> Option<&KawatNode> {
        self.body
            .as_ref()
            .map(|b| b.as_ref())
            .or_else(|| self.root.find_by_tag("body").first().copied())
    }

    /// Get the body element if it exists (mutable version).
    pub fn get_body_mut(&mut self) -> Option<&mut KawatNode> {
        if self.body.is_none() {
            // Find body using immutable traversal and clone it
            if let Some(body) = self.root.find_by_tag("body").first() {
                self.body = Some(Box::new(KawatNode {
                    tag: body.tag.clone(),
                    text: body.text.clone(),
                    tail: body.tail.clone(),
                    attributes: body.attributes.clone(),
                    children: body.children.clone(),
                    parent_tag: body.parent_tag.clone(),
                }));
            }
        }
        self.body.as_mut().map(|b| b.as_mut())
    }

    /// Find all nodes with the given tag name.
    pub fn find_by_tag(&self, tag: &str) -> Vec<&KawatNode> {
        self.root.find_by_tag(tag)
    }

    /// Find all nodes with the given tag name (mutable version).
    ///
    /// Note: For complex tree mutations, consider rebuilding the tree or using
    /// a different approach. This method is intentionally not provided due to
    /// the complexity of safe mutable tree traversal in Rust.
    ///
    /// For mutations, use the immutable version and rebuild the tree with
    /// the desired changes.
    pub fn find_by_tag_mut(&mut self, _tag: &str) -> Vec<&mut KawatNode> {
        unimplemented!(
            "Mutable traversal is not provided. Use immutable traversal and rebuild the tree."
        )
    }

    /// Get an iterator over all descendants of the root.
    pub fn iter_descendants(&self) -> DescendantsIter<'_> {
        self.root.iter_descendants()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kawat_node_creation() {
        let node = KawatNode::new("div");
        assert_eq!(node.tag, "div");
        assert!(node.text.is_none());
        assert!(node.tail.is_none());
        assert!(node.children.is_empty());
        assert!(node.attributes.is_empty());
    }

    #[test]
    fn test_kawat_node_with_text() {
        let node = KawatNode::with_text("p", "Hello world");
        assert_eq!(node.tag, "p");
        assert_eq!(node.text, Some("Hello world".to_string()));
    }

    #[test]
    fn test_add_child() {
        let mut parent = KawatNode::new("div");
        let child = KawatNode::with_text("p", "Hello");

        parent.add_child(child);

        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].tag, "p");
        assert_eq!(parent.children[0].text, Some("Hello".to_string()));
    }

    #[test]
    fn test_remove_child() {
        let mut parent = KawatNode::new("div");
        let child1 = KawatNode::with_text("p", "Hello");
        let child2 = KawatNode::with_text("span", "World");

        parent.add_child(child1);
        parent.add_child(child2);

        let removed = parent.remove_child(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().tag, "p");
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].tag, "span");
    }

    #[test]
    fn test_text_content() {
        let mut parent = KawatNode::with_text("div", "Start ");
        let mut child = KawatNode::with_text("p", "Hello ");
        let grandchild = KawatNode::with_text("span", "World");

        child.add_child(grandchild);
        child.tail = Some(" End".to_string());
        parent.add_child(child);
        parent.tail = Some(" Tail".to_string());

        // text_content includes child's tail but not parent's tail
        assert_eq!(parent.text_content(), "Start Hello World End");
    }

    #[test]
    fn test_find_by_tag() {
        let mut root = KawatNode::new("html");
        let mut body = KawatNode::new("body");
        let mut div = KawatNode::new("div");
        let p1 = KawatNode::with_text("p", "Paragraph 1");
        let p2 = KawatNode::with_text("p", "Paragraph 2");

        div.add_child(p1);
        div.add_child(p2);
        body.add_child(div);
        root.add_child(body);

        let paragraphs = root.find_by_tag("p");
        assert_eq!(paragraphs.len(), 2);
        assert_eq!(paragraphs[0].text, Some("Paragraph 1".to_string()));
        assert_eq!(paragraphs[1].text, Some("Paragraph 2".to_string()));
    }

    #[test]
    fn test_attributes() {
        let mut node = KawatNode::new("a");

        assert!(!node.has_attribute("href"));
        assert_eq!(node.get_attribute("href"), None);

        node.set_attribute("href", "https://example.com");
        node.set_attribute("class", "link");

        assert!(node.has_attribute("href"));
        assert_eq!(node.get_attribute("href"), Some("https://example.com"));
        assert_eq!(node.get_attribute("class"), Some("link"));

        let removed = node.remove_attribute("href");
        assert_eq!(removed, Some("https://example.com".to_string()));
        assert!(!node.has_attribute("href"));
        assert!(node.has_attribute("class"));
    }

    #[test]
    fn test_is_text_node() {
        let text_node = KawatNode::with_text("span", "Hello");
        assert!(text_node.is_text_node());

        let mut element = KawatNode::new("div");
        let child = KawatNode::with_text("p", "Hello");
        element.add_child(child);
        assert!(!element.is_text_node());

        let empty = KawatNode::new("br");
        assert!(!empty.is_text_node());
    }

    #[test]
    fn test_is_empty() {
        let empty = KawatNode::new("br");
        assert!(empty.is_empty());

        let with_text = KawatNode::with_text("p", "Hello");
        assert!(!with_text.is_empty());

        let mut with_tail = KawatNode::new("div");
        with_tail.tail = Some("tail".to_string());
        assert!(!with_tail.is_empty());

        let mut with_child = KawatNode::new("div");
        with_child.add_child(KawatNode::new("span"));
        assert!(!with_child.is_empty());
    }

    #[test]
    fn test_kawat_tree() {
        let mut root = KawatNode::new("html");
        let body = KawatNode::new("body");
        root.add_child(body);

        let tree = KawatTree::new(root);

        assert_eq!(tree.root.tag, "html");
        assert!(tree.body.is_none());

        // Should find body even when body field is None
        let found_body = tree.get_body();
        assert!(found_body.is_some());
        assert_eq!(found_body.unwrap().tag, "body");
    }

    #[test]
    fn test_descendants_iterator() {
        let mut root = KawatNode::new("html");
        let mut body = KawatNode::new("body");
        let mut div = KawatNode::new("div");
        let p1 = KawatNode::with_text("p", "First");
        let p2 = KawatNode::with_text("p", "Second");

        div.add_child(p1);
        div.add_child(p2);
        body.add_child(div);
        root.add_child(body);

        let tree = KawatTree::new(root);

        let tags: Vec<&str> = tree.iter_descendants().map(|n| n.tag.as_str()).collect();
        assert_eq!(tags, vec!["body", "div", "p", "p"]);
    }
}
