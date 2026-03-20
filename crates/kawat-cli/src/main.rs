use anyhow::Result;
use clap::Parser;
use kawat_core::config::{ExtractorOptions, Focus};
use kawat_output::OutputFormat;
use std::str::FromStr;

/// kawat: extract main text content from web pages
///
/// Inspired by trafilatura. kawat is Indonesian for "wire" —
/// the same metaphor as trafilatura (Italian: wire drawing).
#[derive(Parser, Debug)]
#[command(name = "kawat", version, about)]
struct Cli {
    /// URL to extract content from
    #[arg(short, long)]
    url: Option<String>,

    /// Input HTML file
    #[arg(short, long)]
    input: Option<String>,

    /// Output format
    #[arg(short = 'f', long, default_value = "txt")]
    format: String,

    /// Use faster heuristics (skip fallback algorithms)
    #[arg(long)]
    fast: bool,

    /// Favor precision over recall
    #[arg(long)]
    precision: bool,

    /// Favor recall over precision
    #[arg(long)]
    recall: bool,

    /// Include metadata in output
    #[arg(long)]
    with_metadata: bool,

    /// Exclude comments
    #[arg(long)]
    no_comments: bool,

    /// Exclude tables
    #[arg(long)]
    no_tables: bool,

    /// Include links
    #[arg(long)]
    links: bool,

    /// Include formatting
    #[arg(long)]
    formatting: bool,

    /// Target language (ISO 639-1, e.g. "en", "id")
    #[arg(long)]
    target_language: Option<String>,

    /// Deduplicate content
    #[arg(long)]
    deduplicate: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let format = OutputFormat::from_str(&cli.format).unwrap_or_else(|_| {
        eprintln!("Unknown format: {}. Using txt.", cli.format);
        OutputFormat::Txt
    });

    let options = ExtractorOptions {
        format,
        fast: cli.fast,
        focus: if cli.precision {
            Focus::Precision
        } else if cli.recall {
            Focus::Recall
        } else {
            Focus::Balanced
        },
        comments: !cli.no_comments,
        tables: !cli.no_tables,
        links: cli.links,
        formatting: cli.formatting,
        with_metadata: cli.with_metadata,
        target_language: cli.target_language,
        dedup: cli.deduplicate,
        ..Default::default()
    };

    let html = if let Some(url) = &cli.url {
        kawat::fetch_url(url)?
    } else if let Some(path) = &cli.input {
        std::fs::read_to_string(path)?
    } else {
        // Read from stdin
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    };

    match kawat::extract(&html, &options) {
        Ok(text) => print!("{text}"),
        Err(e) => eprintln!("Extraction failed: {e}"),
    }

    Ok(())
}
