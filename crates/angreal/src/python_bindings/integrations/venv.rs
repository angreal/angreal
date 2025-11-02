//! Virtual environment integration submodule
//!
//! This module provides the venv submodule for angreal.integrations.venv

use pyo3::prelude::*;

/// Create the venv submodule
#[pymodule]
pub fn venv(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register VirtualEnv and venv_required from the main venv module
    crate::python_bindings::venv::register_venv(_py, m)?;
    Ok(())
}
