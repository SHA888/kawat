# kawat-xpath

XPath evaluation engine for HTML documents in the kawat web content extraction library.

Provides a wrapper around `sxd_html` + `sxd_xpath` for evaluating XPath 1.0 expressions on HTML.

## Features

- **XPath 1.0 support**: Full XPath 1.0 expression evaluation on HTML
- **Pre-compiled expressions**: Curated XPath expressions from trafilatura's xpaths.py
- **CSS selector fallback**: Alternative CSS selector evaluation path
- **No unsafe code**: Safe Rust implementation

## License

Apache-2.0
