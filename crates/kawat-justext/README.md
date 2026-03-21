# kawat-justext

Pure Rust port of the jusText algorithm for the kawat web content extraction library.

Classifies paragraphs in HTML as boilerplate or content based on text length, link density, and stopword frequency.

## Features

- **Paragraph classification**: Boilerplate vs. content detection
- **Stopword analysis**: Language-specific stopword frequency analysis
- **Link density detection**: Identify high-link-density boilerplate
- **Configurable parameters**: Adjustable thresholds for different use cases

## Reference

Pomikálek, J. (2011). Removing Boilerplate and Duplicate Content from Web Corpora.

## License

Apache-2.0
