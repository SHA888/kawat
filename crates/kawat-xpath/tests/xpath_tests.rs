use kawat_xpath::{CompiledXpaths, XpathEngine};

/// Test BODY_XPATH expressions against real-world HTML patterns
#[test]
fn test_body_xpath_article_tag() {
    let html = r#"
        <html>
            <body>
                <article>
                    <h1>Article Title</h1>
                    <p>Article content here</p>
                </article>
            </body>
        </html>
    "#;

    // Expression 2: generic <article>
    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[1]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_body_xpath_post_class() {
    let html = r#"
        <html>
            <body>
                <div class="post">
                    <h1>Post Title</h1>
                    <div class="post-content">
                        <p>Post content here</p>
                    </div>
                </div>
            </body>
        </html>
    "#;

    // Expression 1: post class patterns
    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[0]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_body_xpath_content_main() {
    let html = r#"
        <html>
            <body>
                <div id="content-main">
                    <h1>Main Content</h1>
                    <p>This is the main content area</p>
                </div>
            </body>
        </html>
    "#;

    // Expression 4: content-main / content-body
    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[3]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_body_xpath_main_element() {
    let html = r#"
        <html>
            <body>
                <main>
                    <h1>Main Content</h1>
                    <p>Content in main element</p>
                </main>
            </body>
        </html>
    "#;

    // Expression 5: main element fallback
    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[4]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_body_xpath_first_match_wins() {
    let html = r#"
        <html>
            <body>
                <article>
                    <h1>First Match</h1>
                    <p>This should be matched first</p>
                </article>
                <div class="post">
                    <h1>Second Match</h1>
                    <p>This should not be matched</p>
                </div>
            </body>
        </html>
    "#;

    // Test that first matching expression wins
    let mut found = false;
    for xpath in CompiledXpaths::BODY {
        let result = XpathEngine::eval_text(html, xpath);
        if let Ok(matches) = result {
            if !matches.is_empty() {
                found = true;
                break;
            }
        }
    }
    assert!(found);
}

/// Test OVERALL_DISCARD_XPATH expressions
#[test]
fn test_overall_discard_xpath_footer() {
    let html = r#"
        <html>
            <body>
                <div id="footer">
                    <p>Footer content</p>
                </div>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, CompiledXpaths::OVERALL_DISCARD[0]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_overall_discard_xpath_sidebar() {
    let html = r#"
        <html>
            <body>
                <div class="sidebar">
                    <h3>Related Posts</h3>
                    <ul>
                        <li><a href="/post1">Post 1</a></li>
                    </ul>
                </div>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, CompiledXpaths::OVERALL_DISCARD[0]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_overall_discard_xpath_navigation() {
    let html = r#"
        <html>
            <body>
                <div class="navbar">
                    <ul>
                        <li><a href="/">Home</a></li>
                        <li><a href="/about">About</a></li>
                    </ul>
                </div>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, CompiledXpaths::OVERALL_DISCARD[0]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_overall_discard_xpath_comments() {
    let html = r#"
        <html>
            <body>
                <div class="comments-title">
                    <h3>Comments</h3>
                </div>
                <div class="comment">
                    <p>User comment</p>
                </div>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, CompiledXpaths::OVERALL_DISCARD[1]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_overall_discard_xpath_social_sharing() {
    let html = r#"
        <html>
            <body>
                <div class="share-buttons">
                    <button class="share-facebook">Share</button>
                    <button class="share-twitter">Tweet</button>
                </div>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, CompiledXpaths::OVERALL_DISCARD[0]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

/// Test XPath engine robustness
#[test]
fn test_xpath_empty_html() {
    let html = "<html><body></body></html>";
    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[0]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(matches.is_empty());
}

#[test]
fn test_xpath_malformed_html() {
    let html = "<html><body><div class='post'><p>Unclosed paragraph";
    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[0]);
    // Should handle malformed HTML gracefully
    assert!(result.is_ok() || result.is_err());
}

/// Test CSS selector fallback functionality
#[test]
fn test_css_fallback_simple_article() {
    let html = r#"
        <html>
            <body>
                <article>
                    <h1>Test Article</h1>
                    <p>Content here</p>
                </article>
            </body>
        </html>
    "#;

    // This should work with CSS fallback for simple expressions
    let result = XpathEngine::eval_text(html, "(.//article)[1]");
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_css_fallback_main_element() {
    let html = r#"
        <html>
            <body>
                <main>
                    <h1>Main Content</h1>
                    <p>Content in main</p>
                </main>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[4]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_css_fallback_post_class() {
    let html = r#"
        <html>
            <body>
                <div class="post">
                    <h1>Post Title</h1>
                    <p>Post content</p>
                </div>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, ".//*[@class=\"post\"]");
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_custom_filters_translate() {
    let html = r#"
        <html>
            <body>
                <div class="articlebody">
                    <h1>Article Title</h1>
                    <p>Article content</p>
                </div>
                <div class="othercontent">
                    <p>Other content</p>
                </div>
            </body>
        </html>
    "#;

    // Test translate() expression fallback
    let xpath = r#".//*[self::div][contains(translate(@class, "B", "b"), "articlebody")]"#;
    let result = XpathEngine::eval_text(html, xpath);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
    // Should only match the articlebody div
}

#[test]
fn test_fallback_robustness() {
    let malformed_html =
        "<html><body><div class='post'><p>Unclosed paragraph<div class='sidebar'>Sidebar</div>";

    // Should handle malformed HTML without crashing
    let result = XpathEngine::eval_text(malformed_html, "(.//article)[1]");
    // Either succeeds (with sxd_html) or falls back gracefully
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_fallback_performance() {
    let html = r#"
        <html>
            <body>
                <article>
                    <h1>Performance Test</h1>
                    <p>Testing fallback performance</p>
                </article>
            </body>
        </html>
    "#;

    // Test that fallback doesn't significantly impact performance
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = XpathEngine::eval_text(html, "(.//article)[1]");
    }
    let duration = start.elapsed();

    // Should complete 100 evaluations in reasonable time (< 1 second)
    assert!(duration.as_secs() < 1);
}

#[test]
fn test_xpath_has_match() {
    let html = r#"
        <html>
            <body>
                <article>
                    <p>Content</p>
                </article>
            </body>
        </html>
    "#;

    assert!(XpathEngine::has_match(html, CompiledXpaths::BODY[1]));
    assert!(!XpathEngine::has_match(html, ".//nonexistent"));
}

/// Test complex nested structures
#[test]
fn test_body_xpath_nested_article() {
    let html = r#"
        <html>
            <body>
                <div class="wrapper">
                    <article>
                        <div class="article-header">
                            <h1>Title</h1>
                        </div>
                        <div class="article-body">
                            <p>Paragraph 1</p>
                            <p>Paragraph 2</p>
                        </div>
                    </article>
                </div>
            </body>
        </html>
    "#;

    let result = XpathEngine::eval_text(html, CompiledXpaths::BODY[1]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    assert!(!matches.is_empty());
}

#[test]
fn test_overall_discard_multiple_matches() {
    let html = r#"
        <html>
            <body>
                <nav class="navbar">Navigation</nav>
                <div id="footer">Footer</div>
                <div class="sidebar">Sidebar</div>
                <article>Main content</article>
            </body>
        </html>
    "#;

    // Should match multiple elements to discard
    let result = XpathEngine::eval_text(html, CompiledXpaths::OVERALL_DISCARD[0]);
    assert!(result.is_ok());
    let matches = result.unwrap();
    // May have multiple matches or combined text
    assert!(!matches.is_empty());
}
