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
- **CLI** for single URL or batch processing

## Extraction cascade

```
HTML → parse → metadata → clean → convert tags → extract comments
  → kawat sequence:
      extract_content (BODY_XPATH, first match)
      → if not fast: compare with readability + justext fallbacks
      → if still short: baseline (JSON-LD → <article> → <p> → body text)
  → size checks → dedup → language filter → output format
```

## Usage

```rust
use kawat::{extract, ExtractorOptions};

let html = std::fs::read_to_string("page.html").unwrap();
let text = extract(&html, &ExtractorOptions::default()).unwrap();
println!("{text}");
```

## CLI

```bash
# From URL
kawat -u "https://example.org/article"

# From file, JSON output with metadata
kawat -i page.html -f json --with-metadata

# From stdin
curl -s https://example.org | kawat
```

## Workspace structure

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
| `kawat-cli` | CLI binary |
| `htmldate-rs` | Standalone date extraction |

## Acknowledgments

This project is a Rust reimplementation inspired by [trafilatura](https://github.com/adbar/trafilatura) by Adrien Barbaresi. The extraction heuristics, XPath expressions, and cascade architecture are derived from trafilatura's published algorithms.

- Barbaresi, A. "Trafilatura: A Web Scraping Library and Command-Line Tool for Text Discovery and Extraction", Proceedings of ACL/IJCNLP 2021: System Demonstrations, 2021, p. 122-131.
- Barbaresi, A. "htmldate: A Python package to extract publication dates from web pages", JOSS 5(51), 2439, 2020.

## License

Apache-2.0
