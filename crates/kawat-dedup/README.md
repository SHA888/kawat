# kawat-dedup

Deduplication for the kawat web content extraction library.

Implements Simhash-based near-duplicate detection with LRU caching.

## Features

- **Simhash fingerprinting**: Fast approximate duplicate detection
- **LRU cache**: Memory-efficient deduplication cache
- **Configurable thresholds**: Adjustable similarity thresholds
- **SHA-1 content hashing**: Exact duplicate detection fallback

## License

Apache-2.0
