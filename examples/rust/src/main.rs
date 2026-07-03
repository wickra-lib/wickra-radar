//! A runnable Rust example: scan a perp universe with the native `scan` API and
//! print the report.
//!
//! ```bash
//! cargo run -p wickra-radar-example
//! ```

use std::collections::BTreeMap;

use radar_core::{scan, Event, RadarSpec};

const SPEC: &str = r#"{
    "symbols": ["AAA"],
    "signals": [{"kind": "funding_flip", "params": [0.0005]}],
    "threshold": 0.0
}"#;

const EVENTS: &str = r#"{
    "AAA": [
        {"kind": "derivatives", "ts": 1, "open_interest": 1.0, "funding_rate": 0.0003, "mark_price": 50.0},
        {"kind": "derivatives", "ts": 2, "open_interest": 1.0, "funding_rate": -0.0004, "mark_price": 50.0}
    ]
}"#;

fn main() {
    let spec: RadarSpec = RadarSpec::from_json(SPEC).expect("valid spec");
    let events: BTreeMap<String, Vec<Event>> = serde_json::from_str(EVENTS).expect("valid events");

    let report = scan(&events, &spec).expect("scan");

    println!("wickra-radar {}", radar_core::version());
    println!(
        "{}",
        serde_json::to_string(&report).expect("serialize report")
    );
    println!("  alerts: {}", report.alerts.len());
}
