# htmldate-rs

Fast and robust date extraction from web pages.

A Rust port of the Python [htmldate](https://github.com/adbar/htmldate) library by Adrien Barbaresi.

## Features

- **Three-tier extraction cascade**: metadata → structural HTML → text heuristics
- **JSON-LD & Open Graph support**: Extracts dates from schema.org and OG meta tags
- **Structural markers**: `<time>`, `<abbr>`, date-related class/id attributes
- **Text heuristics**: Regex-based date extraction from bare text content
- **Configurable date ranges**: Min/max date validation for plausible web content dates
- **No unsafe code**: Workspace-level `#[deny(unsafe_code)]`

## Usage

```rust
use htmldate_rs::{find_date, DateOptions};

let html = r#"<html><head>
  <meta property="article:published_time" content="2024-01-15T10:00:00Z">
</head><body>Article text</body></html>"#;

let date = find_date(html, &DateOptions::default());
assert_eq!(date, Some("2024-01-15".to_string()));
```

## License

Apache-2.0
