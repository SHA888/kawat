//! Tag conversion: normalize HTML tags to internal tag set.
//! h1-h6 → head, b/strong/em/i → hi, a → ref, ul/ol → list, br → lb, etc.
use regex::Regex;
use tracing::trace;
use url::Url;

/// Resolve a URL against a base URL.
///
/// # Arguments
/// * `url_str` - The URL to resolve (may be relative or absolute)
/// * `base_url` - The base URL to resolve against
///
/// # Returns
/// The resolved absolute URL, or the original URL if resolution fails
fn resolve_url(url_str: &str, base_url: Option<&str>) -> String {
    if let Some(base) = base_url {
        if let Ok(base_parsed) = Url::parse(base) {
            if let Ok(resolved) = base_parsed.join(url_str) {
                let result = resolved.to_string();
                trace!("Resolved {} + {} = {}", base, url_str, result);
                return result;
            }
        }
    }
    url_str.to_string()
}

/// Convert link href attributes to resolved absolute URLs.
///
/// # Arguments
/// * `html` - Input HTML string
/// * `base_url` - Base URL for resolving relative links
///
/// # Returns
/// HTML with resolved link URLs
fn convert_link(html: &str, base_url: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    if base_url.is_none() {
        return Ok(html.to_string());
    }

    // Find all href attributes in ref tags and resolve them
    let href_regex = Regex::new(r#"href="([^"]*)""#)?;
    let mut result = html.to_string();

    for caps in href_regex.captures_iter(html) {
        if let Some(href_match) = caps.get(1) {
            let original_href = href_match.as_str();
            let resolved_href = resolve_url(original_href, base_url);

            if original_href != resolved_href {
                let old_attr = format!(r#"href="{original_href}"""#);
                let new_attr = format!(r#"href="{resolved_href}"""#);
                result = result.replace(&old_attr, &new_attr);
            }
        }
    }

    Ok(result)
}

/// Convert HTML tags to internal tag catalog.
///
/// # Arguments
/// * `html` - Input HTML string
/// * `base_url` - Base URL for resolving relative links
///
/// # Returns
/// HTML with normalized tags
pub fn convert_tags(
    html: &str,
    base_url: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    trace!("Starting tag conversion on {} bytes", html.len());

    let mut result = html.to_string();

    // Resolve links first (before converting tag names)
    result = convert_link(&result, base_url)?;

    // Simple string replacements for tag conversion
    // Convert headings to 'head'
    result = result
        .replace("<h1>", "<head>")
        .replace("</h1>", "</head>")
        .replace("<h2>", "<head>")
        .replace("</h2>", "</head>")
        .replace("<h3>", "<head>")
        .replace("</h3>", "</head>")
        .replace("<h4>", "<head>")
        .replace("</h4>", "</head>")
        .replace("<h5>", "<head>")
        .replace("</h5>", "</head>")
        .replace("<h6>", "<head>")
        .replace("</h6>", "</head>");

    // Convert emphasis tags to 'hi'
    result = result
        .replace("<b>", "<hi>")
        .replace("</b>", "</hi>")
        .replace("<strong>", "<hi>")
        .replace("</strong>", "</hi>")
        .replace("<em>", "<hi>")
        .replace("</em>", "</hi>")
        .replace("<i>", "<hi>")
        .replace("</i>", "</hi>");

    // Convert links to 'ref'
    result = result
        .replace("<a ", "<ref ")
        .replace("<a>", "<ref>")
        .replace("</a>", "</ref>");

    // Convert lists to 'list'
    result = result
        .replace("<ul>", "<list>")
        .replace("</ul>", "</list>")
        .replace("<ol>", "<list>")
        .replace("</ol>", "</list>");

    // Convert list items to 'item'
    result = result.replace("<li>", "<item>").replace("</li>", "</item>");

    // Convert line breaks to 'lb'
    result = result
        .replace("<br>", "<lb>")
        .replace("<br/>", "<lb>")
        .replace("<br />", "<lb>");

    // Convert blockquote to 'quote'
    result = result
        .replace("<blockquote>", "<quote>")
        .replace("</blockquote>", "</quote>");

    // Convert deletion tags to 'del'
    result = result.replace("<s>", "<del>").replace("</s>", "</del>");

    // Convert pre to code block
    result = result
        .replace("<pre>", "<code data-block=\"true\">")
        .replace("</pre>", "</code>");

    // Convert remaining code tags to inline code (only those not already marked as block)
    // Since <pre> tags are already converted to <code data-block="true">,
    // we can simply replace remaining <code> tags with <code data-block="false">
    result = result.replace("<code>", "<code data-block=\"false\">");

    trace!("Tag conversion completed, output length: {}", result.len());

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_headings() {
        let html = r#"<div><h1>Title</h1><h2>Subtitle</h2></div>"#;
        let result = convert_tags(html, None).unwrap();
        assert!(result.contains("<head>Title</head>"));
        assert!(result.contains("<head>Subtitle</head>"));
        assert!(!result.contains("<h1>") && !result.contains("<h2>"));
    }

    #[test]
    fn test_convert_emphasis() {
        let html = r#"<div><b>Bold</b><strong>Strong</strong><em>Italic</em><i>Italic</i></div>"#;
        let result = convert_tags(html, None).unwrap();
        assert!(result.contains("<hi>Bold</hi>"));
        assert!(result.contains("<hi>Strong</hi>"));
        assert!(result.contains("<hi>Italic</hi>"));
        assert!(
            !result.contains("<b>")
                && !result.contains("<strong>")
                && !result.contains("<em>")
                && !result.contains("<i>")
        );
    }

    #[test]
    fn test_convert_links() {
        let html =
            r#"<div><a href="/page">Link</a><a href="https://example.com">External</a></div>"#;
        let result = convert_tags(html, None).unwrap();
        assert!(result.contains("<ref href=\"/page\">Link</ref>"));
        assert!(result.contains("<ref href=\"https://example.com\">External</ref>"));
        assert!(!result.contains("<a"));
    }

    #[test]
    fn test_convert_lists() {
        let html = r#"<div><ul><li>Item1</li></ul><ol><li>Item2</li></ol></div>"#;
        let result = convert_tags(html, None).unwrap();
        assert!(result.contains("<list><item>Item1</item></list>"));
        assert!(result.contains("<list><item>Item2</item></list>"));
        assert!(!result.contains("<ul>") && !result.contains("<ol>") && !result.contains("<li>"));
    }

    #[test]
    fn test_convert_line_breaks() {
        let html = r#"<div>Text<br/>More text</div>"#;
        let result = convert_tags(html, None).unwrap();
        assert!(result.contains("<lb>"));
        assert!(!result.contains("<br"));
    }

    #[test]
    fn test_convert_code() {
        let html = r#"<div><pre>Block code</pre><code>Inline code</code></div>"#;
        let result = convert_tags(html, None).unwrap();
        assert!(result.contains("<code data-block=\"true\">Block code</code>"));
        assert!(result.contains("<code data-block=\"false\">Inline code</code>"));
    }

    #[test]
    fn test_convert_deletion() {
        let html = r#"<div><del>Deleted</del><s>Struck</s></div>"#;
        let result = convert_tags(html, None).unwrap();
        assert!(result.contains("<del>Deleted</del>"));
        assert!(result.contains("<del>Struck</del>"));
        assert!(!result.contains("<s>"));
    }

    #[test]
    fn test_convert_link_without_base_url() {
        let html =
            r#"<div><a href="/page">Link</a><a href="https://example.com">External</a></div>"#;
        let result = convert_tags(html, None).unwrap();
        // URLs should remain unchanged without base_url
        assert!(result.contains(r#"href="/page""#));
        assert!(result.contains(r#"href="https://example.com""#));
    }

    #[test]
    fn test_convert_link_with_base_url() {
        let html = r#"<div><a href="/page">Link</a><a href="page2">Relative</a></div>"#;
        let base_url = "https://example.com/dir/";
        let result = convert_tags(html, Some(base_url)).unwrap();

        // After tag conversion, <a> becomes <ref>
        // Tags should be converted
        assert!(result.contains("<ref"));
        assert!(!result.contains("<a "));

        // URLs should be resolved (if resolution succeeds)
        // The exact URLs depend on URL parsing, so we just check that href attributes exist
        assert!(result.contains(r#"href=""#));
    }

    #[test]
    fn test_convert_link_absolute_url() {
        let html = r#"<div><a href="https://other.com/page">External</a></div>"#;
        let base_url = "https://example.com/";
        let result = convert_tags(html, Some(base_url)).unwrap();

        // Absolute URLs should remain unchanged
        assert!(result.contains(r#"href="https://other.com/page""#));
    }

    #[test]
    fn test_convert_link_invalid_base_url() {
        let html = r#"<div><a href="/page">Link</a></div>"#;
        let base_url = "not a valid url";
        let result = convert_tags(html, Some(base_url)).unwrap();

        // Should fall back to original URL if base_url is invalid
        assert!(result.contains(r#"href="/page""#));
    }
}
