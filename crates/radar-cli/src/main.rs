//! The `wickra-radar` reference CLI.
//!
//! Loads a [`RadarSpec`](radar_core::RadarSpec) and a perp event stream, scans
//! the universe through `radar-core`, and prints the report as text or JSON.
//! Argument parsing (`args`) and the run pipeline (`run`) arrive in
//! P-RAD-2.2..2.4; this initial unit reports the core version so the crate is a
//! real, runnable member.

fn main() {
    println!("wickra-radar {}", radar_core::version());
}
