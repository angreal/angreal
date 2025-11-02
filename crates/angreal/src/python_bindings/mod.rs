//! Python bindings for angreal
//!
//! This module provides Python bindings for angreal functionality, organized
//! into logical submodules. It also provides a public API for other Rust
//! projects to initialize angreal's Python bindings.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pymodule;

pub mod decorators;
pub mod integrations;
pub mod venv;

/// Initialize angreal's Python bindings
///
/// This function can be called by other Rust projects to set up angreal's
/// Python bindings in an embedded Python interpreter.
///
/// # Example
///
/// ```rust
/// use angreal::python_bindings;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     python_bindings::initialize()?;
///     // Now Python can import angreal
///     Ok(())
/// }
/// ```
pub fn initialize() -> PyResult<()> {
    Python::attach(|py| {
        let sys = py.import("sys")?;
        let modules_attr = sys.getattr("modules")?;
        let modules = modules_attr.downcast::<PyDict>()?;

        // Create and register the main angreal module
        let angreal_module = create_angreal_module(py)?;
        modules.set_item("angreal", &angreal_module)?;

        Ok(())
    })
}

/// Create the main angreal Python module
///
/// This assembles all the submodules and functions into the main angreal module
/// that Python will import.
pub(crate) fn create_angreal_module(py: Python) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "angreal")?;

    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Register core functions (these will be moved from lib.rs)
    // TODO: Move these from lib.rs
    // m.add_function(wrap_pyfunction!(main, m)?)?;
    // m.add_function(wrap_pyfunction!(ensure_uv_installed, m)?)?;
    // ... etc

    // Register decorator functions
    decorators::register_decorators(py, &m)?;

    // Register submodules
    m.add_wrapped(wrap_pymodule!(integrations::integrations))?;

    Ok(m)
}
