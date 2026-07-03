#![no_main]
//! Fuzz the batch scan: a `{spec, events}` object is parsed and scanned. The
//! spec and the whole perp universe are attacker-controlled; the scan must never
//! panic (an invalid spec is a clean `Err`, not a crash).

use std::collections::BTreeMap;

use libfuzzer_sys::fuzz_target;
use radar_core::{scan, Event, RadarSpec};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    spec: RadarSpec,
    events: BTreeMap<String, Vec<Event>>,
}

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else {
        return;
    };
    let Ok(input) = serde_json::from_str::<Input>(text) else {
        return;
    };
    // Bound the total work so the fuzzer cannot request an unbounded fold.
    let total: usize = input.events.values().map(Vec::len).sum();
    if total > 5000 {
        return;
    }
    let _ = scan(&input.events, &input.spec);
});
