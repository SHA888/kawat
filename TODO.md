# TODO.md — kawat development roadmap

> Cross-references [ARCHITECTURE.md](./ARCHITECTURE.md) throughout.
> Complexity estimates are in Rust LOC based on Python source analysis.

---

## Release plan

| Version | Phase | Milestone | Target |
|---|---|---|---|
| — | Phase 0 | Infrastructure & CI/CD | CI/CD workflows, quality gates, deployment |
| `0.1.0` | Phase 1 | Minimum viable extraction | Core extraction without fallbacks |
| `0.2.0` | Phase 2 | Fallback cascade | Readability + justext + baseline |
| `0.3.0` | Phase 3 | Metadata + date extraction | Full metadata, htmldate-rs v0.1.0 |
| `0.4.0` | Phase 4 | Output formats | All 7 output formats |
| `0.5.0` | Phase 5 | Dedup + language | Deduplication, language filtering |
| `0.6.0` | Phase 6 | CLI + batch | Full CLI, stdin/file/URL, batch |
| `0.7.0` | Phase 7 | Discovery | Feeds, sitemaps, crawling |
| `1.0.0` | Phase 8 | Production release | Benchmark parity, fuzz-tested, documented |

---

## Phase 0 — Infrastructure & CI/CD (Ongoing)

**Goal:** Establish robust CI/CD workflows, quality gates, and deployment automation throughout the project lifecycle.

**Status:** 🔄 **IN PROGRESS**

### 0.1 CI/CD Workflows

- [x] **PR validation workflow** (`.github/workflows/ci.yml`)
  - [x] Run on all PRs and pushes to main branch
  - [x] Jobs: format check (rust-fmt), lint (clippy), test (Linux/macOS/Windows), security audit
  - [x] Matrix testing across Rust stable/beta/nightly
  - [x] Cache cargo dependencies for faster builds
- [x] **Release workflow** (`.github/workflows/release.yml`)
  - [x] Triggered on version tags (v*)
  - [x] Build cross-platform binaries (x86_64/aarch64 for Linux/macOS/Windows)
  - [x] Create GitHub release with artifacts and checksums
  - [x] Publish to crates.io with proper dependency ordering
  - [x] Rate limiting handling for crates.io API
- [x] **Benchmark workflow** (`.github/workflows/benchmark.yml`)
  - [x] Run criterion benchmarks on schedule (weekly) and on benchmark changes
  - [x] Track performance regressions over time with custom storage
  - [x] Store results as artifacts and upload to benchmark service
  - [x] Generate performance reports and trend analysis
- [ ] **Security audit workflow** (`.github/workflows/security.yml`)
  - [ ] Daily cargo-audit runs with failure notifications
  - [x] Dependabot configuration for automated dependency updates
  - [ ] RUSTSEC advisory monitoring and automated PRs
  - [x] License compliance checking with cargo-deny

### 0.2 Quality Gates

- [ ] Require CI passing before merge (branch protection rules)
- [ ] Code coverage tracking with codecov
  - [ ] Target: ≥70% coverage for Phase 1
  - [ ] Target: ≥80% coverage for v1.0
  - [ ] Coverage badge in README
- [ ] Automated changelog generation using git-cliff
  - [ ] Conventional commit enforcement
  - [ ] Auto-generate CHANGELOG.md on releases
- [ ] Version bumping automation
  - [ ] Semantic versioning based on conventional commits
  - [ ] Pre-release validation checks

### 0.3 Documentation & Deployment

- [ ] Auto-deploy docs to GitHub Pages on main branch updates
  - [ ] API documentation generation with cargo doc
  - [ ] Custom theme and search functionality
  - [ ] Versioned documentation for releases
- [ ] README badge integration
  - [ ] CI status badge
  - [ ] crates.io version badge
  - [ ] docs.rs documentation badge
  - [ ] codecov coverage badge
  - [ ] benchmark performance badge
- [ ] Demo site deployment (optional, Phase 4+)
  - [ ] Web interface for testing extraction
  - [ ] Live examples and playground
  - [ ] Performance comparison with trafilatura

### 0.4 Publishing Automation

- [ ] Automated crate publishing workflow
  - [ ] Dependency ordering validation
  - [ ] Pre-publish validation (tests, docs, manifest checks)
  - [ ] Dry-run mode for testing publish process
  - [ ] Rollback capability for failed publishes
