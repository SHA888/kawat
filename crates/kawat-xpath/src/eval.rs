//! XPath evaluation engine.

use crate::{
    XpathError,
    fallback::{CssFallback, CustomFilters},
};

/// Evaluates XPath expressions against HTML documents.
pub struct XpathEngine;

impl XpathEngine {
    /// Evaluate an XPath expression on HTML content, returning matching text fragments.
    pub fn eval_text(html: &str, xpath: &str) -> Result<Vec<String>, XpathError> {
        // Try sxd_html first
        match Self::eval_text_sxd(html, xpath) {
            Ok(results) => Ok(results),
            Err(XpathError::HtmlParseError) => {
                // Fallback to CSS selectors if HTML parsing fails
                Self::eval_text_fallback(html, xpath)
            }
            Err(e) => Err(e),
        }
    }

    /// Evaluate using sxd_html (primary method).
    fn eval_text_sxd(html: &str, xpath: &str) -> Result<Vec<String>, XpathError> {
        let package = sxd_html::parse_html(html);
        let document = package.as_document();

        let factory = sxd_xpath::Factory::new();
        let expression = factory
            .build(xpath)
            .map_err(|e| XpathError::CompileError(format!("{e:?}")))?
            .ok_or_else(|| XpathError::CompileError("empty expression".into()))?;

        let context = sxd_xpath::Context::new();
        let value = expression
            .evaluate(&context, document.root())
            .map_err(|e| XpathError::EvalError(format!("{e:?}")))?;

        match value {
            sxd_xpath::Value::Nodeset(nodes) => Ok(nodes
                .document_order()
                .iter()
                .map(|n| n.string_value())
                .collect()),
            sxd_xpath::Value::String(s) => Ok(vec![s]),
            _ => Ok(vec![value.string()]),
        }
    }

    /// Fallback evaluation using CSS selectors.
    fn eval_text_fallback(html: &str, xpath: &str) -> Result<Vec<String>, XpathError> {
        // Try CSS selector fallback first
        if let Some(result) = CssFallback::eval_text(html, xpath) {
            return result;
        }

        // If CSS fallback isn't available, try custom filters for translate() expressions
        if xpath.contains("translate(") {
            return Self::eval_text_with_custom_filters(html, xpath);
        }

        Err(XpathError::EvalError(
            "XPath cannot be translated to CSS selector".into(),
        ))
    }

    /// Evaluate using custom filters for complex XPath expressions.
    fn eval_text_with_custom_filters(html: &str, xpath: &str) -> Result<Vec<String>, XpathError> {
        // For translate() expressions, extract the base selector and apply custom filters
        let base_selector = Self::extract_base_selector(xpath)?;

        let html_doc = scraper::Html::parse_document(html);
        let selector = scraper::Selector::parse(base_selector)
            .map_err(|e| XpathError::CompileError(format!("CSS selector error: {e}")))?;

        let elements: Vec<scraper::ElementRef> = html_doc.select(&selector).collect();
        let filtered = CustomFilters::apply_filters(elements);

        let results: Vec<String> = filtered.iter().map(|el| el.text().collect()).collect();

        Ok(results)
    }

    /// Extract base CSS selector from complex XPath expression.
    fn extract_base_selector(xpath: &str) -> Result<&str, XpathError> {
        // Simple extraction for common patterns
        if xpath.contains("self::article") {
            return Ok("article");
        }
        if xpath.contains("self::div") {
            return Ok("div");
        }
        if xpath.contains("self::section") {
            return Ok("section");
        }

        Err(XpathError::CompileError(
            "Cannot extract base selector from XPath".into(),
        ))
    }

    /// Check if an XPath expression matches anything in the HTML.
    pub fn has_match(html: &str, xpath: &str) -> bool {
        Self::eval_text(html, xpath)
            .map(|results| !results.is_empty())
            .unwrap_or(false)
    }
}
