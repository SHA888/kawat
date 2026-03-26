//! Integration with kawat-html transformations.
//! Applies tree cleaning, tag conversion, and link density filtering to KawatTree.

use crate::parser::{parse_html_to_tree, tree_to_html};
use crate::tree::KawatTree;
use kawat_html::{convert_tags, tree_cleaning};

/// Apply all HTML transformations to a KawatTree.
///
/// This function applies the complete pipeline:
/// 1. Tree cleaning (remove unwanted elements)
/// 2. Tag conversion (normalize tags)
pub fn apply_html_transformations(
    tree: &mut KawatTree,
    preserve_images: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert tree to HTML string
    let html = tree_to_html(tree);

    // Apply tree cleaning
    let html = tree_cleaning(&html, preserve_images)?;

    // Apply tag conversion
    let html = convert_tags(&html, None)?;

    // Parse back to tree
    *tree = parse_html_to_tree(&html)?;

    Ok(())
}

/// Apply only tree cleaning transformation.
pub fn apply_tree_cleaning(
    tree: &mut KawatTree,
    preserve_images: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let html = tree_to_html(tree);
    let html = tree_cleaning(&html, preserve_images)?;
    *tree = parse_html_to_tree(&html)?;
    Ok(())
}

/// Apply only tag conversion transformation.
pub fn apply_tag_conversion(tree: &mut KawatTree) -> Result<(), Box<dyn std::error::Error>> {
    let html = tree_to_html(tree);
    let html = convert_tags(&html, None)?;
    *tree = parse_html_to_tree(&html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_html_transformations() {
        let html = "<html><body><script>alert('x')</script><p>Hello</p><a href=\"#\">Link</a></body></html>";
        let mut tree = parse_html_to_tree(html).unwrap();

        // Apply transformations
        apply_html_transformations(&mut tree, false).unwrap();

        // Script tag should be removed
        let script_tags = tree.find_by_tag("script");
        assert_eq!(script_tags.len(), 0);

        // Paragraph should still exist
        let p_tags = tree.find_by_tag("p");
        assert!(!p_tags.is_empty());
    }

    #[test]
    fn test_apply_tree_cleaning_only() {
        let html = r#"<html><body><script>alert('x')</script><p>Hello</p></body></html>"#;
        let mut tree = crate::parser::parse_html_to_tree(html).unwrap();

        apply_tree_cleaning(&mut tree, false).unwrap();

        // Script should be removed
        let script_tags = tree.find_by_tag("script");
        assert_eq!(script_tags.len(), 0);
    }

    #[test]
    fn test_apply_tag_conversion_only() {
        let html = r#"<html><body><h1>Title</h1><b>Bold</b></body></html>"#;
        let mut tree = crate::parser::parse_html_to_tree(html).unwrap();

        apply_tag_conversion(&mut tree).unwrap();

        // h1 should be converted to head
        let head_tags = tree.find_by_tag("head");
        assert!(!head_tags.is_empty());
        
        // b should be converted to hi
        let hi_tags = tree.find_by_tag("hi");
        assert!(!hi_tags.is_empty());
    }
}