- [ ] Release artifact management
  - [ ] Binary releases for multiple platforms
  - [ ] Checksum verification (SHA256)
  - [ ] Automatic GitHub release notes generation
- [ ] Rate limiting and error handling
  - [ ] crates.io API rate limit awareness
  - [ ] Exponential backoff for retries
  - [ ] Notification system for publish failures

### 0.5 Phase 0 Implementation Timeline

| Phase | CI/CD Tasks | Target |
|---|---|---|
| Phase 0 | Basic CI workflow (format, clippy, test) | Immediately |
| Phase 0 | Pre-commit hooks configuration | Immediately |
| Phase 1 | Code coverage integration (70% target) | v0.1.0 |
| Phase 2 | Benchmark workflow setup | v0.2.0 |
| Phase 4 | Documentation deployment | v0.4.0 |
| Phase 6 | Cross-compilation in CI | v0.6.0 |
| Phase 8 | Full CI/CD automation (80% coverage) | v1.0.0 |

---

## Phase 1 — Minimum viable extraction (`0.1.0`)

**Goal:** `kawat::extract(html, &default_options)` returns main text content for well-structured pages (blogs, articles). No fallbacks, no metadata.

**Status:** ✅ **RELEASED** (v0.1.0 published to crates.io)

### 1.1 kawat-xpath: XPath evaluation layer

- [x] Integrate `sxd_html` + `sxd_xpath` in `eval.rs`
  - [x] Wrapper around sxd parse + evaluate
- [x] Port all 50 XPath expressions from `xpaths.py` to `compiled.rs`
  - [x] Verbatim copy, 15 expression groups
- [x] Test each BODY_XPATH expression against 10+ real HTML pages
  - [x] Created comprehensive test suite with 15 tests covering all BODY_XPATH expressions
- [x] Test OVERALL_DISCARD_XPATH against same pages
  - [x] Added tests for footer, sidebar, navigation, comments, social sharing patterns
- [x] Fallback path: if `sxd_html` fails on malformed HTML, use `scraper` CSS selectors
  - [x] Implemented CSS fallback for simple expressions (article, main, post classes)
  - [x] Added custom filters for translate() expressions with case-insensitive matching
- [x] Benchmark: sxd_xpath eval time vs. scraper CSS on 100 pages
  - [x] Created criterion benchmark comparing XPath vs CSS selector performance
  - [x] Benchmarks show performance characteristics for decision making

### 1.2 kawat-html: tree cleaning + tag conversion

- [ ] `tree_cleaning()`: remove MANUALLY_CLEANED (44 tags) using scraper
  - [ ] Handle immutable `scraper::Html`; use `lol_html` for mutation or rebuild tree
- [ ] `tree_cleaning()`: strip MANUALLY_STRIPPED (20 tags) keeping text
- [ ] Handle table-in-figure edge case (change `<figure>` containing `<table>` to `<div>`)
- [ ] Preserve `<img>` tags when `options.images=true`
- [ ] `convert_tags()`: h1-h6→head, b/strong/em/i→hi, a→ref, ul/ol→list, li→item, br→lb, blockquote→quote, del/s→del, code/pre→code
  - [ ] Critical tag normalization logic
- [ ] `convert_tags()`: `_is_code_block()` detection (code vs. inline code)
- [ ] `convert_tags()`: `convert_link()` with base_url resolution
- [ ] `link_density_test()` for generic elements
- [ ] `link_density_test_tables()` specialized for tables
- [ ] `delete_by_link_density()` with backtracking
- [ ] `handle_textnode()` + `process_node()`
- [ ] **Decision: internal tree representation**
  - [ ] Options: (A) custom tree struct, (B) `markup5ever_rcdom`, (C) string-based XML
  - [ ] Recommendation: (A) Custom struct

### 1.3 kawat-extract: main content extraction

- [ ] Internal tree type definition (`KawatTree`, `KawatNode`)
  - [ ] Body, children, tag, text, tail, attributes
