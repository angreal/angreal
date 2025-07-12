//! Integration modules for external tools and services

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

pub mod docker;
pub mod git;
pub mod venv;

/// Create the integrations submodule
/// 
/// This will be exposed as angreal.integrations in Python
#[pymodule]
pub fn integrations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(docker::docker_integration))?;
    m.add_wrapped(wrap_pymodule!(git::git_integration))?;
    
    // Add VirtualEnv directly to integrations module for now
    crate::python_bindings::venv::register_venv(_py, m)?;
    
    Ok(())
}