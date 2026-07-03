//! Conformance tests: the JSON contract (§6) is stable and self-consistent —
//! every enum representation round-trips, domain errors surface as in-band JSON,
//! and each alert's `factors` map carries one entry per signal keyed by its
//! canonical key.

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use radar_core::{Event, Radar, RadarAlert, RadarReport, RadarSpec, Side, Signal, SignalKind};

const SPEC: &str =
    r#"{"symbols":["AAA"],"signals":[{"kind":"funding_flip","params":[0.0005]}],"threshold":0.0}"#;

/// Serialize, deserialize, and assert the value survives unchanged.
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) {
    let json = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(value, &back);
}

#[test]
fn enum_tags_are_snake_case() {
    assert_eq!(serde_json::to_string(&Side::Buy).unwrap(), r#""buy""#);
    assert_eq!(serde_json::to_string(&Side::Sell).unwrap(), r#""sell""#);
    assert_eq!(
        serde_json::to_string(&SignalKind::OiDelta).unwrap(),
        r#""oi_delta""#
    );
    assert_eq!(
        serde_json::to_string(&SignalKind::FundingFlip).unwrap(),
        r#""funding_flip""#
    );
    assert_eq!(
        serde_json::to_string(&SignalKind::BookImbalance).unwrap(),
        r#""book_imbalance""#
    );
    assert_eq!(
        serde_json::to_string(&SignalKind::LiqCluster).unwrap(),
        r#""liq_cluster""#
    );
    assert_eq!(
        serde_json::to_string(&SignalKind::OiPriceDivergence).unwrap(),
        r#""oi_price_divergence""#
    );
}

#[test]
fn signal_kinds_round_trip() {
    for kind in [
        SignalKind::OiDelta,
        SignalKind::FundingFlip,
        SignalKind::BookImbalance,
        SignalKind::LiqCluster,
        SignalKind::OiPriceDivergence,
    ] {
        round_trip(&kind);
    }
}

#[test]
fn event_variants_round_trip() {
    round_trip(&Event::Derivatives {
        ts: 1,
        open_interest: 100.0,
        funding_rate: 0.0003,
        mark_price: 50.0,
    });
    round_trip(&Event::Orderbook {
        ts: 2,
        bid_volume: 800.0,
        ask_volume: 1200.0,
        best_bid: 49.0,
        best_ask: 51.0,
    });
    round_trip(&Event::Liquidation {
        ts: 3,
        side: Side::Sell,
        qty: 10.0,
        price: 50.0,
    });
}

#[test]
fn spec_and_signal_round_trip() {
    round_trip(&Signal {
        kind: SignalKind::OiDelta,
        params: vec![20.0, 0.1],
        weight: 2.0,
    });
    let spec: RadarSpec = serde_json::from_str(SPEC).unwrap();
    round_trip(&spec);
}

#[test]
fn invalid_spec_on_construction_errors() {
    // No signals, and a signal with the wrong arity, each fail validation.
    assert!(Radar::new(r#"{"signals":[]}"#).is_err());
    assert!(Radar::new(r#"{"signals":[{"kind":"funding_flip","params":[0.1,0.2]}]}"#).is_err());
}

#[test]
fn bad_spec_command_yields_error_json() {
    let mut radar = Radar::new(SPEC).unwrap();
    let reply = radar
        .command_json(r#"{"cmd":"set_spec","spec":{"signals":[]}}"#)
        .unwrap();
    assert!(reply.contains(r#""ok":false"#), "{reply}");
}

#[test]
fn unknown_command_yields_error_json() {
    let mut radar = Radar::new(SPEC).unwrap();
    let reply = radar.command_json(r#"{"cmd":"nope"}"#).unwrap();
    assert!(reply.contains(r#""ok":false"#), "{reply}");
    assert!(reply.contains("unknown cmd"), "{reply}");
}

#[test]
fn factors_carry_one_entry_per_signal_plus_severity() {
    let spec = r#"{"signals":[
        {"kind":"funding_flip","params":[0.0005]},
        {"kind":"book_imbalance","params":[1.0]}
    ],"threshold":0.0}"#;
    let events = r#"{"AAA":[
        {"kind":"derivatives","ts":1,"open_interest":1.0,"funding_rate":0.0003,"mark_price":50.0},
        {"kind":"derivatives","ts":2,"open_interest":1.0,"funding_rate":-0.0004,"mark_price":50.0},
        {"kind":"orderbook","ts":2,"bid_volume":800.0,"ask_volume":1200.0,"best_bid":49.0,"best_ask":51.0}
    ]}"#;
    let mut radar = Radar::new(spec).unwrap();
    let raw = radar
        .command_json(&format!(r#"{{"cmd":"scan","events":{events}}}"#))
        .unwrap();
    let report: RadarReport = serde_json::from_str(&raw).unwrap();

    assert_eq!(report.scanned, 1);
    let alert: &RadarAlert = &report.alerts[0];
    assert_eq!(alert.symbol, "AAA");
    // One key per signal, plus the "severity" entry.
    assert!(alert.factors.contains_key("funding_flip(0.0005)"));
    assert!(alert.factors.contains_key("book_imbalance(1)"));
    assert!(alert.factors.contains_key("severity"));
    assert_eq!(alert.factors.len(), 3);
}
