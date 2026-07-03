//! Per-symbol state — the O(1) fold and the five signal score formulas (§6.2).
//!
//! A `SymbolState` keeps a bounded recent history of each event kind (open
//! interest, funding, mark price and liquidation quantity, plus the latest
//! order-book snapshot). `fold` updates it in O(1) per event; `score` reduces
//! that window into a normalised `[0, 1]` signal score. Every reduction runs
//! serially in event order, so the f64 rounding is identical everywhere.

use std::collections::VecDeque;

use crate::signal::{Signal, SignalKind};
use crate::spec::RadarSpec;

/// Denominator floor, guarding relative changes against a near-zero base.
const EPS: f64 = 1e-12;

/// Clamp `x` into `[0, 1]`. Only ever called on a finite value (`score` guards
/// non-finite inputs to `0.0` before clamping), so `clamp`'s `NaN` panic path is
/// never reached.
fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

/// Push `v`, dropping the oldest value once the buffer exceeds `cap`.
fn push_capped(buf: &mut VecDeque<f64>, v: f64, cap: usize) {
    buf.push_back(v);
    if buf.len() > cap {
        buf.pop_front();
    }
}

/// The recent per-symbol history needed to score every signal in a spec.
#[derive(Clone, Debug)]
pub struct SymbolState {
    oi: VecDeque<f64>,
    funding: VecDeque<f64>,
    mark: VecDeque<f64>,
    liq_qty: VecDeque<f64>,
    last_book: Option<(f64, f64)>,
    last_ts: i64,
    cap: usize,
}

impl SymbolState {
    /// Create a state sized for `spec`: the buffers hold enough history for the
    /// largest window any signal references (and at least two, so the funding
    /// flip always has a previous value).
    #[must_use]
    pub fn new(spec: &RadarSpec) -> Self {
        let mut max_window = 0usize;
        for sig in &spec.signals {
            if matches!(
                sig.kind,
                SignalKind::OiDelta | SignalKind::LiqCluster | SignalKind::OiPriceDivergence
            ) {
                let w = sig.params.first().copied().unwrap_or(0.0);
                let w = if w.is_finite() && w > 0.0 {
                    w as usize
                } else {
                    0
                };
                max_window = max_window.max(w);
            }
        }
        let cap = (max_window + 1).max(2);
        Self {
            oi: VecDeque::new(),
            funding: VecDeque::new(),
            mark: VecDeque::new(),
            liq_qty: VecDeque::new(),
            last_book: None,
            last_ts: i64::MIN,
            cap,
        }
    }

    /// Fold one event into the state in O(1).
    pub fn fold(&mut self, event: &crate::event::Event) {
        use crate::event::Event;
        match *event {
            Event::Derivatives {
                ts,
                open_interest,
                funding_rate,
                mark_price,
            } => {
                push_capped(&mut self.oi, open_interest, self.cap);
                push_capped(&mut self.funding, funding_rate, self.cap);
                push_capped(&mut self.mark, mark_price, self.cap);
                self.last_ts = self.last_ts.max(ts);
            }
            Event::Orderbook {
                ts,
                bid_volume,
                ask_volume,
                ..
            } => {
                self.last_book = Some((bid_volume, ask_volume));
                self.last_ts = self.last_ts.max(ts);
            }
            Event::Liquidation { ts, qty, .. } => {
                push_capped(&mut self.liq_qty, qty, self.cap);
                self.last_ts = self.last_ts.max(ts);
            }
        }
    }

    /// The greatest timestamp folded so far (the "as of" stamp of an alert).
    #[must_use]
    pub fn last_ts(&self) -> i64 {
        self.last_ts
    }

