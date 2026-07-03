//! Indicator resolution — the reserved seam for indicator-backed signals (§1.6).
//!
//! # Decision: the MVP signals need no indicators
//!
//! All five cascade signals are **pure window reductions** of the raw event
//! streams, so none resolves a `wickra-core` indicator:
//!
//! - **`oi_delta`** — relative change in open interest over a window of
//!   derivatives events.
//! - **`funding_flip`** — the latest funding-rate magnitude, doubled on a sign
//!   change versus the previous derivatives event.
//! - **`book_imbalance`** — the top-of-book bid/ask volume imbalance from the
//!   latest order-book event.
//! - **`liq_cluster`** — the sum of liquidation quantity over a window of
//!   liquidation events.
//! - **`oi_price_divergence`** — the product of the open-interest and mark-price
//!   percent changes over a window (opposite-direction moves only).
//!
//! Because no signal needs a `name + params` indicator lookup, this module ships
//! **no resolver** rather than dead machinery. Should a future signal need a
//! derived series (e.g. an indicator-smoothed funding curve), this is where the
//! `wickra-backtest-core` registry factory would be wired in — the same resolver
//! the screener and backtester use — and the registry dependencies would return
//! to the crate manifest at that point.
