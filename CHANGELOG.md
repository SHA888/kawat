# Changelog

All notable changes to this project will be documented in this file.

## v0.1.3 - 2026-04-07

### Added
- **kawat-extract**: Complete content extraction pipeline mirroring trafilatura
  - `extract_content()`: Main wrapper orchestrating full extraction process
  - `_extract()`: BODY_XPATH iteration with first-match wins logic
  - `prune_unwanted_sections()`: Class filtering + link density calculation
  - `handle_textelem()`: Dispatcher for all HTML element types
  - `handle_titles()`: Markdown heading generation (h1-h6)
  - `handle_paragraphs()`: Complex paragraph processing with nested elements
  - `handle_formatting()`: Bold, italic, underline, span formatting
  - `handle_lists()`: Ordered/unordered lists with nested list support
  - `handle_quotes()` + `handle_code_blocks()`: Blockquotes and code processing
  - `handle_table()`: Markdown table generation with header separators
  - `handle_image()`: Image extraction with alt text
  - `handle_other_elements()`: Fallback handler for unknown elements
  - `recover_wild_text()`: Wild text recovery for short results
  - `baseline()`: Last-resort extraction fallback chain
  - `html2txt()`: HTML-to-plain text conversion
- **Security**: Markdown escaping to prevent injection attacks
- **Content Quality**: Link density calculation for better content pruning
- **Testing**: 38 comprehensive unit tests (all passing)
- **Code Quality**: Clippy warnings fixed, proper error handling

### Changed
- Enhanced content extraction with proper markdown formatting
- Improved nested element handling in paragraphs and lists
- Better text preservation in `html2txt()` function
- Optimized tree traversal and content processing

### Fixed
- Code block markdown formatting (missing closing backticks)
- Table markdown generation (missing header separators)
- Nested list handling with proper indentation
- Content pruning logic for unwanted sections
- Text content extraction with proper whitespace preservation

### Testing
- 38 kawat-extract unit tests (all passing)
- 76 total tests across all crates (all passing)
- Pre-commit hooks: Rust Format, Clippy, Cargo Audit (all passing)

## v0.1.2 - 2026-03-26

### Added
- **kawat-html**: Complete HTML tree cleaning and tag normalization pipeline
  - `tree_cleaning()`: Remove 44 MANUALLY_CLEANED tags and strip 20 MANUALLY_STRIPPED tags
  - `convert_tags()`: Normalize HTML tags to internal catalog (h1-h6→head, b/strong/em/i→hi, a→ref, ul/ol→list, li→item, br→lb, blockquote→quote, del/s→del, code/pre→code)
  - `convert_link()`: Resolve relative URLs against base_url using standards-compliant URL resolution
  - `_is_code_block()`: Distinguish between inline code and code blocks
  - `handle_textnode()` + `process_node()`: Text extraction and normalization for all element types
  - `link_density_test()` and `link_density_test_tables()`: Link density filtering for content extraction
  - `delete_by_link_density()`: Remove high-density link elements with backtracking
- **kawat-extract**: Custom KawatTree structure for lightweight HTML processing
  - `KawatNode` and `KawatTree` structs with full traversal and manipulation methods
  - HTML parsing with proper text/tail distinction
  - Integration with kawat-html transformations
  - 23 comprehensive unit tests

### Changed
- Improved HTML processing pipeline with immutable-first design
- Enhanced error handling with proper Result types throughout

### Fixed
- Lifetime syntax errors in tree.rs (explicit `'_` lifetime parameters)
- Test failures in convert_link and textnode modules
- Inline code formatting (missing closing backtick)

### Testing
- 34 kawat-html unit tests (all passing)
- 23 kawat-extract unit tests (all passing)
- 57 total tests across all crates (all passing)
- Pre-commit hooks: Rust Format, Clippy, Cargo Audit (all passing)

## v0.1.1 - 2026-03-24

### Added
- Comprehensive XPath evaluation engine with CSS selector fallback
- Benchmark suite for XPath performance testing
- Extensive test coverage (15+ tests) for all BODY_XPATH expressions
- Support for complex XPath patterns including `(article)[1]`

### Changed
- Updated dependencies: lru (0.12→0.16), quick-xml (0.37→0.39), scraper (0.22→0.26), criterion (0.5→0.8), reqwest (0.12→0.13)
- Improved TLS configuration in reqwest (rustls-tls → rustls)
- Enhanced error handling in XPath evaluation

### Fixed
- XPath fallback test failures for `(article)[1]` pattern
- CI/CD pipeline issues with deny.toml configuration
- Codecov upload failures for protected branches
- Security workflow permission handling

## v0.1.0 - 2026-03-22
- Initial workspace import of kawat crates (core, extract, html, xpath, metadata, output, CLI, etc.).
- Added date parsing regex fallback and chrono clock feature for htmldate-rs.
- Implemented `FromStr` for `OutputFormat` and hardened CLI format parsing.
- Resolved clippy warnings (unused imports/vars, format args) across crates.
- Added `.gitignore` and `.pre-commit-config.yaml` (fmt, clippy, cargo-audit) and ensured hooks pass.
