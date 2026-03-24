//! CSS selector fallback for XPath evaluation.
//!
//! When `sxd_html` fails on malformed HTML, use `scraper` CSS selectors as fallback.
//! Translates XPath expressions that don't have CSS equivalents to custom filters.

use crate::XpathError;
use scraper::{Html, Selector};

/// CSS selector fallback engine for XPath expressions.
pub struct CssFallback;

impl CssFallback {
    /// Evaluate XPath expression using CSS selectors as fallback.
    /// Returns None if the XPath cannot be translated to CSS.
    pub fn eval_text(html: &str, xpath: &str) -> Option<Result<Vec<String>, XpathError>> {
        let selector = Self::xpath_to_css(xpath)?;

        let html = Html::parse_document(html);
        let elements: Vec<String> = html
            .select(&selector)
            .map(|el| el.text().collect())
            .collect();

        Some(Ok(elements))
    }

    /// Translate XPath expression to CSS selector.
    /// Returns None if translation is not possible.
    fn xpath_to_css(xpath: &str) -> Option<Selector> {
        // Handle specific BODY_XPATH expressions that have CSS equivalents
        match xpath {
            // Expression 2: generic <article>
            "(.//article)[1]" => Selector::parse("article").ok(),
            "(article)[1]" => Selector::parse("article").ok(),

            // Expression 5: main element fallback
            r#"(.//*[self::article or self::div or self::section][starts-with(@class, "main") or starts-with(@id, "main") or starts-with(@role, "main")])[1]|(.//main)[1]"# => {
                Selector::parse("main, [class^=\"main\"], [id^=\"main\"], [role^=\"main\"]").ok()
            }

            // Simple class-based selectors
            expr if expr.contains("@class=\"post\"") => Selector::parse(".post").ok(),
            expr if expr.contains("@id=\"content-main\"") => Selector::parse("#content-main").ok(),

            // For complex expressions with translate() or other functions, return None
            expr if expr.contains("translate(") => None,
            expr if expr.contains("contains(")
                && !expr.contains("@class=")
                && !expr.contains("@id=") =>
            {
                None
            }

            _ => None,
        }
    }

    /// Check if XPath expression can be handled by CSS selectors.
    #[allow(dead_code)]
    pub fn can_handle(xpath: &str) -> bool {
        Self::xpath_to_css(xpath).is_some()
    }
}

/// Custom filter for XPath expressions that don't have CSS equivalents.
pub struct CustomFilters;

impl CustomFilters {
    /// Apply custom filters that mimic XPath translate() and other functions.
    pub fn apply_filters(elements: Vec<scraper::ElementRef>) -> Vec<scraper::ElementRef> {
        elements
            .into_iter()
            .filter(|el| {
                // Mimic translate(@class, "B", "b") == "articlebody" logic
                let class = el.attr("class").unwrap_or("");
                let id = el.attr("id").unwrap_or("");

                // Check for articlebody in various cases
                Self::contains_case_insensitive(class, "articlebody")
                    || Self::contains_case_insensitive(id, "articlebody")
                    || Self::contains_case_insensitive(class, "article")
                    || Self::contains_case_insensitive(id, "article")
            })
            .collect()
    }

    /// Case-insensitive substring check.
    fn contains_case_insensitive(text: &str, pattern: &str) -> bool {
        text.to_lowercase().contains(&pattern.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xpath_to_css_simple() {
        assert!(CssFallback::can_handle("(article)[1]"));
        assert!(CssFallback::can_handle("(.//article)[1]"));
    }

    #[test]
    fn test_xpath_to_css_complex() {
        // Expressions with translate() cannot be handled by CSS
        assert!(!CssFallback::can_handle(
            r#"contains(translate(@class, "B", "b"), "articlebody")"#
        ));
        assert!(!CssFallback::can_handle(
            r#"contains(translate(@id, "CM","cm"), "main-content")"#
        ));
    }

    #[test]
    fn test_css_fallback_evaluation() {
        let html = r#"
            <html>
                <body>
                    <article>
                        <h1>Test Article</h1>
                        <p>Content here</p>
                    </article>
                </body>
            </html>
        "#;

        let result = CssFallback::eval_text(html, "(.//article)[1]");
        assert!(result.is_some());
        let matches = result.unwrap().unwrap();
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_custom_filters() {
        let html = r#"
            <html>
                <body>
                    <div class="articlebody">Content</div>
                    <div class="articlebody">Content 2</div>
                    <div class="other">Other content</div>
                </body>
            </html>
        "#;

        let html_doc = Html::parse_document(html);
        let elements: Vec<scraper::ElementRef> =
            html_doc.select(&Selector::parse("div").unwrap()).collect();

        let filtered = CustomFilters::apply_filters(elements);
        assert_eq!(filtered.len(), 2); // Only articlebody elements
    }
}
