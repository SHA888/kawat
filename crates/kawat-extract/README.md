# kawat-extract

Main content and comment extraction for the kawat web content extraction library.

Implements the core extraction logic from trafilatura's main_extractor.py.

## Features

- **Content extraction**: BODY_XPATH iteration with first-match-wins strategy
- **Comment extraction**: Separate user comments from main content
- **Text element handling**: Specialized handlers for titles, paragraphs, tables, lists, quotes, code blocks
- **Wild text recovery**: Fallback extraction for unstructured text content

## License

Apache-2.0
