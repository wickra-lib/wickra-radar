//! Property-based invariants: for random perp universes and random (valid)
//! specs, `scan` never panics and every report obeys its contract — `scanned`
//! counts the universe, each severity and factor score is a finite number in
//! `[0, 1]`, the `"severity"` factor mirrors the alert severity, every alert is
//! at or above the threshold, the alerts are in total order (severity desc then
//! symbol asc), and the optional limit is respected.
//!
//! The parallel-vs-sequential byte-equality is a compile-time property (the
//! `parallel` feature switches the fold with no runtime toggle), pinned by
//! running the golden suite under both feature sets in CI.

use std::collections::BTreeMap;

use proptest::prelude::*;
use radar_core::{scan, Event, RadarSpec, Side, Signal, SignalKind};

fn arb_ts() -> impl Strategy<Value = i64> {
    0i64..100_000
}

/// A varied (non-geometric) price so log-returns are not constant.
fn arb_price() -> impl Strategy<Value = f64> {
    1.0f64..100_000.0
}

fn arb_qty() -> impl Strategy<Value = f64> {
    0.0f64..1_000.0
}

fn arb_side() -> impl Strategy<Value = Side> {
    prop_oneof![Just(Side::Buy), Just(Side::Sell)]
}

fn arb_event() -> impl Strategy<Value = Event> {
    prop_oneof![
        (arb_ts(), 0.0f64..1_000_000.0, -0.01f64..0.01, arb_price()).prop_map(
            |(ts, open_interest, funding_rate, mark_price)| Event::Derivatives {
                ts,
                open_interest,
                funding_rate,
                mark_price,
            }
        ),
        (arb_ts(), arb_qty(), arb_qty(), arb_price(), arb_price()).prop_map(
            |(ts, bid_volume, ask_volume, best_bid, best_ask)| Event::Orderbook {
                ts,
                bid_volume,
                ask_volume,
                best_bid,
                best_ask,
            }
        ),
        (arb_ts(), arb_side(), arb_qty(), arb_price()).prop_map(|(ts, side, qty, price)| {
            Event::Liquidation {
                ts,
                side,
                qty,
                price,
            }
        }),
    ]
}

/// A universe of 1..6 distinct symbols, each with 0..10 events.
fn arb_universe() -> impl Strategy<Value = BTreeMap<String, Vec<Event>>> {
    prop::collection::vec(prop::collection::vec(arb_event(), 0..10), 1..6).prop_map(|per_symbol| {
        per_symbol
            .into_iter()
            .enumerate()
            .map(|(i, events)| (format!("S{i}"), events))
            .collect()
    })
}

/// One valid signal per kind, with random arity-correct params and a
/// non-negative weight; a subsequence of these keeps the keys distinct.
fn arb_all_signals() -> impl Strategy<Value = Vec<Signal>> {
    let window = 1.0f64..10.0;
    let ref_pct = 0.01f64..1.0;
    let weight = 0.1f64..5.0;
    (
        (window.clone(), ref_pct.clone(), weight.clone()),
        (0.0001f64..0.01, weight.clone()),
        (0.1f64..2.0, weight.clone()),
        (window.clone(), 1.0f64..100.0, weight.clone()),
        (window, ref_pct, weight),
    )
        .prop_map(|(oi, funding, book, liq, div)| {
            vec![
                Signal {
                    kind: SignalKind::OiDelta,
                    params: vec![oi.0, oi.1],
                    weight: oi.2,
                },
                Signal {
                    kind: SignalKind::FundingFlip,
                    params: vec![funding.0],
                    weight: funding.1,
                },
                Signal {
                    kind: SignalKind::BookImbalance,
                    params: vec![book.0],
                    weight: book.1,
                },
                Signal {
                    kind: SignalKind::LiqCluster,
                    params: vec![liq.0, liq.1],
                    weight: liq.2,
                },
                Signal {
                    kind: SignalKind::OiPriceDivergence,
                    params: vec![div.0, div.1],
                    weight: div.2,
                },
            ]
        })
}

fn arb_spec() -> impl Strategy<Value = RadarSpec> {
    (
        arb_all_signals().prop_flat_map(|all| proptest::sample::subsequence(all, 1..=5)),
        0.0f64..1.0,
        prop::option::of(1usize..10),
    )
        .prop_map(|(signals, threshold, limit)| RadarSpec {
            venues: vec![],
            symbols: vec![],
            signals,
            threshold,
            limit,
        })
}

proptest! {
    #[test]
    fn scan_upholds_the_report_contract(
        universe in arb_universe(),
        spec in arb_spec(),
    ) {
        // A validated spec always scans; a panic here is a real defect.
        let report = scan(&universe, &spec).expect("a valid spec must scan");

        prop_assert_eq!(report.scanned, universe.len());
        if let Some(limit) = spec.limit {
            prop_assert!(report.alerts.len() <= limit);
        }

        let mut prev: Option<(f64, &str)> = None;
        for alert in &report.alerts {
            prop_assert!(alert.severity.is_finite());
            prop_assert!((0.0..=1.0).contains(&alert.severity));
            prop_assert!(alert.severity >= spec.threshold);

            for (key, score) in &alert.factors {
                prop_assert!(score.is_finite(), "factor {} not finite", key);
                prop_assert!((0.0..=1.0).contains(score), "factor {} out of range", key);
            }
            let sev_factor = alert.factors.get("severity").copied().unwrap();
            prop_assert!((sev_factor - alert.severity).abs() < 1e-9);

            // Total order: severity descending, then symbol ascending — compared
            // with `total_cmp`, exactly as the scan's sort does.
            if let Some((prev_sev, prev_sym)) = prev {
                let cmp = prev_sev.total_cmp(&alert.severity);
                let ordered = cmp == std::cmp::Ordering::Greater
                    || (cmp == std::cmp::Ordering::Equal && prev_sym <= alert.symbol.as_str());
                prop_assert!(ordered, "alerts out of order");
            }
            prev = Some((alert.severity, alert.symbol.as_str()));
        }
    }
}
