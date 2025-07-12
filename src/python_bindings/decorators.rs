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
    m.add_function(wrap_pyfunction!(command, m)?)?;
    m.add_class::<GroupDecorator>()?;
    m.add_class::<CommandDecorator>()?;
    // TODO: Add argument decorator
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

/// A Python callable that wraps the command decorator logic
#[pyclass]
pub struct CommandDecorator {
    name: Option<String>,
    about: Option<String>,
    long_about: Option<String>,
    when_to_use: Option<Vec<String>>,
    when_not_to_use: Option<Vec<String>>,
}

#[pymethods]
impl CommandDecorator {
    fn __call__(&self, py: Python, func: PyObject) -> PyResult<PyObject> {
        // Get or generate command name
        let name = match &self.name {
            Some(name) => name.clone(),
            None => {
                // Get function name and convert underscores to hyphens
                let func_name = func.getattr(py, "__name__")?
                    .extract::<String>(py)?
                    .to_lowercase()
                    .replace("_", "-");
                func_name
            }
        };
        
        // Initialize __arguments if not present
        if func.getattr(py, "__arguments").is_err() {
            func.setattr(py, "__arguments", py.None())?;
        }
        
        // Convert Option<Vec<String>> to Python objects
        let when_to_use_py = match &self.when_to_use {
            Some(vec) => vec.to_object(py),
            None => py.None(),
        };
        let when_not_to_use_py = match &self.when_not_to_use {
            Some(vec) => vec.to_object(py),
            None => py.None(),
        };
        
        // Create the AngrealCommand using PyO3's class instantiation
        let command_class = py.get_type::<crate::task::AngrealCommand>();
        let command = command_class.call1((
            &name,
            func.clone(),
            self.about.as_deref(),
            self.long_about.as_deref(),
            py.None(), // group (empty initially)
            when_to_use_py,
            when_not_to_use_py,
        ))?;
        
        // Set the __command attribute on the function
        func.setattr(py, "__command", command)?;
        
        // Process any existing arguments (this replicates the Python logic)
        let arguments = func.getattr(py, "__arguments")?;
        if !arguments.is_none(py) {
            if let Ok(args_list) = arguments.extract::<Vec<PyObject>>(py) {
                for arg_dict in args_list {
                    // Create Arg with the command name
                    if let Ok(arg_dict) = arg_dict.downcast::<pyo3::types::PyDict>(py) {
                        // Extract the arg name from the dictionary
                        let arg_name = arg_dict
                            .get_item("name")
                            .and_then(|v| v.extract::<String>().ok())
                            .unwrap_or_default();
                        
                        // Create AngrealArg using PyO3's class instantiation with kwargs
                        let arg_class = py.get_type::<crate::task::AngrealArg>();
                        
                        // Create kwargs dict for the constructor
                        let kwargs = PyDict::new(py);
                        kwargs.set_item("name", &arg_name)?;
                        kwargs.set_item("command_name", &name)?;
                        kwargs.set_item("default_value", py.None())?;
                        kwargs.set_item("is_flag", py.None())?;
                        kwargs.set_item("require_equals", py.None())?;
                        kwargs.set_item("multiple_values", py.None())?;
                        kwargs.set_item("number_of_values", py.None())?;
                        kwargs.set_item("max_values", py.None())?;
                        kwargs.set_item("min_values", py.None())?;
                        kwargs.set_item("short", py.None())?;
                        kwargs.set_item("long", py.None())?;
                        kwargs.set_item("long_help", py.None())?;
                        kwargs.set_item("help", py.None())?;
                        kwargs.set_item("required", py.None())?;
                        kwargs.set_item("takes_value", py.None())?;
                        kwargs.set_item("python_type", py.None())?;
                        
                        // Call with empty args and kwargs
                        let _arg = arg_class.call((), Some(kwargs))?;
                    }
                }
            }
        }
        
        Ok(func)
    }
}

/// Create a command decorator that registers functions as commands
/// 
/// This function returns a Python decorator that can be applied to functions.
/// It's equivalent to the Python @command decorator.
#[pyfunction]
#[pyo3(signature = (**kwargs))]
pub fn command(kwargs: Option<&PyDict>) -> PyResult<CommandDecorator> {
    // Extract parameters from kwargs
    let name = kwargs
        .and_then(|d| d.get_item("name"))
        .map(|v| v.extract::<String>())
        .transpose()?;
        
    let about = kwargs
        .and_then(|d| d.get_item("about"))
        .map(|v| v.extract::<String>())
        .transpose()?;
        
    let long_about = kwargs
        .and_then(|d| d.get_item("long_about"))
        .map(|v| v.extract::<String>())
        .transpose()?;
        
    let when_to_use = kwargs
        .and_then(|d| d.get_item("when_to_use"))
        .map(|v| v.extract::<Vec<String>>())
        .transpose()?;
        
    let when_not_to_use = kwargs
        .and_then(|d| d.get_item("when_not_to_use"))
        .map(|v| v.extract::<Vec<String>>())
        .transpose()?;

    Ok(CommandDecorator {
        name,
        about,
        long_about,
        when_to_use,
        when_not_to_use,
    })
}

// TODO: Implement the remaining decorators:
// - argument