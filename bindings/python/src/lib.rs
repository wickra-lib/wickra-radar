//! Python bindings for `wickra-radar`, exposed under the `wickra_radar` package.
//!
//! Thin glue over the radar core's data-driven surface: build an [`Radar`] from a
//! spec JSON, drive it with a command JSON and read back the response JSON. The
//! same command protocol crosses every binding, so a Python front-end drives the
//! exact same core as the native CLI.

// PyO3 protocol methods take `self` by value/ref regardless of use.
#![allow(clippy::needless_pass_by_value)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use radar_core::Radar;

/// A radar instance driven by JSON commands.
///
/// `unsendable`: a handle owns a mutable dataset and spec, so it is bound to the
/// thread that created it.
#[pyclass(name = "Radar", unsendable)]
struct PyRadar {
    inner: Radar,
}

#[pymethods]
impl PyRadar {
    /// Build a radar from a spec JSON string (`""` or `"{}"` for an empty handle
    /// whose spec is set later via a `set_spec` command).
    #[new]
    fn new(spec_json: &str) -> PyResult<Self> {
        Radar::new(spec_json)
            .map(|inner| Self { inner })
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// Apply a command JSON and return the resulting response JSON.
    fn command(&mut self, cmd_json: &str) -> PyResult<String> {
        self.inner
            .command_json(cmd_json)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// The library version.
    #[staticmethod]
    fn version() -> &'static str {
        radar_core::version()
    }
}

/// The native module (`wickra_radar._wickra_radar`).
#[pymodule]
fn _wickra_radar(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add_class::<PyRadar>()?;
    Ok(())
}
