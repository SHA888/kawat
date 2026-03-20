# TODO.md — kawat development roadmap

> Cross-references [ARCHITECTURE.md](./ARCHITECTURE.md) throughout.
> Complexity estimates are in Rust LOC based on Python source analysis.

---

## Release plan

| Version | Phase | Milestone | Target |
|---|---|---|---|
| `0.1.0` | Phase 1 | Minimum viable extraction | Core extraction without fallbacks |
| `0.2.0` | Phase 2 | Fallback cascade | Readability + justext + baseline |
| `0.3.0` | Phase 3 | Metadata + date extraction | Full metadata, htmldate-rs v0.1.0 |
| `0.4.0` | Phase 4 | Output formats | All 7 output formats |
| `0.5.0` | Phase 5 | Dedup + language | Deduplication, language filtering |
| `0.6.0` | Phase 6 | CLI + batch | Full CLI, stdin/file/URL, batch |
| `0.7.0` | Phase 7 | Discovery | Feeds, sitemaps, crawling |
| `1.0.0` | Phase 8 | Production release | Benchmark parity, fuzz-tested, documented |

---

## Phase 1 — Minimum viable extraction (`0.1.0`)

Goal: `kawat::extract(html, &default_options)` returns main text content for well-structured pages (blogs, articles). No fallbacks, no metadata.

