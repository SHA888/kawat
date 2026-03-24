//! XPath evaluation on HTML documents.
//!
//! Provides a wrapper around `sxd_html` + `sxd_xpath` for evaluating
//! XPath 1.0 expressions on HTML, with `scraper` as CSS-selector fallback.
//!
//! All XPath expressions from trafilatura's `xpaths.py` use a narrow subset:
//! `contains()`, `starts-with()`, `translate()`, `self::`, `[1]` positional,
//! `or`/`and`, and attribute tests. No axes beyond descendant/child.

mod compiled;
mod error;
mod eval;
mod fallback;

pub use compiled::CompiledXpaths;
pub use error::XpathError;
pub use eval::XpathEngine;

/// Pre-compiled XPath expression group (thread-safe, reusable).
pub struct XpathExpr {
    pub raw: &'static str,
    // Evaluation is handled by the engine at runtime
}

impl XpathExpr {
    pub const fn new(raw: &'static str) -> Self {
        Self { raw }
    }
}
