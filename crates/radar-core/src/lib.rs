//! Data-driven core of the Wickra Radar.
//!
//! A serde [`RadarSpec`] is folded over a perp universe — open interest,
//! funding, order-book and liquidation events — into a [`RadarReport`] of
//! [`RadarAlert`]s: five cascade signals (OI delta, funding flip, book
//! imbalance, liquidation cluster, OI/price divergence) are scored per symbol
//! and aggregated with weights into a severity.
//!
//! Every binding drives the core through one entry point,
//! [`Radar::command_json`], whose reply is always a JSON string; the pure
//! [`scan`] function sits underneath for the batch path. With the `parallel`
//! feature (on by default) symbols scan concurrently with rayon;
//! `--no-default-features` scans them sequentially for the WASM target — both
//! yield a byte-identical report.
//!
//! ```
//! use radar_core::Radar;
//! let spec = r#"{"signals":[{"kind":"funding_flip","params":[0.0005]}]}"#;
//! let mut radar = Radar::new(spec).unwrap();
//! let reply = radar.command_json(r#"{"cmd":"version"}"#).unwrap();
//! assert!(reply.contains("version"));
//! ```

mod aggregate;
mod config;
mod error;
mod event;
mod indicator_set;
mod radar;
mod scan;
mod signal;
mod spec;
mod symbol_state;
mod universe;

pub use config::Config;
pub use error::{Error, Result};
pub use event::{Event, Side};
pub use radar::Radar;
pub use scan::{scan, RadarAlert, RadarReport};
pub use signal::{Signal, SignalKind};
pub use spec::RadarSpec;

/// The crate version, e.g. `"0.1.0"`.
#[must_use]
pub fn version() -> &'static str {
    Radar::version()
}
