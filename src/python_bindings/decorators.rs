//! Python decorator functions
//!
//! This module contains PyO3 functions for the decorators:
//! - @required_version
//! - @group 
//! - @command
//! - @argument
//! - command_group helper function

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use version_compare::{Cmp, compare};
use crate::task::AngrealGroup;

/// Check if the current angreal version meets the specified requirement
/// 
/// This is equivalent to Python's packaging.specifiers.Specifier.contains()
/// but implemented in Rust using version_compare crate.
#[pyfunction]
pub fn required_version(specifier: &str) -> PyResult<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    
    // Parse the specifier (e.g., ">=2.0.0", "==1.2.3", "~=1.0")
    let is_compatible = if specifier.starts_with(">=") {
        let required = &specifier[2..];
        compare(current_version, required) != Ok(Cmp::Lt)
    } else if specifier.starts_with("<=") {
        let required = &specifier[2..];
        compare(current_version, required) != Ok(Cmp::Gt)
    } else if specifier.starts_with("==") {
        let required = &specifier[2..];
        compare(current_version, required) == Ok(Cmp::Eq)
    } else if specifier.starts_with("!=") {
        let required = &specifier[2..];
        compare(current_version, required) != Ok(Cmp::Eq)
    } else if specifier.starts_with(">") {
        let required = &specifier[1..];
        compare(current_version, required) == Ok(Cmp::Gt)
    } else if specifier.starts_with("<") {
        let required = &specifier[1..];
        compare(current_version, required) == Ok(Cmp::Lt)
    } else {
        // Default to exact match if no operator
        compare(current_version, specifier) == Ok(Cmp::Eq)
    };
    
    if !is_compatible {
        return Err(PyErr::new::<pyo3::exceptions::PyEnvironmentError, _>(
            format!(
                "You require angreal {} but have {} installed.", 
                specifier, 
                current_version
            )
        ));
    }
    
    Ok(())
}

/// Register decorator functions to a Python module
pub fn register_decorators(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(required_version, m)?)?;
    m.add_function(wrap_pyfunction!(group, m)?)?;
    m.add_function(wrap_pyfunction!(command_group, m)?)?;
    m.add_class::<GroupDecorator>()?;
    // TODO: Add other decorators as we implement them
    Ok(())
}

/// A Python callable that wraps the group decorator logic
#[pyclass]
pub struct GroupDecorator {
    name: String,
    about: Option<String>,
}

#[pymethods]
impl GroupDecorator {
    fn __call__(&self, py: Python, func: PyObject) -> PyResult<PyObject> {
        // Check if function has __command attribute
        let has_command = func.getattr(py, "__command").is_ok();
        
        if !has_command {
            return Err(PyErr::new::<pyo3::exceptions::PySyntaxError, _>(
                "The group decorator must be applied before a command."
            ));
        }
        
        // Create the AngrealGroup using PyO3's class instantiation
        let group_class = py.get_type::<AngrealGroup>();
        let group = group_class.call1((&self.name, self.about.as_deref()))?;
        
        // Get the __command attribute and call add_group on it
        let command = func.getattr(py, "__command")?;
        command.call_method1(py, "add_group", (group,))?;
        
        // Return the original function (no wrapping needed in Rust)
        Ok(func)
    }
}

/// Create a group decorator that assigns commands to a group
/// 
/// This function returns a Python decorator that can be applied to commands.
/// It's equivalent to the Python @group decorator.
#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub fn group(kwargs: Option<&PyDict>) -> PyResult<GroupDecorator> {
    // Extract name and about from kwargs
    let name = kwargs
        .and_then(|d| d.get_item("name"))
        .map(|v| v.extract::<String>())
        .transpose()?
        .unwrap_or_else(|| "default".to_string());
        
    let about = kwargs
        .and_then(|d| d.get_item("about"))
        .map(|v| v.extract::<String>())
        .transpose()?;

    Ok(GroupDecorator { name, about })
}

/// Generate a reusable command group decorator
/// 
/// This function returns a decorator function that can be used multiple times
/// to assign commands to the same group. It's equivalent to Python's
/// functools.partial(group, name=name, about=about).
#[pyfunction]
#[pyo3(signature = (name, about = None))]
pub fn command_group(name: &str, about: Option<&str>) -> PyResult<GroupDecorator> {
    Ok(GroupDecorator {
        name: name.to_string(),
        about: about.map(|s| s.to_string()),
    })
}

// TODO: Implement the remaining decorators:
// - command  
// - argument