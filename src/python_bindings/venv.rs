//! Virtual environment integration
//!
//! This module provides Python bindings for virtual environment operations

use pyo3::prelude::*;
use std::path::PathBuf;
use std::process::Command;

/// Virtual environment manager
#[pyclass(name = "VirtualEnv")]
pub struct VirtualEnv {
    #[pyo3(get)]
    pub path: PathBuf,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub python_executable: PathBuf,
}

#[pymethods]
impl VirtualEnv {
    #[new]
    #[pyo3(signature = (name, now = false))]
    fn __new__(name: &str, now: bool) -> PyResult<Self> {
        let home_dir = home::home_dir()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyEnvironmentError, _>("Cannot determine home directory"))?;
        
        let path = home_dir.join(name);
        
        let python_executable = if cfg!(windows) {
            path.join("Scripts").join("python.exe")
        } else {
            path.join("bin").join("python")
        };
        
        let venv = VirtualEnv {
            path,
            name: name.to_string(),
            python_executable,
        };
        
        if now {
            venv.create()?;
        }
        
        Ok(venv)
    }
    
    fn create(&self) -> PyResult<()> {
        if self.path.exists() {
            return Ok(());
        }
        
        let output = Command::new("python")
            .args(["-m", "venv", &self.path.to_string_lossy()])
            .output()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to create virtual environment: {}", e)))?;
        
        if !output.status.success() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Virtual environment creation failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }
        
        Ok(())
    }
    
    fn activate(&self) -> PyResult<()> {
        // Note: This is a no-op in this implementation since we provide python_executable
        // The caller should use the python_executable path directly
        Ok(())
    }
    
    fn remove(&self) -> PyResult<()> {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to remove virtual environment: {}", e)))?;
        }
        Ok(())
    }
    
    fn __enter__(slf: PyRef<Self>) -> PyResult<PyRef<Self>> {
        slf.create()?;
        Ok(slf)
    }
    
    fn __exit__(&self, _exc_type: &PyAny, _exc_val: &PyAny, _exc_tb: &PyAny) -> PyResult<()> {
        // Context manager exit - can be used for cleanup if needed
        Ok(())
    }
}

/// Decorator that wraps a function to run in a virtual environment
/// 
/// This is equivalent to the Python @venv_required decorator
#[pyfunction]
#[pyo3(signature = (path, requirements = None))]
pub fn venv_required(path: &str, requirements: Option<PyObject>) -> PyResult<VenvRequiredDecorator> {
    Ok(VenvRequiredDecorator {
        path: path.to_string(),
        requirements,
    })
}

/// A Python callable that wraps the venv_required decorator logic
#[pyclass]
pub struct VenvRequiredDecorator {
    path: String,
    requirements: Option<PyObject>,
}

#[pymethods]
impl VenvRequiredDecorator {
    fn __call__(&self, py: Python, func: PyObject) -> PyResult<PyObject> {
        // Create a Rust-based wrapper function
        let wrapper = VenvRequiredWrapper {
            original_func: func,
            path: self.path.clone(),
            requirements: self.requirements.clone(),
        };
        
        // Convert the Rust wrapper to a Python callable
        Ok(wrapper.into_py(py))
    }
}

/// The actual wrapper function that handles venv lifecycle
#[pyclass]
struct VenvRequiredWrapper {
    original_func: PyObject,
    path: String,
    requirements: Option<PyObject>,
}

#[pymethods]
impl VenvRequiredWrapper {
    fn __call__(&self, py: Python, args: &pyo3::types::PyTuple, kwargs: Option<&pyo3::types::PyDict>) -> PyResult<PyObject> {
        // Create VirtualEnv with now=True
        let venv_class = py.get_type::<VirtualEnv>();
        let venv_kwargs = pyo3::types::PyDict::new(py);
        venv_kwargs.set_item("now", true)?;
        if let Some(reqs) = &self.requirements {
            venv_kwargs.set_item("requirements", reqs)?;
        }
        
        let venv = venv_class.call((&self.path,), Some(venv_kwargs))?;
        
        // Install requirements if any
        venv.call_method0("install_requirements")?;
        
        // Activate the venv
        venv.call_method0("activate")?;
        
        // Call the original function and ensure deactivation happens
        let call_result = if let Some(kwargs) = kwargs {
            self.original_func.call(py, args, Some(kwargs))
        } else {
            self.original_func.call(py, args, None)
        };
        
        // Always deactivate, regardless of success or failure
        let _ = venv.call_method0("deactivate");
        
        let result = call_result;
        
        result
    }
}

/// Register the venv module
pub fn register_venv(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<VirtualEnv>()?;
    m.add_class::<VenvRequiredDecorator>()?;
    m.add_class::<VenvRequiredWrapper>()?;
    m.add_function(pyo3::wrap_pyfunction!(venv_required, m)?)?;
    Ok(())
}