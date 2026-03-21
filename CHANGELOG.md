# Changelog

All notable changes to this project will be documented in this file.

## v0.1.0 - 2026-03-22
- Initial workspace import of kawat crates (core, extract, html, xpath, metadata, output, CLI, etc.).
- Added date parsing regex fallback and chrono clock feature for htmldate-rs.
- Implemented `FromStr` for `OutputFormat` and hardened CLI format parsing.
- Resolved clippy warnings (unused imports/vars, format args) across crates.
- Added `.gitignore` and `.pre-commit-config.yaml` (fmt, clippy, cargo-audit) and ensured hooks pass.
