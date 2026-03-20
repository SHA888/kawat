//! Pre-compiled XPath expression groups from trafilatura's xpaths.py.
//!
//! These are the exact expressions used by trafilatura, preserved verbatim
//! for behavioral parity.

/// All pre-compiled XPath expression groups used in extraction.
pub struct CompiledXpaths;

impl CompiledXpaths {
    /// BODY_XPATH: Main content area candidates, tried in order (first match wins).
    /// 5 expressions from trafilatura xpaths.py lines 13-54.
    pub const BODY: &[&str] = &[
        // Expression 1: highly specific article/post/entry patterns
        r#".//*[self::article or self::div or self::main or self::section][@class="post" or @class="entry" or contains(@class, "post-text") or contains(@class, "post_text") or contains(@class, "post-body") or contains(@class, "post-entry") or contains(@class, "postentry") or contains(@class, "post-content") or contains(@class, "post_content") or contains(@class, "postcontent") or contains(@class, "postContent") or contains(@class, "post_inner_wrapper") or contains(@class, "article-text") or contains(@class, "articletext") or contains(@class, "articleText") or contains(@id, "entry-content") or contains(@class, "entry-content") or contains(@id, "article-content") or contains(@class, "article-content") or contains(@id, "article__content") or contains(@class, "article__content") or contains(@id, "article-body") or contains(@class, "article-body") or contains(@id, "article__body") or contains(@class, "article__body") or @itemprop="articleBody" or contains(translate(@id, "B", "b"), "articlebody") or contains(translate(@class, "B", "b"), "articlebody") or @id="articleContent" or contains(@class, "ArticleContent") or contains(@class, "page-content") or contains(@class, "text-content") or contains(@id, "body-text") or contains(@class, "body-text") or contains(@class, "article__container") or contains(@id, "art-content") or contains(@class, "art-content")][1]"#,
        // Expression 2: generic <article>
        "(.//article)[1]",
        // Expression 3: secondary patterns
        r#"(.//*[self::article or self::div or self::main or self::section][contains(@class, 'post-bodycopy') or contains(@class, 'storycontent') or contains(@class, 'story-content') or @class='postarea' or @class='art-postcontent' or contains(@class, 'theme-content') or contains(@class, 'blog-content') or contains(@class, 'section-content') or contains(@class, 'single-content') or contains(@class, 'single-post') or contains(@class, 'main-column') or contains(@class, 'wpb_text_column') or starts-with(@id, 'primary') or starts-with(@class, 'article ') or @class="text" or @id="article" or @class="cell" or @id="story" or @class="story" or contains(@class, "story-body") or contains(@id, "story-body") or contains(@class, "field-body") or contains(translate(@class, "FULTEX","fultex"), "fulltext") or @role='article'])[1]"#,
        // Expression 4: content-main / content-body
        r#"(.//*[self::article or self::div or self::main or self::section][contains(@id, "content-main") or contains(@class, "content-main") or contains(@class, "content_main") or contains(@id, "content-body") or contains(@class, "content-body") or contains(@id, "contentBody") or contains(@class, "content__body") or contains(translate(@id, "CM","cm"), "main-content") or contains(translate(@class, "CM","cm"), "main-content") or contains(translate(@class, "CP","cp"), "page-content") or @id="content" or @class="content"])[1]"#,
        // Expression 5: main element fallback
        r#"(.//*[self::article or self::div or self::section][starts-with(@class, "main") or starts-with(@id, "main") or starts-with(@role, "main")])[1]|(.//main)[1]"#,
    ];

