use criterion::{Criterion, black_box, criterion_group, criterion_main};
use kawat_xpath::{CompiledXpaths, XpathEngine};

fn benchmark_xpath_vs_css(c: &mut Criterion) {
    let html = r#"
        <html>
            <head><title>Test Page</title></head>
            <body>
                <header>
                    <nav class="navbar">
                        <ul>
                            <li><a href="/">Home</a></li>
                            <li><a href="/about">About</a></li>
                        </ul>
                    </nav>
                </header>
                <main class="content-main">
                    <article class="post">
                        <h1>Article Title</h1>
                        <p>First paragraph of content.</p>
                        <p>Second paragraph of content.</p>
                        <div class="sidebar">
                            <h3>Related Links</h3>
                            <ul>
                                <li><a href="/link1">Link 1</a></li>
                                <li><a href="/link2">Link 2</a></li>
                            </ul>
                        </div>
                    </article>
                    <footer>
                        <p>Footer content</p>
                    </footer>
                </main>
            </body>
        </html>
    "#;

    // Benchmark sxd_xpath evaluation
    c.bench_function("sxd_xpath_article", |b| {
        b.iter(|| {
            let result =
                XpathEngine::eval_text(black_box(html), black_box(CompiledXpaths::BODY[1]));
            black_box(result)
        })
    });

    // Benchmark CSS selector equivalent
    c.bench_function("css_selector_article", |b| {
        b.iter(|| {
            let html_doc = scraper::Html::parse_document(black_box(html));
            let selector = scraper::Selector::parse("article").unwrap();
            let elements: Vec<String> = html_doc
                .select(&selector)
                .map(|el| el.text().collect())
                .collect();
            black_box(elements)
        })
    });

    // Benchmark complex XPath expression
    c.bench_function("sxd_xpath_complex", |b| {
        b.iter(|| {
            let result =
                XpathEngine::eval_text(black_box(html), black_box(CompiledXpaths::BODY[0]));
            black_box(result)
        })
    });

    // Benchmark discard XPath
    c.bench_function("sxd_xpath_discard", |b| {
        b.iter(|| {
            let result = XpathEngine::eval_text(
                black_box(html),
                black_box(CompiledXpaths::OVERALL_DISCARD[0]),
            );
            black_box(result)
        })
    });

    // Benchmark CSS selector for discard
    c.bench_function("css_selector_discard", |b| {
        b.iter(|| {
            let html_doc = scraper::Html::parse_document(black_box(html));
            let selector = scraper::Selector::parse(".navbar, .sidebar, footer").unwrap();
            let elements: Vec<String> = html_doc
                .select(&selector)
                .map(|el| el.text().collect())
                .collect();
            black_box(elements)
        })
    });
}

fn benchmark_multiple_evaluations(c: &mut Criterion) {
    let html = r#"
        <html>
            <body>
                <article>
                    <h1>Performance Test</h1>
                    <p>Testing multiple evaluations</p>
                </article>
                <div class="post">
                    <h2>Another Post</h2>
                    <p>More content</p>
                </div>
                <main>
                    <h3>Main Content</h3>
                    <p>Main section content</p>
                </main>
            </body>
        </html>
    "#;

    // Benchmark evaluating multiple XPath expressions
    c.bench_function("multiple_xpath_evals", |b| {
        b.iter(|| {
            let xpaths = [
                CompiledXpaths::BODY[1], // article
                CompiledXpaths::BODY[4], // main
                ".//*[@class=\"post\"]",
            ];

            for xpath in xpaths {
                let _ = XpathEngine::eval_text(black_box(html), black_box(xpath));
            }
        })
    });

    // Benchmark evaluating multiple CSS selectors
    c.bench_function("multiple_css_evals", |b| {
        b.iter(|| {
            let selectors = ["article", "main", ".post"];
            let html_doc = scraper::Html::parse_document(black_box(html));

            for selector_str in selectors {
                let selector = scraper::Selector::parse(selector_str).unwrap();
                let elements: Vec<String> = html_doc
                    .select(&selector)
                    .map(|el| el.text().collect())
                    .collect();
                black_box(elements);
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_xpath_vs_css,
    benchmark_multiple_evaluations
);
criterion_main!(benches);
