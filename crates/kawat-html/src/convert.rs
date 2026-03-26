//! Tag conversion: normalize HTML tags to internal tag set.
//! h1-h6 → head, b/strong/em/i → hi, a → ref, ul/ol → list, br → lb, etc.
use tracing::trace;

/// Convert HTML tags to internal tag catalog.
///
/// # Arguments
/// * `html` - Input HTML string
/// * `base_url` - Base URL for resolving relative links (not implemented yet)
///
/// # Returns
/// HTML with normalized tags
pub fn convert_tags(
    html: &str,
    _base_url: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    trace!("Starting tag conversion on {} bytes", html.len());

    let mut result = html.to_string();

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
}
