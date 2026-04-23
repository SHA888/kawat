#!/usr/bin/env python3
"""Generate golden test fixtures: HTML files and expected trafilatura TXT outputs.

NOTE: This script generates the following artifacts which are committed to git:
- fixtures/*.html — synthetic HTML test inputs
- expected/*.txt — expected TXT output from Python trafilatura
- metadata.json — test manifest mapping inputs to expected outputs

If trafilatura version changes, re-run this script and commit updated expecteds.
"""

import json
import os
import trafilatura

FIXTURES_DIR = os.path.dirname(os.path.abspath(__file__))
HTML_DIR = os.path.join(FIXTURES_DIR, "fixtures")
EXPECTED_DIR = os.path.join(FIXTURES_DIR, "expected")
METADATA_FILE = os.path.join(FIXTURES_DIR, "metadata.json")

os.makedirs(HTML_DIR, exist_ok=True)
os.makedirs(EXPECTED_DIR, exist_ok=True)


FIXTURES = {
    "01_simple_article": """
<!DOCTYPE html>
<html><head><title>Simple Article</title></head>
<body>
<article>
<h1>Hello World</h1>
<p>This is a simple paragraph with some text.</p>
<p>Another paragraph follows here.</p>
</article>
</body></html>
""",

    "02_article_with_meta": """
<!DOCTYPE html>
<html><head>
<title>Article With Metadata</title>
<meta name="author" content="Jane Doe">
<meta name="date" content="2024-01-15">
</head><body>
<article>
<h1>Deep Dive into Rust</h1>
<p>Rust is a systems programming language focused on safety and performance.</p>
<p>Ownership is Rust's most unique feature.</p>
</article>
</body></html>
""",

    "03_multiple_sections": """
<!DOCTYPE html>
<html><head><title>Multi-Section Page</title></head>
<body>
<nav><a href="#">Home</a> <a href="#">About</a></nav>
<main>
<article>
<h1>Main Article Title</h1>
<p>First paragraph of the main content.</p>
<h2>Section One</h2>
<p>Content in section one with more details.</p>
<h2>Section Two</h2>
<p>Content in section two continues the story.</p>
</article>
</main>
<footer>Copyright 2024</footer>
</body></html>
""",

    "04_with_lists": """
<!DOCTYPE html>
<html><head><title>Lists Article</title></head>
<body>
<article>
<h1>Top Programming Languages</h1>
<p>Here are some popular languages:</p>
<ul>
<li>Rust - systems programming</li>
<li>Python - data science</li>
<li>TypeScript - web development</li>
</ul>
<p>Each has its own strengths.</p>
<ol>
<li>Learn the basics</li>
<li>Practice daily</li>
<li>Build projects</li>
</ol>
</article>
</body></html>
""",

    "05_with_table": """
<!DOCTYPE html>
<html><head><title>Table Article</title></head>
<body>
<article>
<h1>Performance Comparison</h1>
<p>Results from benchmark tests:</p>
<table>
<tr><th>Language</th><th>Speed</th><th>Memory</th></tr>
<tr><td>Rust</td><td>Fast</td><td>Low</td></tr>
<tr><td>Python</td><td>Slow</td><td>High</td></tr>
<tr><td>Go</td><td>Medium</td><td>Medium</td></tr>
</table>
<p>The tests were run on standard hardware.</p>
</article>
</body></html>
""",

    "06_with_quotes": """
<!DOCTYPE html>
<html><head><title>Quotes Article</title></head>
<body>
<article>
<h1>Famous Quotes</h1>
<p>Some wisdom from great thinkers:</p>
<blockquote>
<p>The only way to do great work is to love what you do.</p>
<cite>Steve Jobs</cite>
</blockquote>
<p>And another one:</p>
<blockquote>
<p>Stay hungry, stay foolish.</p>
</blockquote>
<p>These quotes inspire action.</p>
</article>
</body></html>
""",

    "07_with_code": """
<!DOCTYPE html>
<html><head><title>Code Article</title></head>
<body>
<article>
<h1>Rust Basics</h1>
<p>Here is a simple function:</p>
<pre><code>fn hello() {
    println!("Hello, world!");
}</code></pre>
<p>And another example with variables:</p>
<pre><code>let x = 5;
let y = 10;
let sum = x + y;</code></pre>
<p>Variables are immutable by default.</p>
</article>
</body></html>
""",

    "08_nested_divs": """
<!DOCTYPE html>
<html><head><title>Nested Article</title></head>
<body>
<div class="wrapper">
<div class="container">
<div class="content">
<article>
<h1>Deeply Nested Content</h1>
<p>This content is wrapped in many divs.</p>
<p>But the article tag should still be found.</p>
</article>
</div>
</div>
</div>
</body></html>
""",

    "09_no_article_tag": """
<!DOCTYPE html>
<html><head><title>No Article Tag</title></head>
<body>
<div class="main">
<h1>Main Content Title</h1>
<p>This page does not use article tags.</p>
<p>The content is just in regular divs and paragraphs.</p>
<p>Extraction should still work using other heuristics.</p>
</div>
</body></html>
""",

    "10_with_sidebar": """
<!DOCTYPE html>
<html><head><title>Sidebar Page</title></head>
<body>
<div class="layout">
<aside>
<h3>Related Links</h3>
<ul><li><a href="#">Link 1</a></li><li><a href="#">Link 2</a></li></ul>
</aside>
<article>
<h1>Main Content</h1>
<p>This is the primary article content.</p>
<p>Sidebar content should be filtered out.</p>
</article>
</div>
</body></html>
""",

    "11_short_content": """
<!DOCTYPE html>
<html><head><title>Short Page</title></head>
<body>
<article>
<h1>Brief Note</h1>
<p>This is a very short article with minimal content.</p>
</article>
</body></html>
""",

    "12_long_article": """
<!DOCTYPE html>
<html><head><title>Long Article</title></head>
<body>
<article>
<h1>Comprehensive Guide</h1>
<p>This is the introduction to a very long article.</p>
<h2>First Chapter</h2>
<p>Chapter one covers the fundamentals of the topic in detail.</p>
<p>It continues with examples and explanations.</p>
<h2>Second Chapter</h2>
<p>Chapter two dives deeper into advanced concepts.</p>
<p>More content here to make the article substantial.</p>
<h2>Third Chapter</h2>
<p>Chapter three wraps up with conclusions.</p>
<p>Final thoughts and recommendations.</p>
</article>
</body></html>
""",

    "13_with_comments": """
<!DOCTYPE html>
<html><head><title>Comments Article</title></head>
<body>
<article>
<h1>Article With Discussion</h1>
<p>This article has content worth discussing.</p>
<p>Readers often share their thoughts below.</p>
</article>
<div class="comments">
<h3>Comments</h3>
<div class="comment"><p>Great article!</p></div>
<div class="comment"><p>Very helpful, thanks.</p></div>
</div>
</body></html>
""",

    "14_boilerplate_heavy": """
<!DOCTYPE html>
<html><head><title>Boilerplate Page</title></head>
<body>
<header><nav><a href="#">Home</a> <a href="#">About</a> <a href="#">Contact</a></nav></header>
<div class="ads"><p>Advertisement content here.</p></div>
<article>
<h1>Real Article Content</h1>
<p>This is the actual content that should be extracted.</p>
<p>Everything else is noise and boilerplate.</p>
</article>
<div class="related"><p>Related articles list.</p></div>
<footer><p>Site footer with links and copyright.</p></footer>
</body></html>
""",

    "15_links_everywhere": """
<!DOCTYPE html>
<html><head><title>Links Article</title></head>
<body>
<article>
<h1>Resource Guide</h1>
<p>This article contains many <a href="https://example.com">useful links</a>.</p>
<p>Here is <a href="https://rust-lang.org">Rust</a> and <a href="https://python.org">Python</a>.</p>
<p>Links should be handled appropriately.</p>
</article>
</body></html>
""",

    "16_mixed_formatting": """
<!DOCTYPE html>
<html><head><title>Formatting Article</title></head>
<body>
<article>
<h1>Text Formatting</h1>
<p>This paragraph has <strong>bold</strong> and <em>italic</em> text.</p>
<p>Also <b>more bold</b> and <i>more italic</i> text here.</p>
<p>And <u>underlined</u> text as well.</p>
<p>Formatting should be preserved or stripped cleanly.</p>
</article>
</body></html>
""",

    "17_image_heavy": """
<!DOCTYPE html>
<html><head><title>Images Article</title></head>
<body>
<article>
<h1>Photo Gallery</h1>
<p>This article includes several images.</p>
<img src="photo1.jpg" alt="First photo">
<p>Description of the first photo.</p>
<img src="photo2.jpg" alt="Second photo">
<p>Description of the second photo.</p>
<p>Images with alt text should be handled.</p>
</article>
</body></html>
""",

    "18_json_ld_article": """
<!DOCTYPE html>
<html><head>
<title>JSON-LD Article</title>
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "NewsArticle",
  "headline": "JSON-LD Headline",
  "author": {"@type": "Person", "name": "John Smith"},
  "datePublished": "2024-06-01"
}
</script>
</head><body>
<article>
<h1>Structured Data Article</h1>
<p>This page includes JSON-LD structured data.</p>
<p>The extractor may use this for metadata.</p>
</article>
</body></html>
""",

    "19_minimal_html": """
<html><body>
<h1>Minimal Page</h1>
<p>No doctype, no head tags, just raw HTML.</p>
<p>Very simple structure.</p>
</body></html>
""",

    "20_emptyish": """
<!DOCTYPE html>
<html><head><title>Mostly Empty</title></head>
<body>
<nav>Navigation only</nav>
<div class="sidebar">Sidebar content</div>
<footer>Footer stuff</footer>
</body></html>
""",
}