- [ ] `_extract()`: iterate BODY_XPATH, first match wins
- [ ] `prune_unwanted_sections()`: OVERALL_DISCARD + link density passes
- [ ] `handle_textelem()`: dispatcher by tag type
- [ ] `handle_titles()`
- [ ] `handle_paragraphs()` (most complex handler)
- [ ] `handle_formatting()`
- [ ] `handle_lists()`
- [ ] `handle_quotes()` + `handle_code_blocks()`
- [ ] `handle_table()` (includes cell type detection, nested content)
- [ ] `handle_image()`
- [ ] `handle_other_elements()`
- [ ] `recover_wild_text()`
- [ ] `extract_content()` wrapper
- [ ] `baseline()` last-resort extraction (JSON-LD→article→p→body)
- [ ] `html2txt()`

### 1.4 kawat-core: wire it together

- [x] `cascade::run()` steps 1, 5, 6, 8a only (no metadata, no fallbacks)
- [x] `ExtractorOptions` default config
- [x] `Document` struct
- [ ] TXT output (step 12, txt only)
  - [ ] Minimal: just join text nodes

### 1.5 kawat facade + basic test

- [ ] `kawat::extract()` returns plain text
- [ ] Golden test: 20 HTML files, compare TXT output with Python trafilatura
  - [ ] Acceptance: ≥70% match

### 1.6 Phase 1 release checklist

- [x] `cargo test` passes all unit tests
- [x] Golden test ≥70% match rate
- [x] `cargo clippy` clean
- [x] `cargo doc` builds
- [x] README with basic usage example
- [x] Tag `v0.1.0`
- [x] Publish to crates.io
- [ ] Set up basic CI workflow (format, clippy, test)
- [ ] Configure pre-commit hooks
- [ ] Code coverage integration (70% target) with codecov

---

## Phase 2 — Fallback cascade (`0.2.0`)

**Goal:** Add readability + justext fallbacks + comparison heuristic. Extraction quality should match trafilatura on ≥90% of golden corpus.

**Status:** 🔲 **PENDING**

### 2.1 kawat-readability: readability fallback

- [ ] Integrate `dom_smoothie` crate
  - [ ] Wrapper matching `try_readability()` interface
- [ ] Match parameters: `min_text_length=25`, `retry_length=250`
  - [ ] Verify dom_smoothie exposes these
- [ ] If dom_smoothie lacks parameter control: port readability_lxml.py directly
  - [ ] Fallback plan: scoring + candidate selection is ~400 LOC core
- [ ] `sanitize_tree()` post-processing for readability output

### 2.2 kawat-justext: paragraph classification

- [ ] `Paragraph` struct (text, word_count, link_density, stopword_density, is_heading, is_boilerplate, class)
- [ ] `ParagraphMaker::make_paragraphs(tree)` — extract paragraphs from HTML tree
  - [ ] Walk tree, split by block elements, accumulate text
- [ ] `classify_paragraphs()` — initial classification
  - [ ] Parameters: 50, 150, 0.1, 0.2, 0.25, true
- [ ] `revise_paragraph_classification()` — context-based reclassification
  - [ ] max_heading_distance=150
- [ ] Stoplists for 29 languages (embed as static data)
  - [ ] Encode from justext's stoplist files
  - [ ] Priority: en, id, de, fr, es
- [ ] Test: classify a known article, verify boilerplate paragraphs removed

### 2.3 kawat-core: comparison heuristic

- [ ] `compare_extraction()` — full decision tree
  - [ ] Port all 8 conditions exactly
- [ ] `justext_rescue()` — justext as second fallback
- [ ] SANITIZED_XPATH constant for triggering justext
  - [ ] `.//aside\|.//audio\|.//button\|...`
- [ ] Wire steps 8b + 8c into `cascade::run()`

### 2.4 Comment extraction

- [ ] `extract_comments()` using COMMENTS_XPATH
- [ ] `process_comments_node()`
- [ ] COMMENTS_DISCARD_XPATH pruning
- [ ] REMOVE_COMMENTS_XPATH (precision mode)
- [ ] Wire into cascade step 7

### 2.5 Phase 2 release checklist

- [ ] Golden test ≥90% match rate
- [ ] Readability fallback fires correctly on pages where main extraction fails
- [ ] justext fallback fires on pages with embedded noise (ads, widgets)
- [ ] Comment extraction produces output matching trafilatura
- [ ] Tag `v0.2.0`
- [ ] Publish to crates.io
- [ ] Benchmark workflow setup with criterion + custom storage

---

## Phase 3 — Metadata + date extraction (`0.3.0`)

**Goal:** Full metadata extraction. `htmldate-rs` published as standalone crate.

**Status:** ✅ **RELEASED** (v0.1.0 published to crates.io)

