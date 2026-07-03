//! The `wickra-radar` reference CLI.
//!
//! Loads a [`RadarSpec`](radar_core::RadarSpec) and a perp event stream, scans
//! the universe through `radar-core`, and prints the report as text or JSON. The
//! run pipeline (`run`) arrives in P-RAD-2.3..2.4; this unit parses the
//! arguments and echoes the resolved run configuration.

mod args;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let source = if args.stdin {
        "stdin".to_owned()
    } else {
        args.events
            .as_ref()
            .map_or_else(|| "<none>".to_owned(), |dir| dir.display().to_string())
    };
    let limit = args
        .limit
        .map_or_else(|| "spec".to_owned(), |n| n.to_string());
    let threshold = args
        .threshold
        .map_or_else(|| "spec".to_owned(), |t| t.to_string());
    println!(
        "wickra-radar {}: spec={} source={source} limit={limit} threshold={threshold} format={:?}",
        radar_core::version(),
        args.spec.display(),
        args.format,
    );
}
