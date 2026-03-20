//! Tag constants mirroring trafilatura settings.py.

/// Tags to completely remove (element and children).
/// 44 tags from trafilatura settings.py MANUALLY_CLEANED.
pub static MANUALLY_CLEANED: &[&str] = &[
    "aside", "embed", "footer", "form", "head", "iframe", "menu", "object", "script", "applet",
    "audio", "canvas", "figure", "map", "picture", "svg", "video", "area", "blink", "button",
    "datalist", "dialog", "frame", "frameset", "fieldset", "link", "input", "ins", "label",
    "legend", "marquee", "math", "menuitem", "nav", "noindex", "noscript", "optgroup", "option",
    "output", "param", "progress", "rp", "rt", "rtc", "select", "source", "style", "track",
    "textarea", "time", "use",
];

/// Tags to strip (remove tag but keep children/text).
/// 20 tags from trafilatura settings.py MANUALLY_STRIPPED.
pub static MANUALLY_STRIPPED: &[&str] = &[
    "abbr", "acronym", "address", "bdi", "bdo", "big", "cite", "data", "dfn", "font", "hgroup",
    "img", "ins", "mark", "meta", "ruby", "small", "tbody", "template", "tfoot", "thead",
];

/// Internal tag catalog used for extraction.
/// 10 tags from trafilatura settings.py TAG_CATALOG.
pub static TAG_CATALOG: &[&str] = &[
    "blockquote",
    "code",
    "del",
    "head",
    "hi",
    "lb",
    "list",
    "p",
    "pre",
    "quote",
];

/// Tags that should be removed if empty.
pub static CUT_EMPTY_ELEMS: &[&str] = &[
    "article",
    "b",
    "blockquote",
    "dd",
    "div",
    "dt",
    "em",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "i",
    "li",
    "main",
    "p",
    "pre",
    "q",
    "section",
    "span",
    "strong",
];