    /// Whether enough events of the right kind have been seen to score `sig`.
    #[must_use]
    pub fn ready(&self, sig: &Signal) -> bool {
        match sig.kind {
            SignalKind::OiDelta | SignalKind::OiPriceDivergence => {
                let window = window_of(sig);
                self.oi.len() > window
            }
            SignalKind::FundingFlip => self.funding.len() >= 2,
            SignalKind::BookImbalance => self.last_book.is_some(),
            SignalKind::LiqCluster => !self.liq_qty.is_empty(),
        }
    }

    /// The normalised `[0, 1]` score for `sig` at the current state (§6.2).
    /// Returns `0.0` when the signal is not ready or an input is non-finite.
    #[must_use]
    pub fn score(&self, sig: &Signal) -> f64 {
        if !self.ready(sig) {
            return 0.0;
        }
        let raw = match sig.kind {
            SignalKind::OiDelta => {
                let window = window_of(sig);
                let ref_pct = sig.params[1];
                let n = self.oi.len();
                let ago = self.oi[n - 1 - window];
                ((self.oi[n - 1] - ago) / ago.max(EPS)).abs() / ref_pct
            }
            SignalKind::FundingFlip => {
                let thresh = sig.params[0];
                let n = self.funding.len();
                let cur = self.funding[n - 1];
                let prev = self.funding[n - 2];
                let base = cur.abs() / thresh;
                if cur * prev < 0.0 {
                    base * 2.0
                } else {
                    base
                }
            }
            SignalKind::BookImbalance => {
                let ref_abs = sig.params[0];
                let Some((bid, ask)) = self.last_book else {
                    return 0.0;
                };
                let imb = (bid - ask) / (bid + ask).max(EPS);
                imb.abs() / ref_abs
            }
            SignalKind::LiqCluster => {
                let window = window_of(sig);
                let ref_qty = sig.params[1];
                let n = self.liq_qty.len();
                let take = window.min(n);
                let sum: f64 = self.liq_qty.iter().skip(n - take).sum();
                sum / ref_qty
            }
            SignalKind::OiPriceDivergence => {
                let window = window_of(sig);
                let ref_pct = sig.params[1];
                let n = self.oi.len();
                let oi_ago = self.oi[n - 1 - window];
                let px_ago = self.mark[n - 1 - window];
                let d_oi = (self.oi[n - 1] - oi_ago) / oi_ago.max(EPS);
                let d_px = (self.mark[n - 1] - px_ago) / px_ago.max(EPS);
                (-d_oi * d_px).max(0.0) / (ref_pct * ref_pct)
            }
        };
        if raw.is_finite() {
            clamp01(raw)
        } else {
            0.0
        }
    }
}

