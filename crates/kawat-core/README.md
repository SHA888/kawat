# kawat-core

Core extraction orchestrator for the kawat web content extraction library.

Implements the full trafilatura extraction cascade with multi-algorithm fallback:

1. **HTML parsing & metadata extraction**
2. **Tree cleaning & tag normalization**
3. **Comment extraction**
4. **Content extraction** (BODY_XPATH → readability → justext → baseline)
5. **Size checks & deduplication**
6. **Language filtering & output formatting**

## Features

- **Extraction cascade**: Multi-algorithm fallback for robust content extraction
- **Configurable focus modes**: Balanced, Precision, or Recall
- **Metadata support**: Title, author, date, URL, categories, tags, license
- **Comment extraction**: Separate user comments from main content
- **Deduplication**: Simhash + LRU cache for duplicate detection
- **Language detection**: Optional language filtering (lingua crate)

## License

Apache-2.0
