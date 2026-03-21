# kawat-cli

Command-line interface for the kawat web content extraction library.

Extract main text, metadata, and comments from web pages via the command line.

## Installation

Build from source:

```bash
cargo install --path crates/kawat-cli
```

Or from crates.io (when published):

```bash
cargo install kawat-cli
```

## Usage

### Extract from URL

```bash
kawat -u "https://example.org/article"
```

### Extract from file

```bash
kawat -i page.html
```

### Extract from stdin

```bash
curl -s https://example.org/article | kawat
```

### Output formats

```bash
# Markdown (default: txt)
kawat -u "https://example.org/article" -f markdown

# JSON with metadata
kawat -i page.html -f json --with-metadata

# XML
kawat -i page.html -f xml

# CSV
kawat -i page.html -f csv
```

### Options

```
USAGE:
    kawat [OPTIONS]

OPTIONS:
    -u, --url <URL>
            URL to extract content from

    -i, --input <INPUT>
            Input HTML file

    -f, --format <FORMAT>
            Output format [default: txt]
            Possible values: txt, markdown, json, xml, xmltei, csv, html

    --fast
            Use faster heuristics (skip fallback algorithms)

    --precision
            Favor precision over recall

    --recall
            Favor recall over precision

    --with-metadata
            Include metadata in output

    --no-comments
            Exclude comments

    --no-tables
            Exclude tables

    --links
            Include links

    --formatting
            Include formatting

    --target-language <LANGUAGE>
            Target language (ISO 639-1, e.g. "en", "id")

    --deduplicate
            Deduplicate content

    -h, --help
            Print help information

    -V, --version
            Print version information
```

## Examples

### Extract and save to file

```bash
kawat -u "https://example.org/article" > article.txt
```

### Extract with metadata as JSON

```bash
kawat -u "https://example.org/article" -f json --with-metadata | jq .
```

### Extract from local file with precision mode

```bash
kawat -i page.html --precision -f markdown > article.md
```

### Extract with language filtering

```bash
kawat -u "https://example.org/article" --target-language en
```

### Batch processing

```bash
for url in $(cat urls.txt); do
    kawat -u "$url" -f json --with-metadata >> articles.jsonl
done
```

## License

Apache-2.0
