//! The `wickra-radar` reference CLI.
//!
//! Loads a [`RadarSpec`](radar_core::RadarSpec) and a perp event stream (a
//! directory of per-symbol `<SYMBOL>.jsonl` files or a batch JSON on stdin),
//! scans the universe through `radar-core`, and prints the report as text or
//! JSON. Exits non-zero if the run fails, so scripts and CI can detect an error.

mod args;
mod run;

use std::process::ExitCode;

use args::Args;
use clap::Parser;

fn main() -> ExitCode {
    let args = Args::parse();
    match run::run(&args) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("wickra-radar: {err}");
            ExitCode::FAILURE
        }
    }
}
