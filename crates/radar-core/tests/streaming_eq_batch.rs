//! Streaming/batch equivalence (§6.6): folding a universe event-by-event with
//! `feed` and then asking for `alerts` yields the *exact* same report as a
//! one-shot `scan` over the whole universe — byte-for-byte, because both go
//! through the shared `report_from_states` aggregation.
//!
//! This is the streaming counterpart of the golden test: the batch `scan` is the
//! path the CLI and the golden fixtures exercise; `feed`/`feed_batch` +`alerts`
//! is the path a live consumer drives. They must never diverge.

use std::fs;
use std::path::{Path, PathBuf};

use radar_core::Radar;
use serde_json::Value;

/// The repository-root `golden/` directory, resolved from this crate's manifest.
fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("golden")
}

/// The shared golden universe as a `{ symbol: [event, …] }` map.
fn events_map() -> serde_json::Map<String, Value> {
    let raw = fs::read_to_string(golden_dir().join("events.json")).unwrap();
    match serde_json::from_str(&raw).unwrap() {
        Value::Object(map) => map,
        other => panic!("events.json is not an object: {other}"),
    }
}

/// Every golden spec path, sorted.
fn spec_paths() -> Vec<PathBuf> {
    let mut specs: Vec<PathBuf> = fs::read_dir(golden_dir().join("specs"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    specs.sort();
    specs
}

/// Drive a fresh radar with per-event `feed`s (events in array order per symbol)
/// and return the `alerts` reply.
fn streamed(spec_json: &str, events: &serde_json::Map<String, Value>) -> String {
    let mut radar = Radar::new(spec_json).unwrap();
    for (symbol, evs) in events {
        for event in evs.as_array().unwrap() {
            let cmd = format!(
                r#"{{"cmd":"feed","symbol":{},"event":{event}}}"#,
                json_str(symbol)
            );
            radar.command_json(&cmd).unwrap();
        }
    }
    radar.command_json(r#"{"cmd":"alerts"}"#).unwrap()
}

/// Drive a fresh radar with one `feed_batch` per symbol and return `alerts`.
fn streamed_batched(spec_json: &str, events: &serde_json::Map<String, Value>) -> String {
    let mut radar = Radar::new(spec_json).unwrap();
    for (symbol, evs) in events {
        let cmd = format!(
            r#"{{"cmd":"feed_batch","symbol":{},"events":{evs}}}"#,
            json_str(symbol)
        );
        radar.command_json(&cmd).unwrap();
    }
    radar.command_json(r#"{"cmd":"alerts"}"#).unwrap()
}

/// A one-shot `scan` over the whole universe.
fn scanned(spec_json: &str, events: &serde_json::Map<String, Value>) -> String {
    let mut radar = Radar::new(spec_json).unwrap();
    let map = Value::Object(events.clone());
    radar
        .command_json(&format!(r#"{{"cmd":"scan","events":{map}}}"#))
        .unwrap()
}

/// JSON-encode a string so a symbol with awkward characters is still valid.
fn json_str(s: &str) -> String {
    Value::String(s.to_string()).to_string()
}

#[test]
fn feed_then_alerts_equals_scan_for_every_golden_spec() {
    let events = events_map();
    let specs = spec_paths();
    assert!(!specs.is_empty(), "no golden specs found");

    for spec_path in specs {
        let name = spec_path.file_stem().unwrap().to_str().unwrap();
        let spec_json = fs::read_to_string(&spec_path).unwrap();

        let batch = scanned(&spec_json, &events);
        assert_eq!(streamed(&spec_json, &events), batch, "{name}: feed vs scan");
        assert_eq!(
            streamed_batched(&spec_json, &events),
            batch,
            "{name}: feed_batch vs scan"
        );
    }
}

#[test]
fn reset_returns_to_the_empty_report() {
    let events = events_map();
    let spec = fs::read_to_string(golden_dir().join("specs").join("funding-flip.json")).unwrap();

    let mut radar = Radar::new(&spec).unwrap();
    for (symbol, evs) in &events {
        let cmd = format!(
            r#"{{"cmd":"feed_batch","symbol":{},"events":{evs}}}"#,
            json_str(symbol)
        );
        radar.command_json(&cmd).unwrap();
    }
    radar.command_json(r#"{"cmd":"reset"}"#).unwrap();
    let after = radar.command_json(r#"{"cmd":"alerts"}"#).unwrap();
    assert_eq!(after, r#"{"alerts":[],"scanned":0}"#);
}
