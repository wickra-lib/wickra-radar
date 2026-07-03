//! The universe — one `SymbolState` per perp being tracked.

use std::collections::BTreeMap;

use crate::event::Event;
use crate::spec::RadarSpec;
use crate::symbol_state::SymbolState;

/// The set of symbols being scanned, keyed by symbol so iteration is
/// deterministic (a `BTreeMap` orders by key).
#[derive(Clone, Debug, Default)]
pub struct Universe {
    /// Per-symbol state, iterated in key order.
    pub symbols: BTreeMap<String, SymbolState>,
}

impl Universe {
    /// An empty universe.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ensure a symbol has a state, building one sized for `spec` if absent.
    pub fn ensure(&mut self, symbol: &str, spec: &RadarSpec) {
        if !self.symbols.contains_key(symbol) {
            self.symbols
                .insert(symbol.to_string(), SymbolState::new(spec));
        }
    }

    /// Fold one event into a symbol's state (a no-op if the symbol is absent).
    pub fn fold(&mut self, symbol: &str, event: &Event) {
        if let Some(state) = self.symbols.get_mut(symbol) {
            state.fold(event);
        }
    }

    /// The number of symbols tracked.
    #[must_use]
    pub fn scanned(&self) -> usize {
        self.symbols.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal::{Signal, SignalKind};

    fn spec() -> RadarSpec {
        RadarSpec {
            venues: vec![],
            symbols: vec![],
            signals: vec![Signal {
                kind: SignalKind::FundingFlip,
                params: vec![0.0005],
                weight: 1.0,
            }],
            threshold: 0.0,
            limit: None,
        }
    }

    fn deriv(ts: i64) -> Event {
        Event::Derivatives {
            ts,
            open_interest: 1.0,
            funding_rate: 0.0001,
            mark_price: 50.0,
        }
    }

    #[test]
    fn ensure_creates_once_and_fold_routes() {
        let spec = spec();
        let mut u = Universe::new();
        u.ensure("BTCUSDT", &spec);
        u.ensure("BTCUSDT", &spec); // idempotent — must not replace the state
        u.fold("BTCUSDT", &deriv(1));
        assert_eq!(u.scanned(), 1);
        assert!(u.symbols.contains_key("BTCUSDT"));
    }

    #[test]
    fn fold_absent_symbol_is_noop() {
        let mut u = Universe::new();
        u.fold("NOPE", &deriv(1));
        assert_eq!(u.scanned(), 0);
    }

    #[test]
    fn scanned_counts_distinct_symbols() {
        let spec = spec();
        let mut u = Universe::new();
        u.ensure("BTCUSDT", &spec);
        u.ensure("ETHUSDT", &spec);
        assert_eq!(u.scanned(), 2);
    }
}
