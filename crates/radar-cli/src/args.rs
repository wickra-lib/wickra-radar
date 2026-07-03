//! Command-line arguments for `wickra-radar` (§2.2).

use std::path::PathBuf;

use clap::Parser;

/// Scan a perp universe for cascade signals from a spec and an event stream.
#[derive(Parser, Debug)]
#[command(name = "wickra-radar", version, about)]
pub struct Args {
    /// Path to the spec file (JSON or TOML, chosen by extension).
    #[arg(long)]
    pub spec: PathBuf,

    /// Directory of per-symbol event files (`<SYMBOL>.jsonl`, one JSON `Event`
    /// per line); the symbol is the file name without its extension.
    #[arg(long, conflicts_with = "stdin")]
    pub events: Option<PathBuf>,

    /// Read the whole batch as one JSON object (`{"SYM":[...],...}`) from stdin
    /// instead of `--events`.
    #[arg(long, conflicts_with = "events")]
    pub stdin: bool,

    /// Override the spec's top-N limit.
    #[arg(long)]
    pub limit: Option<usize>,

    /// Override the spec's severity threshold.
    #[arg(long)]
    pub threshold: Option<f64>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text)]
    pub format: Format,
}

/// How to render the report.
#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// An aligned human-readable table of alerts.
    Text,
    /// The report serialized as JSON.
    Json,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_all_options() {
        let args = Args::try_parse_from([
            "wickra-radar",
            "--spec",
            "s.json",
            "--events",
            "evdir",
            "--limit",
            "10",
            "--threshold",
            "0.4",
            "--format",
            "json",
        ])
        .unwrap();
        assert_eq!(args.spec, PathBuf::from("s.json"));
        assert_eq!(args.events, Some(PathBuf::from("evdir")));
        assert!(!args.stdin);
        assert_eq!(args.limit, Some(10));
        assert_eq!(args.threshold, Some(0.4));
        assert_eq!(args.format, Format::Json);
    }

    #[test]
    fn format_defaults_to_text() {
        let args = Args::try_parse_from(["wickra-radar", "--spec", "s.json", "--stdin"]).unwrap();
        assert_eq!(args.format, Format::Text);
        assert!(args.stdin);
        assert!(args.limit.is_none());
    }

    #[test]
    fn events_and_stdin_conflict() {
        let err = Args::try_parse_from([
            "wickra-radar",
            "--spec",
            "s.json",
            "--events",
            "evdir",
            "--stdin",
        ]);
        assert!(err.is_err());
    }
}
