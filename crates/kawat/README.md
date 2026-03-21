# kawat

A Rust library for web content extraction, inspired by [trafilatura](https://github.com/adbar/trafilatura).

*Kawat* is Indonesian for "wire" — the same metallurgical metaphor as *trafilatura* (Italian for "wire drawing"), symbolizing the refinement of raw HTML into clean, structured text.

## Features

- **Main text extraction** with multi-algorithm fallback cascade
- **Metadata extraction**: title, author, date, categories, tags, license
- **Comment extraction** separated from main content
- **Date extraction** via `htmldate-rs` (standalone crate)
- **Deduplication** at sentence, paragraph, and document level
- **Multiple output formats**: TXT, Markdown, JSON, XML, XML-TEI, CSV, HTML
- **XPath evaluation on HTML** via `sxd_html` + `sxd_xpath`
- **Language detection** (optional feature)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
kawat = "0.1"
```

## Usage

### Basic extraction

```rust
use kawat::{extract, ExtractorOptions};

let html = std::fs::read_to_string("page.html")?;
let text = extract(&html, &ExtractorOptions::default())?;
println!("{}", text);
```

### With metadata

```rust
use kawat::{bare_extraction, ExtractorOptions};

let html = std::fs::read_to_string("page.html")?;
let mut options = ExtractorOptions::default();
options.with_metadata = true;

let doc = bare_extraction(&html, &options)?;
println!("Title: {}", doc.metadata.title.unwrap_or_default());
println!("Author: {}", doc.metadata.author.unwrap_or_default());
println!("Date: {}", doc.metadata.date.unwrap_or_default());
println!("Body:\n{}", doc.body);
```

### Fetch from URL

```rust
use kawat::{fetch_url, extract, ExtractorOptions};

let html = fetch_url("https://example.org/article")?;
let text = extract(&html, &ExtractorOptions::default())?;
println!("{}", text);
```

### Async URL fetching

```rust
use kawat::{fetch_url_async, extract, ExtractorOptions};

let html = fetch_url_async("https://example.org/article").await?;
let text = extract(&html, &ExtractorOptions::default())?;
println!("{}", text);
```

## Extraction Cascade

The extraction process follows this pipeline:

```
HTML → parse → metadata → clean → convert tags → extract comments
  → kawat sequence:
      extract_content (BODY_XPATH, first match)
      → if not fast: compare with readability + justext fallbacks
      → if still short: baseline (JSON-LD → <article> → <p> → body text)
  → size checks → dedup → language filter → output format
```

## Configuration

```rust
use kawat::{ExtractorOptions, Focus, OutputFormat};

let options = ExtractorOptions {
    format: OutputFormat::Markdown,
    fast: false,                    // Use fallback algorithms
    focus: Focus::Balanced,         // Balanced precision/recall
    comments: true,                 // Extract comments
    formatting: true,               // Preserve formatting
    links: false,                   // Include links
    images: false,                  // Include images
    tables: true,                   // Include tables
    dedup: true,                    // Deduplicate content
    target_language: Some("en".to_string()),
    with_metadata: true,
    ..Default::default()
};

let text = kawat::extract(&html, &options)?;
```

## Features

- `language-detection`: Enable language filtering via the `lingua` crate

```toml
[dependencies]
kawat = { version = "0.1", features = ["language-detection"] }
```

## Workspace Structure

| Crate | Purpose |
|---|---|
| `kawat` | Public facade, re-exports |
| `kawat-core` | Extraction cascade orchestrator |
| `kawat-html` | Tree cleaning, tag normalization |
| `kawat-xpath` | XPath on HTML (sxd_html + sxd_xpath) |
| `kawat-extract` | Main content extractor |
| `kawat-readability` | Readability fallback (dom_smoothie) |
| `kawat-justext` | Pure Rust justext port |
| `kawat-metadata` | Title, author, OG, JSON-LD |
| `kawat-dedup` | Simhash + LRU deduplication |
| `kawat-output` | Format converters |
| `htmldate-rs` | Standalone date extraction |

## Acknowledgments

This project is a Rust reimplementation inspired by [trafilatura](https://github.com/adbar/trafilatura) by Adrien Barbaresi. The extraction heuristics, XPath expressions, and cascade architecture are derived from trafilatura's published algorithms.

- Barbaresi, A. "Trafilatura: A Web Scraping Library and Command-Line Tool for Text Discovery and Extraction", Proceedings of ACL/IJCNLP 2021: System Demonstrations, 2021, p. 122-131.
- Barbaresi, A. "htmldate: A Python package to extract publication dates from web pages", JOSS 5(51), 2439, 2020.

## License

Apache-2.0
