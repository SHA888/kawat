//! The extraction cascade.
//! Implements the exact sequence from trafilatura core.py:bare_extraction().

use crate::ExtractionError;
use crate::config::ExtractorOptions;
use crate::document::Document;

/// Run the full extraction cascade.
pub fn run(html: &str, options: &ExtractorOptions) -> Result<Document, ExtractionError> {
    // Step 1: Parse HTML
    let document = scraper::Html::parse_document(html);

    // Step 2: Quick language check (fast mode only)
    // TODO: check_html_lang()

    // Step 3: Extract metadata (TODO: wired in when metadata extraction lands)
    let metadata = kawat_metadata::DocumentMetadata::default();

    // Step 4: User-specified selector pruning
    // TODO: prune by options.prune_selectors

    // Step 5: Tree cleaning
    // TODO: kawat_html::tree_cleaning()

    // Step 6: Convert tags
    // TODO: kawat_html::convert_tags()

    // Step 7: Extract comments (then remove from tree)
    // TODO: kawat_extract::extract_comments()

    // Step 8: trafilatura_sequence
    //   8a: kawat_extract::extract_content()
    //   8b: if !fast → compare::compare_extraction()
    //   8c: if still short → baseline()
    let body = extract_body_text(&document);

    // Step 9: Size checks
    if body.len() < options.min_extracted_size {
        return Err(ExtractionError::TooShort(
            body.len(),
            options.min_extracted_size,
        ));
    }

    // Step 10: Dedup check
    // TODO: kawat_dedup

    // Step 11: Language filter
    // TODO: lingua (optional)

    // Step 12: Format output - handled by caller via Document::to_formatted_string

    Ok(Document {
        metadata,
        body,
        comments: None,
        raw_text: None,
        text: None,
    })
}

/// Extract body text from parsed HTML (minimal baseline implementation).
fn extract_body_text(document: &scraper::Html) -> String {
    let root = document.root_element();
    let text = root.text().collect::<Vec<_>>().join(" ");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
