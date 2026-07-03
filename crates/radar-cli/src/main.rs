//! The `wickra-radar` reference CLI.
//!
//! Loads a [`RadarSpec`](radar_core::RadarSpec) and a perp event stream (a
//! directory of per-symbol `<SYMBOL>.jsonl` files or a batch JSON on stdin),
//! scans the universe through `radar-core`, and prints the report as text or
//! JSON. Proper exit-code handling arrives in P-RAD-2.4.

mod args;
mod run;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    match run::run(&args) {
        Ok(output) => println!("{output}"),
        Err(err) => eprintln!("wickra-radar: {err}"),
    }
}
