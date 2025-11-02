//! Docker integration bindings

use crate::python_bindings::integrations::compose;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

/// Docker integration module
///
/// This will be exposed as angreal.integrations.docker in Python
#[pymodule]
pub fn docker_integration(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<docker_pyo3::Pyo3Docker>()?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::image::image))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::container::container))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::network::network))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::volume::volume))?;

    // Add Docker Compose functionality
    m.add_class::<compose::PyDockerCompose>()?;
    m.add_class::<compose::PyComposeOutput>()?;
    m.add_function(wrap_pyfunction!(compose::compose, m)?)?;

    Ok(())
}
