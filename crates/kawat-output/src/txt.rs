//! txt format output.
//! Mirrors trafilatura xml.py:xmltotxt() - minimal version just joins text nodes.

/// Convert document to plain text format.
///
/// Minimal implementation: joins text nodes with a simple header.
/// Mirrors trafilatura's xmltotxt() without full XML processing.
pub fn to_txt(
    title: Option<&str>,
    author: Option<&str>,
    date: Option<&str>,
    url: Option<&str>,
    body: &str,
) -> String {
    let mut output = String::with_capacity(body.len() + 256);

    // Header section (mirrors trafilatura header format)
    if let Some(t) = title {
        output.push_str(t);
        output.push('\n');
    }
    if let Some(a) = author {
        output.push_str(a);
        output.push('\n');
    }
    if let Some(d) = date {
        output.push_str(d);
        output.push('\n');
    }
    if let Some(u) = url {
        output.push_str(u);
        output.push('\n');
    }

    // Separator between header and body
    if !output.is_empty() {
        output.push('\n');
    }

    // Body: just the joined text nodes (minimal implementation)
    output.push_str(body);

    output
}

/// Extract text without any header - raw text only.
/// Strips trailing whitespace to normalize output.
pub fn to_txt_body_only(body: &str) -> String {
    body.trim_end().to_string()
}
