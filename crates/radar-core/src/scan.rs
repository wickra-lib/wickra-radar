//! The batch scan — folds a perp universe into a sorted `RadarReport`.
//!
//! Each symbol's state is built independently (folds are symbol-local), so the
//! fold parallelises with rayon under the `parallel` feature and runs serially
//! otherwise. The severity aggregation and the final ordering are a total order
//! (severity descending, then symbol ascending), so the report is identical
//! whether or not the fold ran in parallel.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::aggregate::severity;
use crate::error::Result;
use crate::event::Event;
use crate::spec::RadarSpec;
use crate::symbol_state::SymbolState;

/// One perp's alert: its severity, the contributing signal scores and the
/// timestamp of the latest event that fed it.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RadarAlert {
    /// The perp symbol.
    pub symbol: String,
    /// The aggregate severity in `[0, 1]`.
    pub severity: f64,
    /// Each signal's score by its canonical key, plus a `"severity"` entry.
    pub factors: BTreeMap<String, f64>,
    /// The greatest event timestamp folded for this symbol.
    pub ts: i64,
}

/// The result of a scan: the over-threshold alerts (sorted, limited) and how
/// many perps were examined.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RadarReport {
    /// Alerts at or above the threshold, sorted by severity desc then symbol asc.
    pub alerts: Vec<RadarAlert>,
    /// The number of perps examined.
    pub scanned: usize,
}

/// Scan a batch of per-symbol event streams into a `RadarReport`.
///
/// # Errors
/// Returns [`crate::Error::BadSpec`] if the spec fails validation.
pub fn scan(events: &BTreeMap<String, Vec<Event>>, spec: &RadarSpec) -> Result<RadarReport> {
    spec.validate()?;
    let scanned = events.len();
    let states = fold_symbols(events, spec);
    Ok(report_from_states(
        states.iter().map(|(sym, state)| (sym.as_str(), state)),
        spec,
        scanned,
    ))
}

/// Build a sorted, limited report from already-folded per-symbol states.
///
/// Shared by the batch [`scan`] and the streaming `Radar::alerts` so both
/// produce an identical report (§6.6): severity aggregation, the threshold
/// filter, the total-order sort (severity descending, symbol ascending) and the
/// optional limit.
pub(crate) fn report_from_states<'a>(
    states: impl Iterator<Item = (&'a str, &'a SymbolState)>,
    spec: &RadarSpec,
    scanned: usize,
) -> RadarReport {
    let mut alerts = Vec::new();
    for (symbol, state) in states {
        let (sev, factors) = severity(state, spec);
        if sev >= spec.threshold {
            alerts.push(RadarAlert {
                symbol: symbol.to_string(),
                severity: sev,
                factors,
                ts: state.last_ts(),
            });
        }
    }
    // `total_cmp` gives a total order over f64 (scores are finite anyway), so the
    // result is deterministic regardless of the fold order.
    alerts.sort_by(|a, b| {
        b.severity
            .total_cmp(&a.severity)
            .then_with(|| a.symbol.cmp(&b.symbol))
    });
    if let Some(limit) = spec.limit {
        alerts.truncate(limit);
    }
    RadarReport { alerts, scanned }
}

/// Fold every event onto a fresh `SymbolState`.
fn fold_one(events: &[Event], spec: &RadarSpec) -> SymbolState {
    let mut state = SymbolState::new(spec);
    for event in events {
        state.fold(event);
    }
    state
}

/// Build each symbol's state, in parallel across symbols (rayon).
#[cfg(feature = "parallel")]
fn fold_symbols(
    events: &BTreeMap<String, Vec<Event>>,
    spec: &RadarSpec,
) -> Vec<(String, SymbolState)> {
    use rayon::prelude::*;
    events
        .par_iter()
        .map(|(sym, evs)| (sym.clone(), fold_one(evs, spec)))
        .collect()
}

/// Build each symbol's state serially (the WASM / no-rayon fallback).
#[cfg(not(feature = "parallel"))]
fn fold_symbols(
    events: &BTreeMap<String, Vec<Event>>,
    spec: &RadarSpec,
) -> Vec<(String, SymbolState)> {
    events
        .iter()
        .map(|(sym, evs)| (sym.clone(), fold_one(evs, spec)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::{Signal, SignalKind};

    fn spec(threshold: f64, limit: Option<usize>) -> RadarSpec {
        RadarSpec {
            venues: vec![],
            symbols: vec![],
            signals: vec![Signal {
                kind: SignalKind::FundingFlip,
                params: vec![0.0005],
                weight: 1.0,
            }],
            threshold,
            limit,
        }
    }

    fn deriv(ts: i64, funding: f64) -> Event {
        Event::Derivatives {
            ts,
            open_interest: 1.0,
            funding_rate: funding,
            mark_price: 50.0,
        }
    }

    // AAA's funding flips (score 1.0); BBB's does not (score 0.6).
    fn events() -> BTreeMap<String, Vec<Event>> {
        let mut m = BTreeMap::new();
        m.insert("AAA".to_string(), vec![deriv(1, 0.0003), deriv(2, -0.0004)]);
        m.insert("BBB".to_string(), vec![deriv(1, 0.0002), deriv(2, 0.0003)]);
        m
    }

    #[test]
    fn scan_filters_sorts_and_stamps() {
        let report = scan(&events(), &spec(0.0, None)).unwrap();
        assert_eq!(report.scanned, 2);
        assert_eq!(report.alerts.len(), 2);
        assert_eq!(report.alerts[0].symbol, "AAA");
        assert_eq!(report.alerts[1].symbol, "BBB");
        assert!((report.alerts[0].severity - 1.0).abs() < 1e-9);
        assert_eq!(report.alerts[0].ts, 2);
    }

    #[test]
    fn threshold_filters_out_below() {
        let report = scan(&events(), &spec(0.8, None)).unwrap();
        assert_eq!(report.alerts.len(), 1);
        assert_eq!(report.alerts[0].symbol, "AAA");
        assert_eq!(report.scanned, 2);
    }

    #[test]
    fn limit_truncates_after_sort() {
        let report = scan(&events(), &spec(0.0, Some(1))).unwrap();
        assert_eq!(report.alerts.len(), 1);
        assert_eq!(report.alerts[0].symbol, "AAA");
    }

    #[test]
    fn invalid_spec_errors() {
        let bad = RadarSpec {
            venues: vec![],
            symbols: vec![],
            signals: vec![],
            threshold: 0.0,
            limit: None,
        };
        assert!(scan(&events(), &bad).is_err());
    }

    #[test]
    fn tie_broken_by_symbol_ascending() {
        let mut m = BTreeMap::new();
        m.insert("ZZZ".to_string(), vec![deriv(1, 0.0003), deriv(2, -0.0004)]);
        m.insert("AAA".to_string(), vec![deriv(1, 0.0003), deriv(2, -0.0004)]);
        let report = scan(&m, &spec(0.0, None)).unwrap();
        assert_eq!(report.alerts[0].symbol, "AAA");
        assert_eq!(report.alerts[1].symbol, "ZZZ");
    }
}
