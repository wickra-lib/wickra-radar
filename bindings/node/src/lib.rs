//! Node.js bindings for `wickra-radar` (napi-rs).
//!
//! Thin glue over the radar core's data-driven surface: build an `Radar` from a
//! spec JSON, drive it with a command JSON and read back the response JSON. The
//! same command protocol crosses every binding, so a Node front-end drives the
//! exact same core as the native CLI.

#![allow(missing_debug_implementations)]
// napi exposes owned `String` arguments; the bodies only need to borrow them.
#![allow(clippy::needless_pass_by_value)]

use napi::Result;
use napi_derive::napi;

use radar_core::Radar as CoreRadar;

/// Build a napi error from a message.
fn err(message: impl Into<String>) -> napi::Error {
    napi::Error::from_reason(message.into())
}

/// The library version.
#[napi]
pub fn version() -> String {
    CoreRadar::version().to_string()
}

/// A radar instance driven by JSON commands.
#[napi]
pub struct Radar {
    inner: CoreRadar,
}

#[napi]
impl Radar {
    /// Build a radar from a spec JSON string.
    #[napi(constructor)]
    pub fn new(spec_json: String) -> Result<Self> {
        CoreRadar::new(&spec_json)
            .map(|inner| Self { inner })
            .map_err(|e| err(e.to_string()))
    }

    /// Apply a command JSON and return the resulting response JSON.
    #[napi]
    pub fn command(&mut self, cmd_json: String) -> Result<String> {
        self.inner
            .command_json(&cmd_json)
            .map_err(|e| err(e.to_string()))
    }

    /// The library version.
    #[napi]
    pub fn version(&self) -> String {
        CoreRadar::version().to_string()
    }
}
