# Changelog

All notable changes to this project will be documented in this file.

## v0.1.1 - 2026-03-24

### Added
- Comprehensive XPath evaluation engine with CSS selector fallback
- Benchmark suite for XPath performance testing
- Extensive test coverage (15+ tests) for all BODY_XPATH expressions
- Support for complex XPath patterns including `(article)[1]`

### Changed
- Updated dependencies: lru (0.12→0.16), quick-xml (0.37→0.39), scraper (0.22→0.26), criterion (0.5→0.8), reqwest (0.12→0.13)
- Improved TLS configuration in reqwest (rustls-tls → rustls)
- Enhanced error handling in XPath evaluation

### Fixed
- XPath fallback test failures for `(article)[1]` pattern
- CI/CD pipeline issues with deny.toml configuration
- Codecov upload failures for protected branches
- Security workflow permission handling

## v0.1.0 - 2026-03-22
- Initial workspace import of kawat crates (core, extract, html, xpath, metadata, output, CLI, etc.).
- Added date parsing regex fallback and chrono clock feature for htmldate-rs.
- Implemented `FromStr` for `OutputFormat` and hardened CLI format parsing.
- Resolved clippy warnings (unused imports/vars, format args) across crates.
- Added `.gitignore` and `.pre-commit-config.yaml` (fmt, clippy, cargo-audit) and ensured hooks pass.
