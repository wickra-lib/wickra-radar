//! Severity aggregation — the weighted mean of a symbol's signal scores (§6.5).

use std::collections::BTreeMap;

use crate::signal::round_to;
use crate::spec::RadarSpec;
use crate::symbol_state::SymbolState;

/// The fixed output precision for severity and factor scores.
const QUANTUM: f64 = 1e-8;

/// Aggregate a symbol's signal scores into a severity and a `factors` map.
///
/// `severity = (Σ wᵢ·sᵢ) / (Σ wᵢ)`, or `0.0` when the weights sum to zero. The
/// `factors` map carries every signal's rounded score keyed by its canonical
/// [`Signal::key`](crate::Signal::key), plus a `"severity"` entry, so a consumer
/// can see exactly which signal contributed how much. Sums run serially in
/// signal declaration order, and every value is rounded to a fixed precision, so
/// the f64 output is identical in every language.
#[must_use]
pub fn severity(state: &SymbolState, spec: &RadarSpec) -> (f64, BTreeMap<String, f64>) {
    let mut factors = BTreeMap::new();
    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;
    for sig in &spec.signals {
        let score = state.score(sig);
        weighted_sum += sig.weight * score;
        weight_sum += sig.weight;
        factors.insert(sig.key(), round_to(score, QUANTUM));
    }
    // Weights are validated non-negative, so a sum of `0.0` means every weight
    // is zero; guard with `<=` to keep clear of a float equality comparison.
    let sev = if weight_sum <= 0.0 {
        0.0
    } else {
        round_to(weighted_sum / weight_sum, QUANTUM)
    };
    factors.insert("severity".to_string(), sev);
    (sev, factors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use crate::signal::{Signal, SignalKind};

    fn deriv(ts: i64, funding: f64) -> Event {
        Event::Derivatives {
            ts,
            open_interest: 1.0,
            funding_rate: funding,
            mark_price: 50.0,
        }
    }

    fn book() -> Event {
        Event::Orderbook {
            ts: 3,
            bid_volume: 800.0,
            ask_volume: 1200.0,
            best_bid: 49.0,
            best_ask: 51.0,
        }
    }

    fn state_and_spec(weights: (f64, f64)) -> (SymbolState, RadarSpec) {
        let spec = RadarSpec {
            venues: vec![],
            symbols: vec![],
            signals: vec![
                Signal {
                    kind: SignalKind::FundingFlip,
                    params: vec![0.0005],
                    weight: weights.0,
                },
                Signal {
                    kind: SignalKind::BookImbalance,
                    params: vec![1.0],
                    weight: weights.1,
                },
            ],
            threshold: 0.0,
            limit: None,
        };
        let mut state = SymbolState::new(&spec);
        state.fold(&deriv(1, 0.0002));
        state.fold(&deriv(2, 0.0003));
        state.fold(&book());
        (state, spec)
    }

    #[test]
    fn severity_is_weighted_mean_with_factors() {
        let (state, spec) = state_and_spec((2.0, 1.0));
        let (sev, factors) = severity(&state, &spec);
        // funding_flip = 0.6 (weight 2), book_imbalance = 0.2 (weight 1): (1.2+0.2)/3.
        assert!((sev - 0.466_666_67).abs() < 1e-7);
        assert!((factors["funding_flip(0.0005)"] - 0.6).abs() < 1e-7);
        assert!((factors["book_imbalance(1)"] - 0.2).abs() < 1e-7);
        assert!((factors["severity"] - sev).abs() < 1e-12);
    }

    #[test]
    fn zero_weight_sum_yields_zero_severity() {
        let (state, spec) = state_and_spec((0.0, 0.0));
        let (sev, factors) = severity(&state, &spec);
        assert!(sev.abs() < 1e-12);
        assert!(factors["severity"].abs() < 1e-12);
        // The per-signal scores are still recorded.
        assert!((factors["funding_flip(0.0005)"] - 0.6).abs() < 1e-7);
    }
}
