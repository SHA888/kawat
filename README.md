# kawat

[![CI](https://github.com/SHA888/kawat/workflows/CI/badge.svg)](https://github.com/SHA888/kawat/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/SHA888/kawat/branch/main/graph/badge.svg)](https://codecov.io/gh/SHA888/kawat)
[![Crates.io](https://img.shields.io/crates/v/kawat.svg)](https://crates.io/crates/kawat)
[![docs.rs](https://docs.rs/kawat/badge.svg)](https://docs.rs/kawat)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://github.com/SHA888/kawat/blob/main/LICENSE)
[![Benchmark](https://img.shields.io/badge/benchmark-passing-brightgreen.svg)](https://github.com/SHA888/kawat/actions/workflows/benchmark.yml)

A Rust library for web content extraction, inspired by [trafilatura](https://github.com/adbar/trafilatura).

*Kawat* is Indonesian for "wire" — the same metallurgical metaphor as *trafilatura* (Italian for "wire drawing"), symbolizing the refinement of raw HTML into clean, structured text.

## Features

- **Main text extraction** with complete trafilatura-compatible pipeline
- **Content extraction handlers** for all HTML elements (titles, paragraphs, lists, tables, quotes, code, images)
- **Markdown output** with proper formatting and escaping for security
- **Link density calculation** for intelligent content pruning
- **XPath evaluation** with BODY_XPATH expressions for content area detection
- **Metadata extraction**: title, author, date, categories, tags, license
- **Comment extraction** separated from main content
- **Date extraction** via `htmldate-rs` (standalone crate)
- **Deduplication** at sentence, paragraph, and document level
- **Multiple output formats**: TXT, Markdown, JSON, XML, XML-TEI, CSV, HTML
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

### Direct kawat-extract usage

```rust
use kawat_extract::{extract_content, parse_html_to_tree};

let html = std::fs::read_to_string("page.html").unwrap();
let tree = parse_html_to_tree(&html).unwrap();
let content = extract_content(&tree).unwrap();
println!("{}", content); // Markdown formatted content
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
| `kawat-extract` | **NEW: Complete content extraction pipeline** |
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
