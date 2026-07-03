//! Input events — the perp-derivative, order-book and liquidation streams the
//! radar folds.
//!
//! `Event` is an internally-tagged serde enum whose variants mirror the
//! `wickra-exchange` derivative / order-book / liquidation stream shapes. The
//! JSON form (`{"kind": "...", ...}`, `snake_case`) is the language boundary and
//! must be byte-identical across all ten bindings.

use serde::{Deserialize, Serialize};

/// The side of a liquidation. `Buy` means a long position was liquidated (a buy
/// to close); `Sell` means a short was liquidated.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    /// A long position was liquidated.
    Buy,
    /// A short position was liquidated.
    Sell,
}

/// A single perp market event. Field names mirror the `wickra-exchange`
/// derivative / order-book / liquidation streams; `ts` is seconds or
/// milliseconds, consistent with the feed.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    /// A derivatives snapshot: open interest, funding rate and mark price.
    Derivatives {
        /// Event timestamp.
        ts: i64,
        /// Open interest, in contracts or base units.
        open_interest: f64,
        /// The current funding rate (a small signed fraction).
        funding_rate: f64,
        /// The mark price.
        mark_price: f64,
    },
    /// A top-of-book snapshot: resting bid/ask volume and best bid/ask.
    Orderbook {
        /// Event timestamp.
        ts: i64,
        /// Resting bid volume near the top of book.
        bid_volume: f64,
        /// Resting ask volume near the top of book.
        ask_volume: f64,
        /// Best bid price.
        best_bid: f64,
        /// Best ask price.
        best_ask: f64,
    },
    /// A liquidation: the side, quantity and price of a forced close.
    Liquidation {
        /// Event timestamp.
        ts: i64,
        /// Which side was liquidated.
        side: Side,
        /// The liquidated quantity.
        qty: f64,
        /// The liquidation price.
        price: f64,
    },
}

impl Event {
    /// The event timestamp, regardless of variant.
    #[must_use]
    pub fn ts(&self) -> i64 {
        match *self {
            Event::Derivatives { ts, .. }
            | Event::Orderbook { ts, .. }
            | Event::Liquidation { ts, .. } => ts,
        }
    }
}
