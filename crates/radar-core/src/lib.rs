//! Data-driven core of the Wickra Radar.
//!
//! A serde `RadarSpec` is folded over a perp universe — open interest, funding,
//! order-book and liquidation events — into a `RadarAlert`: five cascade signals
//! (OI delta, funding flip, book imbalance, liquidation cluster, OI/price
//! divergence) are scored per symbol and aggregated with weights into a severity.
//! Symbols scan in parallel (rayon) or sequentially (the WASM fallback),
//! producing a byte-identical `RadarAlert`.
//!
//! The public surface is assembled module by module through P-RAD-1; the final
//! re-export block lands in `lib.rs` (P-RAD-1.13).

mod aggregate;
mod error;
mod event;
mod indicator_set;
mod signal;
mod spec;
mod symbol_state;
mod universe;

pub use aggregate::severity;
pub use error::{Error, Result};
pub use event::{Event, Side};
pub use signal::{Signal, SignalKind};
pub use spec::RadarSpec;
pub use symbol_state::SymbolState;
pub use universe::Universe;
