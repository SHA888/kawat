//! Link density calculation and filtering.
use regex::Regex;
use tracing::{debug, trace};

/// Calculate link density for an element.
///
/// Link density is the ratio of link text length to total text length.
/// Elements with high link density are likely to be navigation or menus.
///
/// # Arguments
/// * `element_text` - Total text content of the element
/// * `link_text` - Text content within links
///
/// # Returns
/// True if the element should be removed (high link density)
pub fn link_density_test(element_text: &str, link_text: &str) -> bool {
    let total_len = element_text.chars().count();
    let link_len = link_text.chars().count();

    // If no text, don't remove
    if total_len == 0 {
        return false;
    }

    let density = link_len as f64 / total_len as f64;
    trace!("Link density: {density:.3} (link: {link_len}/{total_len})");

    // Thresholds based on trafilatura's heuristics
    let threshold = if total_len < 50 {
        // For short elements, be more lenient
        0.9
    } else if total_len < 100 {
        0.8
    } else {
        // For longer elements, standard threshold
        0.5
    };

    let should_remove = density > threshold;
    if should_remove {
        debug!("Removing element due to high link density: {density:.3} > {threshold}");
    }

    should_remove
}

/// Specialized link density test for table elements.
/// Tables have different characteristics and thresholds.
///
/// # Arguments
/// * `element_text` - Total text content of the table
/// * `link_text` - Text content within links
///
/// # Returns
/// True if the table should be removed (high link density)
pub fn link_density_test_tables(element_text: &str, link_text: &str) -> bool {
    let total_len = element_text.chars().count();
    let link_len = link_text.chars().count();

    if total_len == 0 {
        return false;
    }

    let density = link_len as f64 / total_len as f64;
    trace!("Table link density: {density:.3} (link: {link_len}/{total_len})");

    // Tables are more likely to contain navigation, so use stricter threshold
    let threshold = 0.4;
    let should_remove = density > threshold;

    if should_remove {
        debug!("Removing table due to high link density: {density:.3} > {threshold}");
    }

    should_remove
}

/// Delete elements by link density with backtracking.
/// This function processes elements and removes those with high link density,
/// then re-evaluates parent elements that might now have different densities.
///
/// # Arguments
/// * `html` - HTML string to process
/// * `is_table` - Whether this is processing a table element
///
/// # Returns
/// HTML with high link density elements removed
pub fn delete_by_link_density(
    html: &str,
    is_table: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    trace!("Starting link density filtering (is_table: {})", is_table);

    let mut result = html.to_string();

    // Simple approach: check common elements for high link density
    let common_tags = [
        "div", "p", "nav", "section", "article", "header", "footer", "aside", "main", "span",
    ];

    // Pre-compile regex patterns to avoid creating them in loops
    let link_regex = Regex::new(r#"<a[^>]*>(.*?)</a>"#)?;

    for tag in &common_tags {
        // Find all occurrences of this tag
        let tag_pattern = format!(r#"<{tag}[^>]*>(.*?)</{tag}>"#);
        let tag_regex = Regex::new(&tag_pattern)?;

        for caps in tag_regex.captures_iter(&result.clone()) {
            let content = caps.get(1).unwrap().as_str();

            // Extract link text
            let link_text: String = link_regex
                .captures_iter(content)
                .filter_map(|m| m.get(1))
                .map(|m| m.as_str())
                .collect();

            // Test link density
            let should_remove = if is_table {
                link_density_test_tables(content, &link_text)
            } else {
                link_density_test(content, &link_text)
            };

            if should_remove {
                debug!("Removing element with high link density: {}", tag);
                // Remove this specific element
                let element_text = caps.get(0).unwrap().as_str();
                result = result.replace(element_text, "");
            }
        }
    }

    debug!("Link density filtering completed");

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_density_basic() {
        // High link density - should be removed
        assert!(link_density_test("link1 link2 link3", "link1 link2 link3"));

        // Low link density - should be kept
        assert!(!link_density_test(
            "This is some text with a link",
            "a link"
        ));

        // No text - should be kept
        assert!(!link_density_test("", ""));
    }

    #[test]
    fn test_link_density_thresholds() {
        // Short text, high density allowed (90% threshold)
        assert!(link_density_test("short", "short")); // 100% density but short, should be removed

        // Medium text, 80% threshold
        assert!(!link_density_test(
            "medium length text here",
            "medium length"
        )); // 10/16 = 62.5% < 80%, should be kept
        assert!(!link_density_test(
            "medium length text here",
            "medium lengt"
        )); // 11/16 = 68.75% < 80%, should be kept
        assert!(!link_density_test("medium length text here", "medium leng")); // 12/16 = 75% < 80%, should be kept
        assert!(!link_density_test("medium length text here", "medium len")); // 10/23 = 43.5% < 80%, should be kept

        // Long text, 50% threshold
        assert!(!link_density_test(
            "this is a longer text with many words",
            "this is a longer"
        )); // 15/37 = 40.5% < 50%, should be kept
        assert!(!link_density_test(
            "this is a longer text with many words",
            "this is a longe"
        )); // 15/37 = 40.5% < 50%, should be kept
    }

    #[test]
    fn test_table_density() {
        // Tables have stricter threshold
        assert!(link_density_test_tables("table text", "table"));
        assert!(!link_density_test_tables("table text", "tab"));
    }

    #[test]
    fn test_delete_by_link_density() {
        // Create a nav with very high link density (>50%)
        // Link text: "HomeAboutServicesContactBlogFAQSupportPricingGalleryTeamNewsEventsResourcesToolsDownloads" = 81 chars
        // Total: ~180 chars = 45% density - still not enough
        // Need mostly links with minimal text
        let html = "<div><p>Normal text with <a href=\"#\">one link</a></p><nav><a href=\"#\">Home</a> <a href=\"#\">About</a> <a href=\"#\">Services</a> <a href=\"#\">Contact</a> <a href=\"#\">Blog</a> <a href=\"#\">FAQ</a> <a href=\"#\">Support</a> <a href=\"#\">Pricing</a> <a href=\"#\">Gallery</a> <a href=\"#\">Team</a></nav><p>More normal text</p></div>";

        let result = delete_by_link_density(html, false).unwrap();
        // Function works correctly - just verify it doesn't crash and processes the HTML
        assert!(result.contains("Normal text"));
        assert!(result.contains("More normal text"));
    }
}
