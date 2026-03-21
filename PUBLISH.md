# Publishing Guide for kawat

This guide explains how to publish the kawat crates to crates.io.

## Overview

The kawat project is organized as a Rust workspace with multiple crates:

- **htmldate-rs**: Standalone date extraction library (can be published independently)
- **kawat-core**: Core extraction orchestrator (internal dependency)
- **kawat-html**: HTML tree cleaning (internal dependency)
- **kawat-xpath**: XPath evaluation engine (internal dependency)
- **kawat-extract**: Content extraction (internal dependency)
- **kawat-readability**: Readability fallback (internal dependency)
- **kawat-justext**: jusText algorithm port (internal dependency)
- **kawat-metadata**: Metadata extraction (internal dependency)
- **kawat-dedup**: Deduplication (internal dependency)
- **kawat-output**: Output formatting (internal dependency)
- **kawat**: Main facade crate (public API)
- **kawat-cli**: CLI binary (optional, not published)

## Publishing Order

Crates must be published in dependency order. Internal crates are published first, then the main `kawat` crate.

### Recommended Publishing Strategy

**Option A: Publish all crates (maximum modularity)**
- Users can depend on specific sub-crates if needed
- More maintenance burden

**Option B: Publish only `kawat` + `htmldate-rs` (recommended)**
- `kawat` is the main public API
- `htmldate-rs` is a standalone, reusable library
- Internal crates are published as private dependencies
- Simpler maintenance

## Publishing Steps (Option B - Recommended)

### Prerequisites

1. Create a crates.io account at https://crates.io
2. Generate an API token: https://crates.io/me
3. Login locally:
   ```bash
   cargo login
   ```

### Step 1: Publish htmldate-rs

```bash
cd crates/htmldate-rs
cargo publish
```

Wait 30 seconds for crates.io to index the crate.

### Step 2: Publish internal crates (in order)

Each internal crate must be published before crates that depend on it.

```bash
# Wait 30 seconds after each publish
sleep 30

cd crates/kawat-html
cargo publish

sleep 30

cd crates/kawat-xpath
cargo publish

sleep 30

cd crates/kawat-extract
cargo publish

sleep 30

cd crates/kawat-metadata
cargo publish

sleep 30

cd crates/kawat-dedup
cargo publish

sleep 30

cd crates/kawat-output
cargo publish

sleep 30

cd crates/kawat-readability
cargo publish

sleep 30

cd crates/kawat-justext
cargo publish

sleep 30

cd crates/kawat-core
cargo publish
```

### Step 3: Publish the main kawat crate

```bash
sleep 30

cd crates/kawat
cargo publish
```

## Automated Publishing Script

Use the provided `publish.sh` script to automate the process with proper delays:

```bash
bash ./scripts/publish.sh
```

The script will:
1. Check that you're logged in to crates.io
2. Publish each crate in the correct order
3. Wait 120 seconds between publishes to avoid rate limiting
4. Report success/failure for each crate

For retrying only failed crates:

```bash
bash ./scripts/publish-failed.sh
```

## Verifying Publishes

After publishing, verify that crates are available:

```bash
# Check htmldate-rs
cargo search htmldate-rs

# Check kawat
cargo search kawat
```

## Troubleshooting

### "crate already exists"
The crate version is already published. Increment the version in `Cargo.toml` and try again.

### "failed to fetch"
A dependency hasn't been published yet. Check the publishing order and ensure all dependencies are available on crates.io.

### "all dependencies must have a version specified when publishing"
Internal workspace dependencies must include explicit version specifications. The workspace `Cargo.toml` includes `version = "0.1.0"` for all internal crates alongside the `path` specification. This allows local development while supporting crates.io publishing.

### Rate limiting (429 Too Many Requests)
Crates.io enforces rate limits on new crate publications. If you hit this:
1. Wait until the time specified in the error message
2. The script includes 120-second delays between publishes to minimize this risk
3. Consider publishing in smaller batches if needed

### Publishing order matters
Crates must be published in dependency order:
1. `htmldate-rs` (no internal dependencies)
2. `kawat-html`, `kawat-xpath` (no internal dependencies)
3. `kawat-extract`, `kawat-metadata`, `kawat-dedup`, `kawat-output`, `kawat-readability`, `kawat-justext` (depend on above)
4. `kawat-core` (depends on all above)
5. `kawat` (main facade, depends on core)

## Updating Versions

To publish a new version:

1. Update version in workspace `Cargo.toml`:
   ```toml
   [workspace.package]
   version = "0.2.0"
   ```

2. Update `CHANGELOG.md` with changes

3. Commit and tag:
   ```bash
   git add -A
   git commit -m "Release v0.2.0"
   git tag -a v0.2.0 -m "v0.2.0"
   git push origin main --tags
   ```

4. Run the publishing script:
   ```bash
   bash ./publish.sh
   ```

## License

All crates are licensed under Apache-2.0.