def generate():
    metadata = []
    for name, html in FIXTURES.items():
        html_path = os.path.join(HTML_DIR, f"{name}.html")
        with open(html_path, "w") as f:
            f.write(html.strip())

        # Generate expected output with trafilatura (no metadata, txt format)
        expected = trafilatura.extract(html, output_format="txt", with_metadata=False) or ""
        expected_path = os.path.join(EXPECTED_DIR, f"{name}.txt")
        with open(expected_path, "w") as f:
            f.write(expected)

        # Also generate with metadata for comparison
        expected_with_meta = trafilatura.extract(html, output_format="txt", with_metadata=True) or ""
        meta_path = os.path.join(EXPECTED_DIR, f"{name}_meta.txt")
        with open(meta_path, "w") as f:
            f.write(expected_with_meta)

        metadata.append({
            "name": name,
            "html_file": f"fixtures/{name}.html",
            "expected_file": f"expected/{name}.txt",
            "expected_with_meta": f"expected/{name}_meta.txt",
            "expected_len": len(expected),
            "expected_meta_len": len(expected_with_meta),
        })

    with open(METADATA_FILE, "w") as f:
        json.dump(metadata, f, indent=2)

    print(f"Generated {len(FIXTURES)} fixtures in {HTML_DIR}")
    print(f"Generated {len(FIXTURES)} expected outputs in {EXPECTED_DIR}")
    print(f"Wrote metadata to {METADATA_FILE}")


if __name__ == "__main__":
    generate()
