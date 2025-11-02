//! Integration modules for external tools and services

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

pub mod compose;
pub mod docker;
pub mod git;
pub mod venv;

/// Create the integrations submodule
///
/// This will be exposed as angreal.integrations in Python
#[pymodule]
pub fn integrations(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(docker::docker_integration))?;

    // Create and register the git submodule
    let git_module = wrap_pymodule!(git::git_integration)(py);
    m.add_submodule(git_module.bind(py))?;

    // Create and register the venv submodule
    let venv_module = wrap_pymodule!(venv::venv)(py);
    m.add_submodule(venv_module.bind(py))?;

    // Also register all modules in sys.modules for proper import support
    let sys = py.import("sys")?;
    let modules = sys.getattr("modules")?;
    modules.set_item("angreal.integrations.git", git_module)?;
    modules.set_item("angreal.integrations.venv", venv_module)?;

    Ok(())
}