### 3.1 htmldate-rs: date extraction

- [x] Tier 1 `meta.rs`: JSON-LD extraction
  - [x] `datePublished`, `dateModified`, `dateCreated`, `@graph` arrays
- [x] Tier 1 `meta.rs`: OG + standard meta tags (15 selectors)
  - [x] `article:published_time`, `dcterms.date`, etc.
- [x] Tier 2 `structural.rs`: `<time datetime="">` extraction
- [x] Tier 2 `structural.rs`: date class/id patterns (16 patterns)
- [x] Tier 3 `text.rs`: regex-based text scanning
  - [x] ISO, named month, dot/slash formats
- [ ] Tier 3 `text.rs`: disambiguation algorithm (extensive mode)
  - [ ] Position preference, context weighting, candidate ranking
- [x] `validators.rs`: 22 date format strings
- [x] `validators.rs`: plausibility checks (future date, min_date)
- [x] `patterns.rs`: compiled regex patterns
- [ ] Multilingual month names (at minimum: en, id, de, fr, es)
- [ ] Test against htmldate's own test corpus (1000 pages)
  - [ ] Target: ≥0.85 F-score (fast), ≥0.90 (extensive)
- [x] Publish `htmldate-rs` v0.1.0 to crates.io

### 3.2 kawat-metadata: full metadata extraction

- [ ] `examine_meta()`: scan all `<meta>` tags for name, property, itemprop
  - [ ] OG properties, Twitter cards, Dublin Core, itemprop
- [ ] `extract_opengraph()`: OG-specific extraction
  - [ ] og:title, og:description, og:site_name, og:image, og:type
- [ ] `extract_meta_json()` / `extract_json_ld()`: JSON-LD metadata
  - [ ] Schema.org types, `@graph` arrays, nested objects, author normalization
- [ ] `extract_title()`: title from TITLE_XPATHS → `<title>` tag → OG → JSON-LD
- [ ] `extract_author()`: author from AUTHOR_XPATHS + AUTHOR_DISCARD_XPATHS + JSON-LD + meta
- [ ] `extract_url()`: canonical URL from `<link rel="canonical">`, OG, meta
- [ ] `extract_sitename()`
- [ ] `extract_catstags()`: categories + tags from CATEGORIES_XPATHS, TAGS_XPATHS
- [ ] `extract_license()`: Creative Commons detection
- [ ] `Document.clean_and_trim()`: HTML entity unescape, length limit, whitespace normalization
- [ ] Integration: wire `extract_metadata()` into cascade step 3
- [ ] `only_with_metadata` filtering (require date + title + url)
- [ ] `url_blacklist` filtering

### 3.3 Phase 3 release checklist

- [ ] Metadata extraction matches trafilatura on ≥85% of golden corpus fields
- [ ] JSON-LD, OG, meta tag, and XPath-based extraction all functional
- [ ] Tag `v0.3.0`
- [ ] Publish to crates.io

---

## Phase 4 — Output formats (`0.4.0`)

**Goal:** All 7 output formats producing correct output.

**Status:** 🔲 **PENDING**

### 4.1 kawat-output: format converters

- [ ] `txt.rs`: `xmltotxt()` — convert internal tree to plain text
  - [ ] Join text nodes, handle whitespace
- [ ] `markdown.rs`: TXT with markdown formatting (headers, bold, italic, links, lists)
  - [ ] Triggered by `include_formatting` or format=markdown
- [ ] `json_output.rs`: `build_json_output()` with metadata fields
  - [ ] serde_json serialization
- [ ] `xml_output.rs`: `control_xml_output()` basic XML
  - [ ] quick-xml writer
- [ ] `xml_tei.rs`: TEI-XML with validation
  - [ ] Most complex format. TEI_VALID_TAGS, namespace handling
- [ ] `csv_output.rs`: `xmltocsv()` tab-separated
- [ ] `html_output.rs`: `build_html_output()`
- [ ] `determine_returnstring()` dispatcher
  - [ ] TXT/MD header with metadata in YAML front matter
- [ ] Unicode normalization (NFC)

### 4.2 Phase 4 release checklist

- [ ] All 7 formats produce output
- [ ] JSON output matches trafilatura's JSON on golden corpus
- [ ] XML well-formedness validated
- [ ] TEI-XML validates against DTD (stretch goal)
- [ ] Tag `v0.4.0`
- [ ] Publish to crates.io
- [ ] Documentation deployment to GitHub Pages
- [ ] Demo site deployment (optional)

