# ARCHITECTURE.md — kawat

> Last updated: 2026-03-20
> Reference implementation: [trafilatura v2.0.0](https://github.com/adbar/trafilatura) (8,000 lines Python)

## 1. Overview

kawat is a Rust workspace that extracts main text content, metadata, and comments from HTML documents. It is a ground-up reimplementation of the extraction logic from Python's trafilatura, using Rust-native libraries.

The name *kawat* (Indonesian: "wire") mirrors *trafilatura* (Italian: "wire drawing") — raw material (HTML) pulled through a die (extraction algorithms) to produce refined output (clean text).

### 1.1 Design principles

1. **Behavioral parity first, then optimize.** Match trafilatura's output on its own test corpus before diverging.
2. **Composable crates.** Each extraction stage is a separate publishable crate. Users can depend on `htmldate-rs` alone without pulling in the full extraction stack.
3. **No unsafe.** Workspace-level `#[deny(unsafe_code)]`.
4. **XPath expressions preserved verbatim.** The curated XPath expressions from `xpaths.py` encode 7+ years of empirical tuning. They are ported as-is, not rewritten.

---

## 2. Reference architecture (trafilatura v2.0.0)

### 2.1 Module inventory

| Module | Lines | Role |
|---|---|---|
| `main_extractor.py` | 707 | Core text extraction: BODY_XPATH iteration, text node handling (27 functions: handle_titles, handle_paragraphs, handle_table, handle_lists, handle_quotes, handle_code_blocks, handle_image, handle_textelem, recover_wild_text, extract_content, extract_comments, etc.) |
| `core.py` | 636 | Orchestrator: `bare_extraction()`, `extract()`, `extract_with_metadata()`, `trafilatura_sequence()`, `determine_returnstring()` |
| `xml.py` | 632 | Output: XML, TEI-XML, JSON, CSV, TXT/Markdown conversion. `xmltotxt()`, `build_json_output()`, `control_xml_output()`, `xmltocsv()` |
| `metadata.py` | 592 | Metadata: title, author, URL, sitename, categories, tags, license. `extract_metadata()`, `examine_meta()`, `extract_opengraph()`, `extract_title()`, `extract_author()`, `extract_url()`, `extract_catstags()`, `extract_license()` |
| `utils.py` | 535 | HTML loading (`load_html()`), Unicode normalization, language detection (`language_filter()`), text utilities (`trim()`, `text_chars_test()`) |
| `downloads.py` | 525 | HTTP fetching via urllib3, response handling, politeness |
| `readability_lxml.py` | 512 | **Bundled fork** of readability-lxml. `Document.summary()`, element scoring, candidate selection. NOT an external dependency. |
| `cli_utils.py` | 511 | CLI batch processing, file I/O, URL list handling |
| `settings.py` | 475 | `Extractor` class (28 slots), `Document` class (20 slots), `MANUALLY_CLEANED` (44 tags), `MANUALLY_STRIPPED` (20 tags), `TAG_CATALOG` (10 tags), `CUT_EMPTY_ELEMS` (23 tags), `JUSTEXT_LANGUAGES` (29 languages), config loading |
| `htmlprocessing.py` | 470 | Tree cleaning (`tree_cleaning()`), link density (`link_density_test()`, `delete_by_link_density()`), tag conversion (`convert_tags()`: h1-h6→head, b/strong→hi, a→ref, ul/ol→list, br→lb), `handle_textnode()`, `process_node()` |
| `spider.py` | 352 | Web crawler: `focused_crawler()` |
| `feeds.py` | 312 | RSS/Atom/JSON feed parsing: `find_feed_urls()` |
| `sitemaps.py` | 304 | Sitemap XML/TXT parsing: `sitemap_search()` |
| `deduplication.py` | 278 | `Simhash` class (custom implementation), `LRUCache` class (custom, not stdlib), `content_fingerprint()` (SHA-1 + Base64), `duplicate_test()` |
| `json_metadata.py` | 268 | JSON-LD + Schema.org parsing: `extract_json()`, `normalize_authors()`, `normalize_json()` |
| `xpaths.py` | 267 | 15 compiled XPath expression groups (~50 individual expressions): `BODY_XPATH` (5), `COMMENTS_XPATH` (4), `REMOVE_COMMENTS_XPATH` (1), `OVERALL_DISCARD_XPATH` (2), `TEASER_DISCARD_XPATH` (1), `PRECISION_DISCARD_XPATH` (2), `DISCARD_IMAGE_ELEMENTS` (1), `COMMENTS_DISCARD_XPATH` (3), `AUTHOR_XPATHS` (3), `AUTHOR_DISCARD_XPATHS` (2), `CATEGORIES_XPATHS` (6), `TAGS_XPATHS` (4), `TITLE_XPATHS` (3) |
| `external.py` | 190 | Fallback comparison: `compare_extraction()` decision tree, `try_readability()`, `try_justext()`, `justext_rescue()`, `sanitize_tree()` |
| `baseline.py` | 123 | Last-resort extraction: JSON-LD articleBody → `<article>` → `<p>` elements → `body.text_content()`. Also `html2txt()`. |

### 2.2 External dependencies (what kawat must replicate or wrap)

| Python dep | What it does | Rust equivalent in kawat |
|---|---|---|
| `lxml` | HTML parsing, XPath evaluation, tree manipulation | `sxd_html` + `sxd_xpath` (XPath path); `scraper` (CSS selector path) |
| `htmldate >= 1.9.2` | Date extraction (its own 6-module package) | `htmldate-rs` (our port) |
| `justext >= 3.0.1` | Paragraph classification by stopword density | `kawat-justext` (pure Rust port) |
| `courlan >= 1.3.2` | URL normalization, validation, domain extraction | `url` crate + custom normalization |
| `charset_normalizer` | Encoding detection | `encoding_rs` |
| `urllib3` | HTTP client | `reqwest` |
| Optional: `py3langid` | Language detection | `lingua` crate |

### 2.3 The extraction cascade

This is the exact sequence from `core.py:bare_extraction()` (lines 223-348), validated against the source:

```
 1. load_html(filecontent)
 │  Parse HTML string via lxml.html. In kawat: scraper::Html::parse_document()
 │
 2. check_html_lang(tree, target_lang)           [fast mode only]
 │  Check <html lang="xx"> attribute against target language.
 │
 3. extract_metadata(tree, url, date_params, fast, author_blacklist)
 │  [only if with_metadata=true]
 │  Returns Document with: title, author, date, url, hostname,
 │  description, sitename, categories, tags, license, image, pagetype
 │
 4. prune_unwanted_nodes(tree, user_prune_xpath)
 │  Apply user-specified XPath/CSS pruning expressions
 │
 5. tree_cleaning(tree, options)
 │  Remove MANUALLY_CLEANED (44 tags) entirely
 │  Strip MANUALLY_STRIPPED (20 tags) keeping text
 │  Handle table-in-figure edge case
 │  Handle image preservation if options.images
 │
 6. convert_tags(cleaned_tree, options, url)
 │  h1-h6 → head, b/strong/em/i → hi, a → ref,
 │  ul/ol → list, li → item, br → lb, blockquote → quote,
 │  del/s/strike → del, details/summary → special handling,
 │  code/pre → code with is_code_block detection
 │
 7. extract_comments(cleaned_tree, options)
 │  Iterate COMMENTS_XPATH (4 expressions)
 │  Prune with COMMENTS_DISCARD_XPATH
 │  Return (comments_body, comments_text, comments_len, tree_sans_comments)
 │  If precision mode: also run REMOVE_COMMENTS_XPATH
 │
 8. trafilatura_sequence(cleaned_tree, cleaned_tree_backup, tree_backup, options):
 │  ├── 8a. extract_content(cleaned_tree, options)
 │  │       _extract():
 │  │         - Build potential_tags set from TAG_CATALOG + options
 │  │         - FOR EACH expr IN BODY_XPATH (5 expressions, FIRST MATCH WINS):
 │  │             subtree = expr(tree).first_non_none()
 │  │             subtree = prune_unwanted_sections(subtree):
 │  │               - prune_unwanted_nodes(OVERALL_DISCARD_XPATH, with_backup=True)
 │  │               - if !recall: prune(TEASER_DISCARD_XPATH)
 │  │               - if precision: prune(PRECISION_DISCARD_XPATH)
 │  │               - 2× passes: delete_by_link_density(div), (list), (p)
 │  │               - if tables: link_density_test_tables
 │  │               - if precision: delete trailing heads, link_density head/quote
 │  │             if <p> text too short → add 'div' to potential_tags
 │  │             FOR EACH subelement:
 │  │               handle_textelem() → dispatch by tag:
 │  │                 head → handle_titles()
 │  │                 hi/ref/span → handle_formatting()
 │  │                 list → handle_lists()
 │  │                 quote/code → handle_quotes() / handle_code_blocks()
 │  │                 table → handle_table()
 │  │                 graphic → handle_image()
 │  │                 p → handle_paragraphs()
 │  │                 * → handle_other_elements()
 │  │             Remove trailing NOT_AT_THE_END tags
 │  │             BREAK if result has >1 children
 │  │
 │  │       If too short → recover_wild_text(backup):
 │  │         Scan for orphan p/blockquote/code/pre/q/quote/table
 │  │         (in recall mode also div/lb)
 │  │
 │  ├── 8b. [if NOT fast] compare_extraction(cleaned_backup, tree_backup, body, text, len, options)
 │  │       DECISION TREE (external.py lines 45-108):
 │  │         if recall AND len > 10×min_size → keep own
 │  │         readability_result = try_readability(backup_tree)
 │  │           → bundled readability_lxml.Document(tree, min_text_length=25, retry_length=250).summary()
 │  │         algo_text = readability_result.text_content()
 │  │         COMPARE:
 │  │           len_algo==0 or ==len_text       → keep own
 │  │           len_text==0, len_algo>0          → use readability
 │  │           len_text > 2×len_algo            → keep own
 │  │           len_algo > 2×len_text (not JSON) → use readability
 │  │           own has no <p> text, algo > 2×min → use readability
 │  │           own has more tables than <p>, algo > 2×min → use readability
 │  │           recall mode, own no headings, algo has h2-h4, algo longer → use readability
 │  │           else → keep own
 │  │         THEN regardless:
 │  │           if result has sanitized_xpath matches OR too short → justext_rescue():
 │  │             basic_cleaning(tree)
 │  │             ParagraphMaker.make_paragraphs(tree)
 │  │             classify_paragraphs(paragraphs, stoplist, 50, 150, 0.1, 0.2, 0.25, True)
 │  │             revise_paragraph_classification(paragraphs, 150)
 │  │             keep non-boilerplate paragraphs
 │  │           justext replaces main if main is NOT > 4×longer
 │  │         POST: if used readability and NOT justext → sanitize_tree()
 │  │
 │  └── 8c. [if len < min_extracted_size AND focus != precision]
 │          baseline(tree_backup):
 │            JSON-LD articleBody → <article>.text_content() → <p>/<blockquote>/<code> → body.text_content()
 │
 9. Size checks
 │  min_output_size (default=1), min_extracted_comm_size (default=1)
 │  max_tree_size (strip 'hi' tags, then error if still too long)
 │
10. duplicate_test(postbody, options)
 │  Simhash + LRU cache at document level
 │
11. language_filter(text, comments, target_lang, document)
 │  py3langid detection, reject if mismatch
 │
12. determine_returnstring(document, options)
    Dispatch to output formatter:
      xml/xmltei → control_xml_output()
      csv → xmltocsv()
      json → build_json_output()
      html → build_html_output()
      txt/markdown → header + xmltotxt()
```

### 2.4 XPath feature subset

The exact XPath features used across all 50 expressions in `xpaths.py`:

| Feature | Count | Example | CSS equivalent |
|---|---|---|---|
| `contains(@attr, 'val')` | 244 | `contains(@class, "post-content")` | `[class*="post-content"]` |
| `self::tag` | 86 | `self::article or self::div` | `article, div` (in context) |
| `starts-with(@attr, 'val')` | 52 | `starts-with(@id, 'primary')` | `[id^="primary"]` |
| `translate(@attr, "X","x")` | 20 | Case-insensitive match | No direct CSS equivalent |
| `[1]` positional | 7 | `(.//article)[1]` | `:first-of-type` (approximate) |
| `@attr` test | varied | `@itemprop="articleBody"` | `[itemprop="articleBody"]` |
| `\|` union | varied | `@id\|@class` shorthand | Multiple selectors |
| `or` / `and` | 310 | Combining predicates | `,` for or; chaining for and |

**Not used:** `ancestor::`, `following-sibling::`, `preceding-sibling::`, `last()`, `position()>1`, `substring()`, `normalize-space()`, any XPath 2.0+ features.

### 2.5 htmldate internal architecture

Source: [htmldate v1.9.4](https://github.com/adbar/htmldate) (6 modules):

```
htmldate/
  core.py         — orchestrator: find_date()
  extractors.py   — pattern matching against HTML structure
  meta.py         — header/meta tag extraction (OG, Dublin Core, etc.)
  settings.py     — config, constants
  utils.py        — HTML loading, text processing
  validators.py   — date plausibility checks, format conversion
```

Three-tier heuristic cascade:
1. **Header metadata** (fast): JSON-LD `datePublished`/`dateModified`, Open Graph `article:published_time`, standard meta tags (`date`, `dcterms.date`, `DC.date.issued`, `sailthru.date`, `http-equiv=date`), itemprop `datePublished`
2. **Structural HTML** (medium): `<time datetime="">`, `<abbr class="published">`, elements with date-related class/id (30+ patterns)
3. **Bare text** (slow/extensive): regex scan of cleaned text, candidate collection, disambiguation by position + context

Performance on 1000 pages: fast mode 0.903 F-score, extensive mode 0.928 F-score.

---

## 3. kawat workspace architecture

### 3.1 Crate dependency graph

```
kawat-cli (binary)
  └── kawat (facade)
        ├── kawat-core (orchestrator)
        │     ├── kawat-html           → scraper, lol_html, encoding_rs
        │     ├── kawat-xpath          → sxd_html, sxd-document, sxd-xpath, scraper
        │     ├── kawat-extract        → kawat-xpath, kawat-html, scraper
        │     ├── kawat-readability    → dom_smoothie, scraper
        │     ├── kawat-justext        → scraper
        │     ├── kawat-metadata       → kawat-xpath, htmldate-rs, scraper, url, serde_json
        │     ├── kawat-dedup          → lru, sha1_smol
        │     └── kawat-output         → serde_json, quick-xml, csv
        ├── reqwest (fetch)
        └── htmldate-rs (standalone)
              └── chrono, regex, once_cell, serde_json, scraper
```

### 3.2 Crate responsibilities

| Crate | Mirrors (trafilatura) | Key types / functions |
|---|---|---|
| `kawat` | `__init__.py` | `extract()`, `bare_extraction()`, `fetch_url()`, `html2txt()` |
| `kawat-core` | `core.py` | `cascade::run()`, `compare::compare_extraction()`, `ExtractorOptions`, `Document` |
| `kawat-html` | `htmlprocessing.py` + tag lists from `settings.py` | `tree_cleaning()`, `convert_tags()`, `link_density_test()`, `delete_by_link_density()`, `handle_textnode()`, `process_node()` |
| `kawat-xpath` | `xpaths.py` | `CompiledXpaths::BODY`, `::OVERALL_DISCARD`, etc. `XpathEngine::eval_text()` |
| `kawat-extract` | `main_extractor.py` + `baseline.py` | `extract_content()`, `extract_comments()`, `handle_textelem()`, `recover_wild_text()`, `baseline()` |
| `kawat-readability` | `readability_lxml.py` (512-line bundled fork) | `try_readability()` wrapping `dom_smoothie` |
| `kawat-justext` | `justext` external dep (called via `external.py`) | `ParagraphMaker`, `classify_paragraphs()`, `revise_paragraph_classification()`, stoplists for 29 languages |
| `kawat-metadata` | `metadata.py` + `json_metadata.py` | `extract_metadata()`, `extract_title()`, `extract_author()`, `extract_opengraph()`, `extract_json_ld()`, `extract_catstags()`, `extract_license()` |
| `kawat-dedup` | `deduplication.py` | `Simhash`, `DedupCache` (LRU), `content_fingerprint()`, `duplicate_test()` |
| `kawat-output` | `xml.py` + output parts of `core.py` | `xmltotxt()`, `build_json_output()`, `control_xml_output()`, `xmltocsv()`, `build_html_output()` |
| `kawat-cli` | `cli.py` + `cli_utils.py` | Binary with clap, stdin/file/URL input, batch processing |
| `htmldate-rs` | `htmldate` package (separate repo, 6 modules) | `find_date()`, `find_date_parsed()`, three-tier extraction |

### 3.3 Data flow

```
              ┌──────────────────────────────────────────────────────┐
              │                    kawat-core                         │
              │                                                      │
  HTML ──────►│  1. parse (scraper)                                  │
  string      │  2. lang check                                       │
              │  3. metadata ─────────► kawat-metadata ◄── htmldate-rs│
              │  4. user pruning                                      │
              │  5. tree_cleaning ────► kawat-html                    │
              │  6. convert_tags ─────► kawat-html                    │
              │  7. extract_comments ─► kawat-extract                 │
              │  8. cascade:                                          │
              │     8a. extract_content ► kawat-extract + kawat-xpath  │
              │     8b. compare ────────► kawat-readability            │
              │                         ► kawat-justext               │
              │     8c. baseline ──────► kawat-extract                │
              │  9. size checks                                       │
              │ 10. dedup ────────────► kawat-dedup                   │
              │ 11. lang filter                                       │
              │ 12. format ───────────► kawat-output                  │
              │                                                      │
              └──────────────────────────────────────────────────────┘
                          │
                          ▼
                   Document { body, metadata, comments, text }
```

### 3.4 Internal XML tree representation

trafilatura converts HTML to an internal XML-like tree using custom tags. kawat must replicate this internal representation. The tag set:

| Internal tag | HTML source | Purpose |
|---|---|---|
| `head` | h1-h6 | Headings |
| `p` | p, div (when promoted) | Paragraphs |
| `hi` | b, strong, em, i, mark | Highlighted/formatted text |
| `ref` | a | Links (when include_links) |
| `list` | ul, ol | Lists |
| `item` | li | List items |
| `quote` | blockquote, q | Quotations |
| `code` | code, pre | Code blocks |
| `del` | del, s, strike | Deleted text |
| `lb` | br | Line breaks |
| `graphic` | img | Images (when include_images) |
| `table`, `row`, `cell` | table, tr, td/th | Tables (when include_tables) |

The `body` element is the root of extracted content. `commentsbody` is a separate `body` for comments.

### 3.5 Scoring and heuristics detail

**Link density** (htmlprocessing.py:link_density_test):
```
link_density = len(link_text) / len(total_text)
if link_density > threshold → discard element
```
Thresholds vary: 0.5 for generic elements, stricter for precision mode.

**Readability scoring** (readability_lxml.py, 512 lines):
- Initializes all elements with score 0
- Adds/subtracts based on tag name, class, id patterns
- Positive: article, post, content, entry, text
- Negative: comment, meta, footer, sidebar, nav, combx, widget
- Propagates scores to parents (score/2 to grandparent)
- Selects candidate with highest score
- Cleans candidate by removing low-scoring children

**justext classification** (external dependency):
- `ParagraphMaker`: extracts paragraphs from HTML tree
- `classify_paragraphs(paragraphs, stoplist, length_low=50, length_high=150, stopwords_low=0.1, stopwords_high=0.2, max_link_density=0.25, no_headings=True)`:
  - short + low stopwords → boilerplate
  - long + high stopwords → good
  - considers link density
- `revise_paragraph_classification(paragraphs, max_heading_distance=150)`:
  - context-dependent reclassification using neighbor paragraphs

---

## 4. Test strategy

### 4.1 Golden test corpus

Generate expected outputs from Python trafilatura on a fixed set of HTML files:

```bash
# Generate golden outputs
python3 -c "
import trafilatura, json, os
for f in os.listdir('tests/golden/html/'):
    with open(f'tests/golden/html/{f}') as fh:
        html = fh.read()
    result = trafilatura.bare_extraction(html, output_format='python')
    if result:
        with open(f'tests/golden/expected/{f}.json', 'w') as out:
            json.dump(result.as_dict(), out, indent=2, default=str)
"
```

### 4.2 Test levels

| Level | What | How |
|---|---|---|
| Unit | Individual functions (link_density, parse_date_string, fingerprint) | Standard `#[test]` |
| Integration | Full extraction on golden corpus | Compare JSON output field-by-field |
| XPath parity | Each XPath expression against test HTML | Python lxml vs. sxd_xpath on same input |
| Benchmark | Throughput (docs/sec), memory (peak RSS) | `criterion` crate |
| Fuzz | Malformed HTML, edge cases | `cargo-fuzz` on `extract()` |

### 4.3 Acceptance criteria per phase

- **Phase 1**: main text extraction matches trafilatura on ≥70% of golden corpus files
- **Phase 2**: with fallbacks, matches ≥90%
- **Phase 3**: with metadata, matches ≥85% of metadata fields
- **Phase 4**: full output format parity for txt, json, xml

---

## 5. Performance targets

| Metric | trafilatura (Python) | kawat target |
|---|---|---|
| Single page extraction | ~5-10ms | <2ms |
| Memory per page | ~1-5MB peak | <500KB peak |
| Throughput (batch) | ~100-200 pages/sec/core | ~500-1000 pages/sec/core |

The `lol_html` streaming path (for html2txt/baseline) should be especially fast: its memory is tunable and independent of input size.