> Architecture ref: [§2.3 steps 1, 5, 6, 8a](#23-the-extraction-cascade), [§3.4 internal tag set](#34-internal-xml-tree-representation)

### 1.1 kawat-xpath: XPath evaluation layer

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| Integrate `sxd_html` + `sxd_xpath` in `eval.rs` | ✅ Scaffolded | [§2.4](#24-xpath-feature-subset) | 80 | Wrapper around sxd parse + evaluate |
| Port all 50 XPath expressions from `xpaths.py` to `compiled.rs` | ✅ Done | [§2.1 xpaths.py](#21-module-inventory) | 200 | Verbatim copy, 15 expression groups |
| Test each BODY_XPATH expression against 10+ real HTML pages | 🔲 | | — | Use Python lxml to generate expected match counts |
| Test OVERALL_DISCARD_XPATH against same pages | 🔲 | | — | |
| Fallback path: if `sxd_html` fails on malformed HTML, use `scraper` CSS selectors | 🔲 | [§2.4 CSS equivalent column](#24-xpath-feature-subset) | 150 | Translate the ~15 expressions that use `translate()` (no CSS equivalent) to custom scraper filters |
| Benchmark: sxd_xpath eval time vs. scraper CSS on 100 pages | 🔲 | | — | Decision point: keep both or drop one |

### 1.2 kawat-html: tree cleaning + tag conversion

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `tree_cleaning()`: remove MANUALLY_CLEANED (44 tags) using scraper | 🔲 | [§2.3 step 5](#23-the-extraction-cascade) | 120 | `scraper::Html` is immutable; need to use `lol_html` for mutation or rebuild tree |
| `tree_cleaning()`: strip MANUALLY_STRIPPED (20 tags) keeping text | 🔲 | | 60 | |
| Handle table-in-figure edge case (change `<figure>` containing `<table>` to `<div>`) | 🔲 | htmlprocessing.py:56-58 | 10 | |
| Preserve `<img>` tags when `options.images=true` | 🔲 | htmlprocessing.py:60-63 | 15 | |
| `convert_tags()`: h1-h6→head, b/strong/em/i→hi, a→ref, ul/ol→list, li→item, br→lb, blockquote→quote, del/s→del, code/pre→code | 🔲 | [§3.4](#34-internal-xml-tree-representation), htmlprocessing.py:387-437 | 200 | This is the critical tag normalization |
| `convert_tags()`: `_is_code_block()` detection (code vs. inline code) | 🔲 | htmlprocessing.py:318-325 | 30 | |
| `convert_tags()`: `convert_link()` with base_url resolution | 🔲 | htmlprocessing.py:375-385 | 25 | |
| `link_density_test()` for generic elements | 🔲 | [§3.5](#35-scoring-and-heuristics-detail), htmlprocessing.py:128-165 | 60 | |
| `link_density_test_tables()` specialized for tables | 🔲 | htmlprocessing.py:168-185 | 40 | |
| `delete_by_link_density()` with backtracking | 🔲 | htmlprocessing.py:187-216 | 80 | |
| `handle_textnode()` + `process_node()` | 🔲 | htmlprocessing.py:218-280 | 100 | |
| **Decision: internal tree representation** | 🔲 | [§3.4](#34-internal-xml-tree-representation) | — | Options: (A) custom tree struct with kawat tags, (B) lxml-style Element tree via `markup5ever_rcdom`, (C) string-based XML building. Recommend (A). |

### 1.3 kawat-extract: main content extraction

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| Internal tree type definition (`KawatTree`, `KawatNode`) | 🔲 | [§3.4](#34-internal-xml-tree-representation) | 150 | Body, children, tag, text, tail, attributes |
| `_extract()`: iterate BODY_XPATH, first match wins | 🔲 | [§2.3 step 8a](#23-the-extraction-cascade), main_extractor.py:586-636 | 100 | |
| `prune_unwanted_sections()`: OVERALL_DISCARD + link density passes | 🔲 | main_extractor.py:552-583 | 80 | |
| `handle_textelem()`: dispatcher by tag type | 🔲 | main_extractor.py:498-526 | 60 | |
| `handle_titles()` | 🔲 | main_extractor.py:44-67 | 40 | |
| `handle_paragraphs()` | 🔲 | main_extractor.py:275-355 | 120 | Most complex handler |
| `handle_formatting()` | 🔲 | main_extractor.py:70-117 | 60 | |
| `handle_lists()` | 🔲 | main_extractor.py:164-203 | 60 | |
| `handle_quotes()` + `handle_code_blocks()` | 🔲 | main_extractor.py:230-247, 221-228 | 40 | |
| `handle_table()` | 🔲 | main_extractor.py:366-454 | 120 | Includes cell type detection, nested content |
| `handle_image()` | 🔲 | main_extractor.py:456-496 | 50 | |
| `handle_other_elements()` | 🔲 | main_extractor.py:248-273 | 40 | |
| `recover_wild_text()` | 🔲 | main_extractor.py:528-550 | 60 | |
| `extract_content()` wrapper | 🔲 | main_extractor.py:639-659 | 30 | |
| `baseline()` last-resort extraction | 🔲 | [§2.3 step 8c](#23-the-extraction-cascade), baseline.py:25-101 | 100 | JSON-LD→article→p→body |
| `html2txt()` | 🔲 | baseline.py:104-123 | 20 | |

### 1.4 kawat-core: wire it together

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `cascade::run()` steps 1, 5, 6, 8a only (no metadata, no fallbacks) | 🔲 | [§2.3](#23-the-extraction-cascade) | 100 | |
| `ExtractorOptions` default config | ✅ Done | settings.cfg | | |
| `Document` struct | ✅ Done | | | |
| TXT output (step 12, txt only) | 🔲 | | 30 | Minimal: just join text nodes |

### 1.5 kawat facade + basic test

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `kawat::extract()` returns plain text | 🔲 | | 10 | |
| Golden test: 20 HTML files, compare TXT output with Python trafilatura | 🔲 | [§4.1](#41-golden-test-corpus) | — | Acceptance: ≥70% match |

### 1.6 Phase 1 release checklist

- [ ] `cargo test` passes all unit tests
- [ ] Golden test ≥70% match rate
- [ ] `cargo clippy` clean
- [ ] `cargo doc` builds
- [ ] README with basic usage example
- [ ] Tag `v0.1.0`

---

## Phase 2 — Fallback cascade (`0.2.0`)

Goal: Add readability + justext fallbacks + comparison heuristic. Extraction quality should match trafilatura on ≥90% of golden corpus.

> Architecture ref: [§2.3 step 8b](#23-the-extraction-cascade), [§3.5 scoring detail](#35-scoring-and-heuristics-detail)

### 2.1 kawat-readability: readability fallback

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| Integrate `dom_smoothie` crate | 🔲 | readability_lxml.py (512 lines) | 80 | Wrapper matching `try_readability()` interface |
| Match parameters: `min_text_length=25`, `retry_length=250` | 🔲 | external.py:36-37 | — | Verify dom_smoothie exposes these |
| If dom_smoothie lacks parameter control: port readability_lxml.py directly | 🔲 | | ~600 | Fallback plan: scoring + candidate selection is ~400 LOC core |
| `sanitize_tree()` post-processing for readability output | 🔲 | external.py:163-190 | 50 | |

### 2.2 kawat-justext: paragraph classification

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `Paragraph` struct (text, word_count, link_density, stopword_density, is_heading, is_boilerplate, class) | 🔲 | [§3.5](#35-scoring-and-heuristics-detail) | 40 | |
| `ParagraphMaker::make_paragraphs(tree)` — extract paragraphs from HTML tree | 🔲 | justext source | 150 | Walk tree, split by block elements, accumulate text |
| `classify_paragraphs()` — initial classification | 🔲 | [§3.5](#35-scoring-and-heuristics-detail) | 100 | Parameters: 50, 150, 0.1, 0.2, 0.25, true |
| `revise_paragraph_classification()` — context-based reclassification | 🔲 | | 80 | max_heading_distance=150 |
| Stoplists for 29 languages (embed as static data) | 🔲 | settings.py:442-475 | — | Encode from justext's stoplist files. Priority: en, id, de, fr, es |
| Test: classify a known article, verify boilerplate paragraphs removed | 🔲 | | — | |

### 2.3 kawat-core: comparison heuristic

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `compare_extraction()` — full decision tree | 🔲 | [§2.3 step 8b](#23-the-extraction-cascade), external.py:45-108 | 120 | Port all 8 conditions exactly |
| `justext_rescue()` — justext as second fallback | 🔲 | external.py:153-160 | 30 | |
| SANITIZED_XPATH constant for triggering justext | 🔲 | external.py:29 | 5 | `.//aside\|.//audio\|.//button\|...` |
| Wire steps 8b + 8c into `cascade::run()` | 🔲 | | 40 | |

### 2.4 Comment extraction

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `extract_comments()` using COMMENTS_XPATH | 🔲 | [§2.3 step 7](#23-the-extraction-cascade), main_extractor.py:676-707 | 60 | |
| `process_comments_node()` | 🔲 | main_extractor.py:662-673 | 20 | |
| COMMENTS_DISCARD_XPATH pruning | 🔲 | | 10 | |
| REMOVE_COMMENTS_XPATH (precision mode) | 🔲 | | 10 | |
| Wire into cascade step 7 | 🔲 | | 15 | |

### 2.5 Phase 2 release checklist

- [ ] Golden test ≥90% match rate
- [ ] Readability fallback fires correctly on pages where main extraction fails
- [ ] justext fallback fires on pages with embedded noise (ads, widgets)
- [ ] Comment extraction produces output matching trafilatura
- [ ] Tag `v0.2.0`

---

## Phase 3 — Metadata + date extraction (`0.3.0`)

Goal: Full metadata extraction. `htmldate-rs` published as standalone crate.

> Architecture ref: [§2.3 step 3](#23-the-extraction-cascade), [§2.5 htmldate](#25-htmldate-internal-architecture)

### 3.1 htmldate-rs: date extraction

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| Tier 1 `meta.rs`: JSON-LD extraction | ✅ Scaffolded | [§2.5 tier 1](#25-htmldate-internal-architecture) | 120 | `datePublished`, `dateModified`, `dateCreated`, `@graph` arrays |
| Tier 1 `meta.rs`: OG + standard meta tags (15 selectors) | ✅ Scaffolded | | 80 | `article:published_time`, `dcterms.date`, etc. |
| Tier 2 `structural.rs`: `<time datetime="">` extraction | ✅ Scaffolded | | 40 | |
| Tier 2 `structural.rs`: date class/id patterns (16 patterns) | ✅ Scaffolded | | 60 | |
| Tier 3 `text.rs`: regex-based text scanning | ✅ Scaffolded | | 80 | ISO, named month, dot/slash formats |
| Tier 3 `text.rs`: disambiguation algorithm (extensive mode) | 🔲 | | 150 | Position preference, context weighting, candidate ranking |
| `validators.rs`: 22 date format strings | ✅ Done | | | |
| `validators.rs`: plausibility checks (future date, min_date) | ✅ Done | | | |
| `patterns.rs`: compiled regex patterns | ✅ Done | | | |
| Multilingual month names (at minimum: en, id, de, fr, es) | 🔲 | | 100 | |
| Test against htmldate's own test corpus (1000 pages) | 🔲 | | — | Target: ≥0.85 F-score (fast), ≥0.90 (extensive) |
| Publish `htmldate-rs` v0.1.0 to crates.io | 🔲 | | — | |

### 3.2 kawat-metadata: full metadata extraction

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `examine_meta()`: scan all `<meta>` tags for name, property, itemprop | 🔲 | metadata.py:221-319 | 150 | OG properties, Twitter cards, Dublin Core, itemprop |
| `extract_opengraph()`: OG-specific extraction | 🔲 | metadata.py:198-219 | 40 | og:title, og:description, og:site_name, og:image, og:type |
| `extract_meta_json()` / `extract_json_ld()`: JSON-LD metadata | 🔲 | json_metadata.py (268 lines) | 200 | Schema.org types, `@graph` arrays, nested objects, author normalization |
| `extract_title()`: title from TITLE_XPATHS → `<title>` tag → OG → JSON-LD | 🔲 | metadata.py:354-380, xpaths.py:261-265 | 60 | |
| `extract_author()`: author from AUTHOR_XPATHS + AUTHOR_DISCARD_XPATHS + JSON-LD + meta | 🔲 | metadata.py:382-390, xpaths.py:214-232 | 80 | |
| `extract_url()`: canonical URL from `<link rel="canonical">`, OG, meta | 🔲 | metadata.py:392-417 | 50 | |
| `extract_sitename()` | 🔲 | metadata.py:419-423 | 15 | |
| `extract_catstags()`: categories + tags from CATEGORIES_XPATHS, TAGS_XPATHS | 🔲 | metadata.py:425-450, xpaths.py:235-257 | 60 | |
| `extract_license()`: Creative Commons detection | 🔲 | metadata.py:452-483 | 50 | |
| `Document.clean_and_trim()`: HTML entity unescape, length limit, whitespace normalization | 🔲 | settings.py:289-299 | 20 | |
| Integration: wire `extract_metadata()` into cascade step 3 | 🔲 | | 30 | |
| `only_with_metadata` filtering (require date + title + url) | 🔲 | core.py:252-257 | 10 | |
| `url_blacklist` filtering | 🔲 | core.py:247-250 | 10 | |

### 3.3 Phase 3 release checklist

- [ ] `htmldate-rs` v0.1.0 published on crates.io
- [ ] Metadata extraction matches trafilatura on ≥85% of golden corpus fields
- [ ] JSON-LD, OG, meta tag, and XPath-based extraction all functional
- [ ] Tag `v0.3.0`

---

## Phase 4 — Output formats (`0.4.0`)

Goal: All 7 output formats producing correct output.

> Architecture ref: [§3.2 kawat-output responsibilities](#32-crate-responsibilities)

### 4.1 kawat-output: format converters

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `txt.rs`: `xmltotxt()` — convert internal tree to plain text | 🔲 | xml.py (portions) | 80 | Join text nodes, handle whitespace |
| `markdown.rs`: TXT with markdown formatting (headers, bold, italic, links, lists) | 🔲 | xml.py (portions) | 120 | Triggered by `include_formatting` or format=markdown |
| `json_output.rs`: `build_json_output()` with metadata fields | 🔲 | xml.py:build_json_output | 60 | serde_json serialization |
| `xml_output.rs`: `control_xml_output()` basic XML | 🔲 | xml.py:control_xml_output | 120 | quick-xml writer |
| `xml_tei.rs`: TEI-XML with validation | 🔲 | xml.py (TEI portions), data/tei_corpus.dtd | 200 | Most complex format. TEI_VALID_TAGS, namespace handling |
| `csv_output.rs`: `xmltocsv()` tab-separated | 🔲 | xml.py:xmltocsv | 40 | |
| `html_output.rs`: `build_html_output()` | 🔲 | htmlprocessing.py:439-469 | 50 | |
| `determine_returnstring()` dispatcher | 🔲 | core.py:44-99 | 60 | TXT/MD header with metadata in YAML front matter |
| Unicode normalization (NFC) | 🔲 | core.py:99 | 5 | |

### 4.2 Phase 4 release checklist

- [ ] All 7 formats produce output
- [ ] JSON output matches trafilatura's JSON on golden corpus
- [ ] XML well-formedness validated
- [ ] TEI-XML validates against DTD (stretch goal)
- [ ] Tag `v0.4.0`

---

## Phase 5 — Dedup + language detection (`0.5.0`)

Goal: Document-level deduplication, language filtering.

> Architecture ref: [§2.3 steps 10-11](#23-the-extraction-cascade)

### 5.1 kawat-dedup

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `content_fingerprint()`: SHA-1 + Base64 | ✅ Done | deduplication.py:165-170 | 30 | |
| `LRUCache` with configurable capacity (default 4096) | ✅ Done | deduplication.py:173-257 | 40 | Using `lru` crate |
| `Simhash`: token sampling + weighted hash | 🔲 | deduplication.py:82-163 | 120 | Custom simhash, not external crate |
| `sample_tokens()` + `sample_tokens_fallback()` | 🔲 | deduplication.py:46-73 | 40 | |
| `generate_bow_hash()` | 🔲 | deduplication.py:75-80 | 15 | |
| `duplicate_test()`: combine simhash + LRU | 🔲 | deduplication.py:267-278 | 30 | |
| `is_similar_domain()` | 🔲 | deduplication.py:29-34 | 10 | |
| Wire into cascade step 10 | 🔲 | | 15 | |
| `reset_caches()` for memory management | 🔲 | meta.py:12-30 | 10 | |

### 5.2 Language detection

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `check_html_lang()`: check `<html lang="">` attribute | 🔲 | utils.py (partial) | 20 | |
| `language_filter()`: detect language of extracted text | 🔲 | utils.py (partial) | 40 | Using `lingua` crate, behind feature flag |
| Wire into cascade steps 2 + 11 | 🔲 | | 15 | |

### 5.3 Phase 5 release checklist

- [ ] Duplicate documents correctly rejected
- [ ] `reset_caches()` frees memory
- [ ] Language filtering works for ≥10 languages
- [ ] Tag `v0.5.0`

---

## Phase 6 — CLI + batch processing (`0.6.0`)

Goal: Full CLI with stdin/file/URL input, batch processing, output to files.

### 6.1 kawat-cli

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| Clap argument parsing | ✅ Done | cli.py (244 lines) | | |
| Single URL extraction (`-u`) | 🔲 | | 20 | |
| File input (`-i`) | 🔲 | | 10 | |
| Stdin piping | 🔲 | | 10 | |
| Batch URL list (`--inputfile links.txt`) | 🔲 | cli_utils.py:batch portions | 60 | |
| Output directory (`--outputdir`) | 🔲 | cli_utils.py | 40 | |
| HTML backup directory (`--backup-dir`) | 🔲 | cli_utils.py | 20 | |
| Parallel processing (tokio + configurable concurrency) | 🔲 | settings.py:307 (PARALLEL_CORES) | 80 | |
| Politeness: sleep between requests, user-agent, robots.txt respect | 🔲 | downloads.py | 60 | |
| Config file support (`--config-file`) | 🔲 | | 30 | |
| `prune_xpath` CLI flag | 🔲 | | 10 | |
| Progress output (stderr) | 🔲 | | 20 | |

### 6.2 Phase 6 release checklist

- [ ] `kawat -u URL` works end to end
- [ ] `cat page.html | kawat` works
- [ ] Batch processing with `--inputfile`
- [ ] Parallel downloads with politeness
- [ ] Tag `v0.6.0`

---

## Phase 7 — Discovery (`0.7.0`)

Goal: Feed, sitemap, and crawler support.

### 7.1 Feed discovery

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `find_feed_urls()`: auto-discover RSS/Atom/JSON feeds from homepage | 🔲 | feeds.py (312 lines) | 250 | |
| Feed parsing (RSS, Atom, JSON Feed) | 🔲 | | 150 | Consider `feed-rs` crate |
| URL filtering by target language heuristic | 🔲 | feeds.py | 40 | |

### 7.2 Sitemap discovery

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `sitemap_search()`: discover and parse XML/TXT sitemaps | 🔲 | sitemaps.py (304 lines) | 200 | |
| Sitemap index support (nested sitemaps) | 🔲 | | 40 | |
| URL deduplication across sitemaps | 🔲 | | 20 | |

### 7.3 Web crawler

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| `focused_crawler()`: BFS crawler with URL frontier | 🔲 | spider.py (352 lines) | 300 | |
| `max_seen_urls`, `max_known_urls` limits | 🔲 | | — | |
| robots.txt respect | 🔲 | | 40 | |
| Language-based URL filtering | 🔲 | | 30 | |

### 7.4 Phase 7 release checklist

- [ ] Feed discovery finds feeds on 10+ major news sites
- [ ] Sitemap parsing handles real-world sitemaps
- [ ] Crawler respects robots.txt and politeness settings
- [ ] Tag `v0.7.0`

---

## Phase 8 — Production release (`1.0.0`)

Goal: Benchmark parity with Python trafilatura, comprehensive testing, documentation.

### 8.1 Benchmark parity

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| Run ScrapingHub article extraction benchmark | 🔲 | [§2.1](#21-module-inventory) | — | Match trafilatura's #1 ranking |
| Run against Bevendorff et al. 2023 evaluation | 🔲 | | — | ROUGE-LSum Mean F1 |
| Performance benchmark: single page latency | 🔲 | [§5](#5-performance-targets) | — | Target: <2ms |
| Memory benchmark: peak RSS per page | 🔲 | | — | Target: <500KB |
| Throughput benchmark: pages/sec/core | 🔲 | | — | Target: 500-1000 |

### 8.2 Testing + fuzzing

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| Golden corpus: 100+ HTML files with expected outputs | 🔲 | [§4.1](#41-golden-test-corpus) | — | |
| Fuzz `extract()` with `cargo-fuzz` | 🔲 | | — | |
| Fuzz `htmldate_rs::find_date()` | 🔲 | | — | |
| Property-based testing for dedup (proptest) | 🔲 | | — | |
| CI: GitHub Actions for Linux/macOS/Windows | 🔲 | | — | |

### 8.3 Documentation

| Task | Status | Ref | Est. LOC | Notes |
|---|---|---|---|---|
| API docs (`cargo doc`) for all public types | 🔲 | | — | |
| README with comprehensive examples | 🔲 | | — | |
| ARCHITECTURE.md (this file) kept up to date | 🔲 | | — | |
| CHANGELOG.md | 🔲 | | — | |
| crates.io metadata (description, categories, keywords) | ✅ Done | | — | In Cargo.toml |

### 8.4 Publishing

| Task | Status | Notes |
|---|---|---|
| `htmldate-rs` v1.0.0 on crates.io | 🔲 | Standalone, no kawat dependency |
| `kawat` v1.0.0 on crates.io | 🔲 | All sub-crates published |
| `kawat-cli` binary on GitHub Releases | 🔲 | Cross-compiled: x86_64-linux, aarch64-linux, x86_64-macos, aarch64-macos, x86_64-windows |

### 8.5 Phase 8 release checklist

- [ ] Extraction quality ≥ trafilatura on standard benchmarks
- [ ] All fuzz targets run for ≥1 hour without panics
- [ ] `cargo deny check` clean (no license or vulnerability issues)
- [ ] Published to crates.io
- [ ] Tag `v1.0.0`

---

## Estimated total effort

| Phase | Est. Rust LOC | Complexity | Notes |
|---|---|---|---|
| Phase 1 | ~1,800 | High | Internal tree representation is the key design decision |
| Phase 2 | ~1,000 | High | Comparison heuristic must be exact port |
| Phase 3 | ~1,200 | Medium | htmldate-rs is well-bounded; metadata is pattern-heavy |
| Phase 4 | ~700 | Medium | TEI-XML is the most complex format |
| Phase 5 | ~350 | Low | Simhash is the only non-trivial piece |
| Phase 6 | ~400 | Low | Mostly CLI plumbing |
| Phase 7 | ~1,100 | Medium | Crawler is most complex |
| Phase 8 | ~200 | Low | Testing, benchmarks, polish |
| **Total** | **~6,800** | | Comparable to trafilatura's 8,000 Python lines |

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