/// The window parameter (`params[0]`) of a windowed signal, as a bounded index.
fn window_of(sig: &Signal) -> usize {
    let w = sig.params.first().copied().unwrap_or(0.0);
    if w.is_finite() && w > 0.0 {
        w as usize
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;

    fn scored(sig: &Signal, events: &[Event]) -> f64 {
        let spec = RadarSpec {
            venues: vec![],
            symbols: vec![],
            signals: vec![sig.clone()],
            threshold: 0.0,
            limit: None,
        };
        let mut state = SymbolState::new(&spec);
        for e in events {
            state.fold(e);
        }
        state.score(sig)
    }

    fn deriv(ts: i64, oi: f64, funding: f64, mark: f64) -> Event {
        Event::Derivatives {
            ts,
            open_interest: oi,
            funding_rate: funding,
            mark_price: mark,
        }
    }

    #[test]
    fn oi_delta_scores_relative_change() {
        let sig = Signal {
            kind: SignalKind::OiDelta,
            params: vec![2.0, 0.1],
            weight: 1.0,
        };
        // OI rises 100 -> 110 over the 2-event window: raw = 0.1, /0.1 = 1.0.
        let s = scored(
            &sig,
            &[
                deriv(1, 100.0, 0.0, 50.0),
                deriv(2, 105.0, 0.0, 50.0),
                deriv(3, 110.0, 0.0, 50.0),
            ],
        );
        assert!((s - 1.0).abs() < 1e-9);
    }

    #[test]
    fn oi_delta_not_ready_scores_zero() {
        let sig = Signal {
            kind: SignalKind::OiDelta,
            params: vec![2.0, 0.1],
            weight: 1.0,
        };
        let s = scored(
            &sig,
            &[deriv(1, 100.0, 0.0, 50.0), deriv(2, 110.0, 0.0, 50.0)],
        );
        assert!(s.abs() < 1e-12);
    }

    #[test]
    fn funding_flip_doubles_on_sign_change() {
        let sig = Signal {
            kind: SignalKind::FundingFlip,
            params: vec![0.0005],
            weight: 1.0,
        };
        // 0.0003 -> -0.0004: flip. base = 0.0004/0.0005 = 0.8, doubled -> 1.6, clamped 1.0.
        let flipped = scored(
            &sig,
            &[deriv(1, 1.0, 0.0003, 50.0), deriv(2, 1.0, -0.0004, 50.0)],
        );
        assert!((flipped - 1.0).abs() < 1e-9);
        // 0.0002 -> 0.0003: no flip. base = 0.6.
        let steady = scored(
            &sig,
            &[deriv(1, 1.0, 0.0002, 50.0), deriv(2, 1.0, 0.0003, 50.0)],
        );
        assert!((steady - 0.6).abs() < 1e-9);
    }

    #[test]
    fn book_imbalance_scores_thin_side() {
        let sig = Signal {
            kind: SignalKind::BookImbalance,
            params: vec![1.0],
            weight: 1.0,
        };
        let book = Event::Orderbook {
            ts: 1,
            bid_volume: 800.0,
            ask_volume: 1200.0,
            best_bid: 49.0,
            best_ask: 51.0,
        };
        // imb = (800-1200)/2000 = -0.2, |imb|/1 = 0.2.
        assert!((scored(&sig, &[book]) - 0.2).abs() < 1e-9);
    }

    #[test]
    fn liq_cluster_sums_window() {
        let sig = Signal {
            kind: SignalKind::LiqCluster,
            params: vec![5.0, 50.0],
            weight: 1.0,
        };
        let liq = |ts: i64, qty: f64| Event::Liquidation {
            ts,
            side: crate::event::Side::Sell,
            qty,
            price: 50.0,
        };
        // sum = 25 over ref 50 -> 0.5.
        assert!((scored(&sig, &[liq(1, 10.0), liq(2, 15.0)]) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn oi_price_divergence_fires_only_on_opposite_moves() {
        let sig = Signal {
            kind: SignalKind::OiPriceDivergence,
            params: vec![2.0, 0.1],
            weight: 1.0,
        };
        // OI +10%, price -10%: -d_oi*d_px = 0.01, /0.01 = 1.0.
        let diverge = scored(
            &sig,
            &[
                deriv(1, 100.0, 0.0, 100.0),
                deriv(2, 105.0, 0.0, 95.0),
                deriv(3, 110.0, 0.0, 90.0),
            ],
        );
        assert!((diverge - 1.0).abs() < 1e-9);
        // OI +10%, price +10%: same direction -> max(0, negative) -> 0.
        let together = scored(
            &sig,
            &[
                deriv(1, 100.0, 0.0, 100.0),
                deriv(2, 105.0, 0.0, 105.0),
                deriv(3, 110.0, 0.0, 110.0),
            ],
        );
        assert!(together.abs() < 1e-12);
    }

    #[test]
    fn non_finite_input_scores_zero() {
        let sig = Signal {
            kind: SignalKind::OiDelta,
            params: vec![2.0, 0.1],
            weight: 1.0,
        };
        let s = scored(
            &sig,
            &[
                deriv(1, 100.0, 0.0, 50.0),
                deriv(2, 105.0, 0.0, 50.0),
                deriv(3, f64::NAN, 0.0, 50.0),
            ],
        );
        assert!(s.abs() < 1e-12);
    }
}
