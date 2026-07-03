//! Signals — the weighted building blocks of a `RadarSpec`.
//!
//! A `Signal` is a `kind` plus its `params` and a `weight`. Each kind produces a
//! normalised score in `[0, 1]` for a perp (the scoring itself lives in
//! `symbol_state`, where the windowed event data is held). Every signal has a
//! canonical string `key` (`"<kind>(<params>)"`) used as its entry in the
//! `factors` map of a `RadarAlert`.

use serde::{Deserialize, Serialize};

/// The five cascade signals. Each scores how strongly it fires for one perp.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    /// Relative change in open interest over a window (leverage building up).
    OiDelta,
    /// Funding-rate magnitude, doubled on a sign flip (positioning stress).
    FundingFlip,
    /// Top-of-book bid/ask volume imbalance.
    BookImbalance,
    /// Sum of liquidation quantity over a window (a cascade in progress).
    LiqCluster,
    /// Open interest rising while price falls, or vice versa (trapped
    /// positioning — the core early-warning signal).
    OiPriceDivergence,
}

impl SignalKind {
    /// The canonical `snake_case` name, matching the serde representation.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            SignalKind::OiDelta => "oi_delta",
            SignalKind::FundingFlip => "funding_flip",
            SignalKind::BookImbalance => "book_imbalance",
            SignalKind::LiqCluster => "liq_cluster",
            SignalKind::OiPriceDivergence => "oi_price_divergence",
        }
    }

    /// The exact number of `params` this kind requires (checked in
    /// `RadarSpec::validate`).
    #[must_use]
    pub fn arity(self) -> usize {
        match self {
            // [window, ref_pct] / [window, ref_qty] / [window, ref_pct]
            SignalKind::OiDelta | SignalKind::LiqCluster | SignalKind::OiPriceDivergence => 2,
            // [flip_threshold] / [ref_abs]
            SignalKind::FundingFlip | SignalKind::BookImbalance => 1,
        }
    }
}

fn one() -> f64 {
    1.0
}

/// A single weighted signal in a `RadarSpec`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Signal {
    /// Which signal this is.
    pub kind: SignalKind,
    /// The kind-specific parameters (see `SignalKind::arity`).
    #[serde(default)]
    pub params: Vec<f64>,
    /// The aggregation weight; defaults to `1.0`.
    #[serde(default = "one")]
    pub weight: f64,
}

impl Signal {
    /// The canonical key for the `factors` map: `"<kind>(<params>)"`, with each
    /// parameter rounded to the fixed output precision so the format never
    /// drifts on trailing digits (e.g. `"oi_delta(20,0.1)"`).
    #[must_use]
    pub fn key(&self) -> String {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|p| fmt_param(round_to(*p, 1e-8)))
            .collect();
        format!("{}({})", self.kind.as_str(), params.join(","))
    }
}

/// Round `x` to the nearest multiple of `quantum` (the fixed output precision,
/// `1e-8`). Keeps `severity`, `factors` scores and key params byte-stable.
pub(crate) fn round_to(x: f64, quantum: f64) -> f64 {
    (x / quantum).round() * quantum
}

/// Format a parameter value with the shortest round-tripping representation
/// (`20.0 -> "20"`, `0.1 -> "0.1"`), matching across every binding since the
/// core string is returned verbatim.
fn fmt_param(x: f64) -> String {
    format!("{x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_formats_kind_and_params() {
        let s = Signal { kind: SignalKind::OiDelta, params: vec![20.0, 0.1], weight: 2.0 };
        assert_eq!(s.key(), "oi_delta(20,0.1)");
        let f = Signal { kind: SignalKind::FundingFlip, params: vec![0.0005], weight: 1.0 };
        assert_eq!(f.key(), "funding_flip(0.0005)");
        let d =
            Signal { kind: SignalKind::OiPriceDivergence, params: vec![20.0, 0.1], weight: 3.0 };
        assert_eq!(d.key(), "oi_price_divergence(20,0.1)");
    }

    #[test]
    fn arity_matches_each_kind() {
        assert_eq!(SignalKind::OiDelta.arity(), 2);
        assert_eq!(SignalKind::FundingFlip.arity(), 1);
        assert_eq!(SignalKind::BookImbalance.arity(), 1);
        assert_eq!(SignalKind::LiqCluster.arity(), 2);
        assert_eq!(SignalKind::OiPriceDivergence.arity(), 2);
    }

    #[test]
    fn round_to_snaps_trailing_drift() {
        assert!((round_to(0.1 + 0.2, 1e-8) - 0.3).abs() < 1e-12);
    }

    #[test]
    fn signal_json_round_trip_defaults_weight() {
        let s: Signal = serde_json::from_str(r#"{"kind":"funding_flip","params":[0.0005]}"#).unwrap();
        assert!((s.weight - 1.0).abs() < f64::EPSILON);
        assert_eq!(s.kind, SignalKind::FundingFlip);
    }
}
