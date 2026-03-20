#[derive(Debug, thiserror::Error)]
pub enum XpathError {
    #[error("failed to parse HTML for XPath evaluation")]
    HtmlParseError,
    #[error("XPath compilation error: {0}")]
    CompileError(String),
    #[error("XPath evaluation error: {0}")]
    EvalError(String),
    #[error("no results for expression")]
    NoResults,
}
