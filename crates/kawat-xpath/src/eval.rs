//! XPath evaluation engine.

use crate::XpathError;

/// Evaluates XPath expressions against HTML documents.
pub struct XpathEngine;

impl XpathEngine {
    /// Evaluate an XPath expression on HTML content, returning matching text fragments.
    pub fn eval_text(html: &str, xpath: &str) -> Result<Vec<String>, XpathError> {
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

    /// Check if an XPath expression matches anything in the HTML.
    pub fn has_match(html: &str, xpath: &str) -> bool {
        Self::eval_text(html, xpath)
            .map(|results| !results.is_empty())
            .unwrap_or(false)
    }
}