    /// OVERALL_DISCARD_XPATH: Elements to remove (navigation, footers, ads, etc.)
    pub const OVERALL_DISCARD: &[&str] = &[
        r#".//*[self::div or self::item or self::list or self::p or self::section or self::span][contains(translate(@id, "F","f"), "footer") or contains(translate(@class, "F","f"), "footer") or contains(@id, "related") or contains(@class, "elated") or contains(@id|@class, "viral") or starts-with(@id|@class, "shar") or contains(@class, "share-") or contains(translate(@id, "S", "s"), "share") or contains(@id|@class, "social") or contains(@class, "sociable") or contains(@id|@class, "syndication") or starts-with(@id, "jp-") or starts-with(@id, "dpsp-content") or contains(@class, "embedded") or contains(@class, "embed") or contains(@id|@class, "newsletter") or contains(@class, "subnav") or contains(@id|@class, "cookie") or contains(@id|@class, "tags") or contains(@class, "tag-list") or contains(@id|@class, "sidebar") or contains(@id|@class, "banner") or contains(@class, "bar") or contains(@class, "meta") or contains(@id, "menu") or contains(@class, "menu") or contains(translate(@id, "N", "n"), "nav") or contains(translate(@role, "N", "n"), "nav") or starts-with(@class, "nav") or contains(@class, "avigation") or contains(@class, "navbar") or contains(@class, "navbox") or starts-with(@class, "post-nav") or contains(@id|@class, "breadcrumb") or contains(@id|@class, "bread-crumb") or contains(@id|@class, "author") or contains(@id|@class, "button") or contains(translate(@class, "B", "b"), "byline") or contains(@class, "rating") or contains(@class, "widget") or contains(@class, "attachment") or contains(@class, "timestamp") or contains(@class, "user-info") or contains(@class, "user-profile") or contains(@class, "-ad-") or contains(@class, "-icon") or contains(@class, "article-infos") or contains(@class, "nfoline") or contains(@data-component, "MostPopularStories") or contains(@class, "outbrain") or contains(@class, "taboola") or contains(@class, "criteo") or contains(@class, "options") or contains(@class, "expand") or contains(@class, "consent") or contains(@class, "modal-content") or contains(@class, " ad ") or contains(@class, "permission") or contains(@class, "next-") or contains(@class, "-stories") or contains(@class, "most-popular") or contains(@class, "mol-factbox") or starts-with(@class, "ZendeskForm") or contains(@id|@class, "message-container") or contains(@class, "yin") or contains(@class, "zlylin") or contains(@class, "xg1") or contains(@id, "bmdh") or contains(@class, "slide") or contains(@class, "viewport") or @data-lp-replacement-content or contains(@id, "premium") or contains(@class, "overlay") or contains(@class, "paid-content") or contains(@class, "paidcontent") or contains(@class, "obfuscated") or contains(@class, "blurred")]"#,
        r#".//*[@class="comments-title" or contains(@class, "comments-title") or contains(@class, "nocomments") or starts-with(@id|@class, "reply-") or contains(@class, "-reply-") or contains(@class, "message") or contains(@id, "reader-comments") or contains(@id, "akismet") or contains(@class, "akismet") or contains(@class, "suggest-links") or starts-with(@class, "hide-") or contains(@class, "-hide-") or contains(@class, "hide-print") or contains(@id|@style, "hidden") or contains(@class, " hidden") or contains(@class, " hide") or contains(@class, "noprint") or contains(@style, "display:none") or contains(@style, "display: none") or @aria-hidden="true" or contains(@class, "notloaded")]"#,
    ];

    /// COMMENTS_XPATH: Sections likely to contain user comments.
    pub const COMMENTS: &[&str] = &[
        r#".//*[self::div or self::list or self::section][contains(@id|@class, 'commentlist') or contains(@class, 'comment-page') or contains(@id|@class, 'comment-list') or contains(@class, 'comments-content') or contains(@class, 'post-comments')]"#,
        r#".//*[self::div or self::section or self::list][starts-with(@id|@class, 'comments') or starts-with(@class, 'Comments') or starts-with(@id|@class, 'comment-') or contains(@class, 'article-comments')]"#,
        r#".//*[self::div or self::section or self::list][starts-with(@id, 'comol') or starts-with(@id, 'disqus_thread') or starts-with(@id, 'dsq-comments')]"#,
        r#".//*[self::div or self::section][starts-with(@id, 'social') or contains(@class, 'comment')]"#,
    ];

    /// PRECISION_DISCARD_XPATH: Additional elements removed in precision mode.
    pub const PRECISION_DISCARD: &[&str] = &[
        ".//header",
        r#".//*[self::div or self::item or self::list or self::p or self::section or self::span][contains(@id|@class, "bottom") or contains(@id|@class, "link") or contains(@style, "border")]"#,
    ];

    /// TEASER_DISCARD_XPATH: Teaser elements removed except in recall mode.
    pub const TEASER_DISCARD: &[&str] = &[
        r#".//*[self::div or self::item or self::list or self::p or self::section or self::span][contains(translate(@id, "T", "t"), "teaser") or contains(translate(@class, "T", "t"), "teaser")]"#,
    ];
}