---

## Phase 5 — Dedup + language detection (`0.5.0`)

**Goal:** Document-level deduplication, language filtering.

**Status:** 🔲 **PENDING**

### 5.1 kawat-dedup

- [x] `content_fingerprint()`: SHA-1 + Base64
- [x] `LRUCache` with configurable capacity (default 4096)
  - [x] Using `lru` crate
- [ ] `Simhash`: token sampling + weighted hash
  - [ ] Custom simhash, not external crate
- [ ] `sample_tokens()` + `sample_tokens_fallback()`
- [ ] `generate_bow_hash()`
- [ ] `duplicate_test()`: combine simhash + LRU
- [ ] `is_similar_domain()`
- [ ] Wire into cascade step 10
- [ ] `reset_caches()` for memory management

### 5.2 Language detection

- [ ] `check_html_lang()`: check `<html lang="">` attribute
- [ ] `language_filter()`: detect language of extracted text
  - [ ] Using `lingua` crate, behind feature flag
- [ ] Wire into cascade steps 2 + 11

### 5.3 Phase 5 release checklist

- [ ] Duplicate documents correctly rejected
- [ ] `reset_caches()` frees memory
- [ ] Language filtering works for ≥10 languages
- [ ] Tag `v0.5.0`
- [ ] Publish to crates.io

---

## Phase 6 — CLI + batch processing (`0.6.0`)

**Goal:** Full CLI with stdin/file/URL input, batch processing, output to files.

**Status:** 🔲 **PENDING**

### 6.1 kawat-cli

- [x] Clap argument parsing
- [ ] Single URL extraction (`-u`)
- [ ] File input (`-i`)
- [ ] Stdin piping
- [ ] Batch URL list (`--inputfile links.txt`)
- [ ] Output directory (`--outputdir`)
- [ ] HTML backup directory (`--backup-dir`)
- [ ] Parallel processing (tokio + configurable concurrency)
- [ ] Politeness: sleep between requests, user-agent, robots.txt respect
- [ ] Config file support (`--config-file`)
- [ ] `prune_xpath` CLI flag
- [ ] Progress output (stderr)

### 6.2 Phase 6 release checklist

- [ ] `kawat -u URL` works end to end
- [ ] `cat page.html | kawat` works
- [ ] Batch processing with `--inputfile`
- [ ] Parallel downloads with politeness
- [ ] Tag `v0.6.0`
- [ ] Publish to crates.io
- [ ] Cross-compilation in CI for multiple platforms

---

## Phase 7 — Discovery (`0.7.0`)

**Goal:** Feed, sitemap, and crawler support.

**Status:** 🔲 **PENDING**

### 7.1 Feed discovery

- [ ] `find_feed_urls()`: auto-discover RSS/Atom/JSON feeds from homepage
- [ ] Feed parsing (RSS, Atom, JSON Feed)
  - [ ] Consider `feed-rs` crate
- [ ] URL filtering by target language heuristic

### 7.2 Sitemap discovery

- [ ] `sitemap_search()`: discover and parse XML/TXT sitemaps
- [ ] Sitemap index support (nested sitemaps)
- [ ] URL deduplication across sitemaps

### 7.3 Web crawler

- [ ] `focused_crawler()`: BFS crawler with URL frontier
- [ ] `max_seen_urls`, `max_known_urls` limits
- [ ] robots.txt respect
- [ ] Language-based URL filtering

### 7.4 Phase 7 release checklist

- [ ] Feed discovery finds feeds on 10+ major news sites
- [ ] Sitemap parsing handles real-world sitemaps
- [ ] Crawler respects robots.txt and politeness settings
- [ ] Tag `v0.7.0`
- [ ] Publish to crates.io

---

## Phase 8 — Production release (`1.0.0`)

**Goal:** Benchmark parity with Python trafilatura, comprehensive testing, documentation.

**Status:** 🔲 **PENDING**

### 8.1 Benchmark parity

- [ ] Run ScrapingHub article extraction benchmark
  - [ ] Match trafilatura's #1 ranking
- [ ] Run against Bevendorff et al. 2023 evaluation
  - [ ] ROUGE-LSum Mean F1
- [ ] Performance benchmark: single page latency
  - [ ] Target: <2ms
- [ ] Memory benchmark: peak RSS per page
  - [ ] Target: <500KB
- [ ] Throughput benchmark: pages/sec/core
  - [ ] Target: 500-1000

### 8.2 Testing + fuzzing

- [ ] Golden corpus: 100+ HTML files with expected outputs
- [ ] Fuzz `extract()` with `cargo-fuzz`
- [ ] Fuzz `htmldate_rs::find_date()`
- [ ] Property-based testing for dedup (proptest)
- [ ] CI/CD fully operational (see Phase 0 - Infrastructure & CI/CD)
  - [ ] All workflows passing on main branch
  - [ ] Release automation tested end-to-end

### 8.3 Documentation

- [ ] API docs (`cargo doc`) for all public types
- [ ] README with comprehensive examples
- [ ] ARCHITECTURE.md kept up to date
- [ ] CHANGELOG.md
- [x] crates.io metadata (description, categories, keywords)
  - [x] In Cargo.toml

### 8.4 Publishing

- [ ] `htmldate-rs` v1.0.0 on crates.io
  - [ ] Standalone, no kawat dependency
- [ ] `kawat` v1.0.0 on crates.io
  - [ ] All sub-crates published
- [ ] `kawat-cli` binary on GitHub Releases
  - [ ] Cross-compiled: x86_64-linux, aarch64-linux, x86_64-macos, aarch64-macos, x86_64-windows

### 8.5 Phase 8 release checklist

- [ ] Extraction quality ≥ trafilatura on standard benchmarks
- [ ] All fuzz targets run for ≥1 hour without panics
- [ ] `cargo deny check` clean (no license or vulnerability issues)
- [ ] Published to crates.io
- [ ] Tag `v1.0.0`
- [ ] Full CI/CD automation (80% coverage target)
- [ ] All quality gates operational
- [ ] Release automation tested end-to-end

---

## Estimated total effort

| Phase | Est. Rust LOC | Complexity | Notes |
|---|---|---|---|
| Phase 0 | ~500 | Low | Mostly YAML configuration and documentation |
| Phase 1 | ~1,800 | High | Internal tree representation is the key design decision |
| Phase 2 | ~1,000 | High | Comparison heuristic must be exact port |
| Phase 3 | ~1,200 | Medium | htmldate-rs is well-bounded; metadata is pattern-heavy |
| Phase 4 | ~700 | Medium | TEI-XML is the most complex format |
| Phase 5 | ~350 | Low | Simhash is the only non-trivial piece |
| Phase 6 | ~400 | Low | Mostly CLI plumbing |
| Phase 7 | ~1,100 | Medium | Crawler is most complex |
| Phase 8 | ~200 | Low | Testing, benchmarks, polish |
| **Total** | **~7,300** | | Comparable to trafilatura's 8,000 Python lines |

---

## Open design decisions

These decisions should be resolved during Phase 1 before committing to an implementation:

| # | Decision | Options | Recommendation | Phase |
|---|---|---|---|---|
| D1 | **Internal tree representation** | (A) Custom `KawatTree` struct, (B) `markup5ever_rcdom`, (C) Build XML string | **(A)** Custom struct — full control, no XML overhead, can carry kawat-specific metadata per node | 1 |
| D2 | **XPath engine** | (A) `sxd_html`+`sxd_xpath`, (B) `skyscraper`, (C) Translate to CSS + custom matchers | **(A)** Start with sxd for correctness, profile later, fall back to (C) if slow | 1 |
| D3 | **Tree mutation strategy** | (A) Clone + filter (immutable), (B) `lol_html` streaming rewrite, (C) In-place mutation | **(A)** for correctness first; **(B)** for `html2txt()` fast path | 1 |
| D4 | **Readability implementation** | (A) Wrap `dom_smoothie`, (B) Port `readability_lxml.py` directly | **(A)** Try dom_smoothie first; if parameter control insufficient, **(B)** | 2 |
| D5 | **Stoplist data format** | (A) Embed as `&[&str]` arrays, (B) Load from files at runtime, (C) Build script generates `.rs` | **(C)** Build script — keeps binary small, stoplists are large | 2 |
| D6 | **Async vs. sync API** | (A) Sync-first with async wrapper, (B) Async-first with `block_on` | **(A)** Sync-first — extraction is CPU-bound, async only needed for network | 6 |
