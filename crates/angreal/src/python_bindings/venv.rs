//! Virtual environment integration
//!
//! This module provides Python bindings for virtual environment operations

use pyo3::prelude::*;
use pyo3::types::PyType;
use std::path::PathBuf;
use std::process::Command;

/// Virtual environment manager
#[pyclass(name = "VirtualEnv")]
pub struct VirtualEnv {
    pub path: PathBuf,
    #[pyo3(get)]
    pub name: String,
    pub python_executable: PathBuf,
    pub python_version: Option<String>,
    pub requirements: Option<Py<PyAny>>,
    #[pyo3(get)]
    pub _is_activated: bool,
    pub _original_prefix: Option<String>,
    pub _original_path: Option<Vec<String>>,
    pub _original_env_path: Option<String>,
    pub _original_virtual_env: Option<Option<String>>,
}

#[pymethods]
impl VirtualEnv {
    #[new]
    #[pyo3(signature = (path=None, python=None, requirements=None, now=true))]
    fn __new__(
        path: Option<Py<PyAny>>,
        python: Option<&str>,
        requirements: Option<Py<PyAny>>,
        now: bool,
    ) -> PyResult<Self> {
        Python::attach(|py| {
            // Convert path to string - handle both str and Path objects, with default
            let path_str = if let Some(path_obj) = path {
                if let Ok(s) = path_obj.extract::<String>(py) {
                    s
                } else {
                    // Try to convert Path object to string
                    path_obj
                        .call_method0(py, "__str__")?
                        .extract::<String>(py)?
                }
            } else {
                ".venv".to_string()
            };

            // Convert to Path and resolve to absolute path
            let path_buf = if path_str.starts_with('/') || path_str.starts_with('~') {
                // Absolute path or home-relative path
                PathBuf::from(&path_str)
            } else {
                // Relative path - resolve from current directory
                std::env::current_dir()
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                            "Cannot get current directory: {}",
                            e
                        ))
                    })?
                    .join(&path_str)
            };

            // Use Python's Path.resolve() behavior instead of Rust's canonicalize()
            // to ensure consistent path resolution
            let resolved_path = {
                let pathlib = py.import("pathlib")?;
                let path_class = pathlib.getattr("Path")?;
                let py_path = path_class.call1((path_buf.to_str().unwrap(),))?;
                let resolved_py_path = py_path.call_method0("resolve")?;
                let resolved_str = resolved_py_path
                    .call_method0("__str__")?
                    .extract::<String>()?;
                PathBuf::from(resolved_str)
            };

            let python_executable = if cfg!(windows) {
                resolved_path.join("Scripts").join("python.exe")
            } else {
                resolved_path.join("bin").join("python")
            };

            let venv = VirtualEnv {
                path: resolved_path,
                name: path_str.to_string(),
                python_executable,
                python_version: python.map(|s| s.to_string()),
                requirements,
                _is_activated: false,
                _original_prefix: None,
                _original_path: None,
                _original_env_path: None,
                _original_virtual_env: None,
            };

            if now {
                venv.create()?;
            }

            Ok(venv)
        })
    }

    fn create(&self) -> PyResult<()> {
        if self.path.exists() {
            return Ok(());
        }

        // Try to create venv with pip support
        let output = if Command::new("uv").arg("--version").output().is_ok() {
            // Use UV with --seed to ensure pip is available
            Command::new("uv")
                .args(["venv", "--seed", &self.path.to_string_lossy()])
                .output()
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to create virtual environment with UV: {}",
                        e
                    ))
                })?
        } else {
            // Fall back to standard Python venv
            Command::new("python")
                .args(["-m", "venv", &self.path.to_string_lossy()])
                .output()
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to create virtual environment: {}",
                        e
                    ))
                })?
        };

        if !output.status.success() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Virtual environment creation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    fn activate(&mut self) -> PyResult<()> {
        if self._is_activated {
            return Ok(()); // Already activated
        }

        Python::attach(|py| {
            if !self.exists(py)? {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Virtual environment at {} does not exist",
                    self.path.display()
                )));
            }

            // Save current state
            let sys = py.import("sys")?;
            let current_prefix = sys.getattr("prefix")?.extract::<String>()?;
            let current_path = sys.getattr("path")?.extract::<Vec<String>>()?;

            self._original_prefix = Some(current_prefix);
            self._original_path = Some(current_path.clone());

            // Save current environment variables
            let os = py.import("os")?;
            let environ = os.getattr("environ")?;

            // Save current PATH
            let current_env_path = environ.get_item("PATH")?.extract::<String>()?;
            self._original_env_path = Some(current_env_path.clone());

            // Save current VIRTUAL_ENV (may not exist)
            let current_virtual_env = if let Ok(venv) = environ.get_item("VIRTUAL_ENV") {
                Some(venv.extract::<String>()?)
            } else {
                None
            };
            self._original_virtual_env = Some(current_virtual_env);

            // Set new prefix
            sys.setattr("prefix", self.path.to_str().unwrap())?;

            // Update sys.path to include venv's site-packages
            let site_packages = if cfg!(windows) {
                self.path.join("Lib").join("site-packages")
            } else {
                self.path
                    .join("lib")
                    .join(format!(
                        "python{}.{}",
                        py.version_info().major,
                        py.version_info().minor
                    ))
                    .join("site-packages")
            };

            let path_list = sys.getattr("path")?;
            path_list.call_method1("insert", (0, site_packages.to_str().unwrap()))?;

            // Update environment variables for subprocess calls
            // Prepend venv's bin directory to PATH
            let bin_dir = if cfg!(windows) {
                self.path.join("Scripts")
            } else {
                self.path.join("bin")
            };

            let new_path = format!("{}:{}", bin_dir.to_string_lossy(), current_env_path);
            environ.set_item("PATH", new_path)?;

            // Set VIRTUAL_ENV
            environ.set_item("VIRTUAL_ENV", self.path.to_str().unwrap())?;

            self._is_activated = true;
            Ok(())
        })
    }

    fn remove(&self) -> PyResult<()> {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                    "Failed to remove virtual environment: {}",
                    e
                ))
            })?;
        }
        Ok(())
    }

    fn __enter__(mut slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        slf.create()?;
        slf.activate()?;
        Ok(slf)
    }

    fn __exit__(
        &mut self,
        _exc_type: &Bound<'_, PyAny>,
        _exc_val: &Bound<'_, PyAny>,
        _exc_tb: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        self.deactivate()?;
        Ok(())
    }

    // Property to check if virtual environment exists
    #[getter]
    fn exists(&self, _py: Python) -> PyResult<bool> {
        Ok(self.path.join("pyvenv.cfg").exists())
    }

    // Custom getter for path property to return Python Path object
    #[getter]
    fn path(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.path.to_str().unwrap(),))?;
        Ok(result.into())
    }

    // Custom getter for python_executable property to return Python Path object
    #[getter]
    fn python_executable(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.python_executable.to_str().unwrap(),))?;
        Ok(result.into())
    }

    // Add deactivate method
    fn deactivate(&mut self) -> PyResult<()> {
        if !self._is_activated {
            return Ok(()); // Not activated
        }

        Python::attach(|py| {
            // Restore original state
            if let (Some(prefix), Some(path)) = (&self._original_prefix, &self._original_path) {
                let sys = py.import("sys")?;
                sys.setattr("prefix", prefix)?;

                // Clear sys.path and restore original
                let path_list = sys.getattr("path")?;
                path_list.call_method0("clear")?;
                for p in path {
                    path_list.call_method1("append", (p,))?;
                }
            }

            // Restore environment variables
            if let Some(original_env_path) = &self._original_env_path {
                let os = py.import("os")?;
                let environ = os.getattr("environ")?;

                // Restore PATH
                environ.set_item("PATH", original_env_path)?;

                // Restore or remove VIRTUAL_ENV
                if let Some(Some(original_venv)) = &self._original_virtual_env {
                    environ.set_item("VIRTUAL_ENV", original_venv)?;
                } else {
                    // VIRTUAL_ENV didn't exist before, so remove it
                    let _ = environ.call_method1("pop", ("VIRTUAL_ENV", py.None()));
                }
            }

            self._is_activated = false;
            self._original_prefix = None;
            self._original_path = None;
            self._original_env_path = None;
            self._original_virtual_env = None;
            Ok(())
        })
    }

    // Install requirements that were set during initialization
    fn install_requirements(&self) -> PyResult<()> {
        if let Some(reqs) = &self.requirements {
            // Validate requirements format first
            Python::attach(|py| {
                // Check if it's a string, list, or something else
                if reqs.extract::<String>(py).is_ok() || reqs.extract::<Vec<String>>(py).is_ok() {
                    self.install(reqs.clone_ref(py))
                } else {
                    // Try to convert to string for validation
                    match reqs.extract::<i32>(py) {
                        Ok(_) => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                            "requirements should be a string, list of strings, or Path object, not int",
                        )),
                        Err(_) => self.install(reqs.clone_ref(py)), // Let install handle the error
                    }
                }
            })
        } else {
            Ok(())
        }
    }

    // Install packages using pip
    fn install(&self, packages: Py<PyAny>) -> PyResult<()> {
        Python::attach(|py| {
            let pip_exe = if cfg!(windows) {
                self.path.join("Scripts").join("pip.exe")
            } else {
                self.path.join("bin").join("pip")
            };

            // Check if packages is a string, list, or Path object
            if let Ok(package_str) = packages.extract::<String>(py) {
                // Single package or requirements file
                if package_str.ends_with(".txt") {
                    // Requirements file
                    let output = Command::new(&pip_exe)
                        .arg("install")
                        .args(["-r", &package_str])
                        .output()
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to install requirements: {}",
                                e
                            ))
                        })?;

                    if !output.status.success() {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "pip install failed: {}",
                            String::from_utf8_lossy(&output.stderr)
                        )));
                    }
                } else {
                    // Single package
                    let output = Command::new(&pip_exe)
                        .arg("install")
                        .arg(&package_str)
                        .output()
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to install package: {}",
                                e
                            ))
                        })?;

                    if !output.status.success() {
                        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "pip install failed: {}",
                            String::from_utf8_lossy(&output.stderr)
                        )));
                    }
                }
            } else if let Ok(package_list) = packages.extract::<Vec<String>>(py) {
                // List of packages
                let output = Command::new(&pip_exe)
                    .arg("install")
                    .args(&package_list)
                    .output()
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to install packages: {}",
                            e
                        ))
                    })?;

                if !output.status.success() {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "pip install failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )));
                }
            } else {
                // Try to convert to string (for Path objects)
                let package_str = packages
                    .call_method0(py, "__str__")?
                    .extract::<String>(py)?;
                let output = Command::new(&pip_exe)
                    .arg("install")
                    .args(["-r", &package_str])
                    .output()
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to install requirements: {}",
                            e
                        ))
                    })?;

                if !output.status.success() {
                    return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "pip install failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )));
                }
            }

            Ok(())
        })
    }

    // Class methods for UV compatibility
    #[classmethod]
    fn discover_available_pythons(_cls: &Bound<'_, PyType>) -> PyResult<Vec<(String, String)>> {
        // Return some basic Python info for compatibility
        // In practice, this would use UV to discover installations
        Ok(vec![
            (
                "cpython-3.11.10".to_string(),
                "/usr/bin/python3.11".to_string(),
            ),
            (
                "cpython-3.12.9".to_string(),
                "/usr/bin/python3.12".to_string(),
            ),
        ])
    }

    #[classmethod]
    fn ensure_python(_cls: &Bound<'_, PyType>, version: &str) -> PyResult<String> {
        // This is a stub for UV compatibility
        // In a full implementation, this would ensure Python version is available
        Ok(format!("/usr/bin/python{}", version))
    }

    #[classmethod]
    fn version(_cls: &Bound<'_, PyType>) -> PyResult<String> {
        // Return a version string for compatibility
        Ok("uv 0.1.0 (stub)".to_string())
    }
}

/// Decorator that wraps a function to run in a virtual environment
///
/// This is equivalent to the Python @venv_required decorator
#[pyfunction]
#[pyo3(signature = (path, requirements = None))]
pub fn venv_required(
    path: &str,
    requirements: Option<Py<PyAny>>,
) -> PyResult<VenvRequiredDecorator> {
    Ok(VenvRequiredDecorator {
        path: path.to_string(),
        requirements,
    })
}

/// A Python callable that wraps the venv_required decorator logic
#[pyclass]
pub struct VenvRequiredDecorator {
    path: String,
    requirements: Option<Py<PyAny>>,
}

#[pymethods]
impl VenvRequiredDecorator {
    fn __call__(&self, py: Python, func: Py<PyAny>) -> PyResult<Py<PyAny>> {
        // Create a Rust-based wrapper function
        let wrapper = VenvRequiredWrapper {
            original_func: func,
            path: self.path.clone(),
            requirements: self.requirements.as_ref().map(|r| r.clone_ref(py)),
        };

        // Convert the Rust wrapper to a Python callable
        Ok(Py::new(py, wrapper)?.into())
    }
}

/// The actual wrapper function that handles venv lifecycle
#[pyclass]
struct VenvRequiredWrapper {
    original_func: Py<PyAny>,
    path: String,
    requirements: Option<Py<PyAny>>,
}

impl Clone for VenvRequiredWrapper {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            original_func: self.original_func.clone_ref(py),
            path: self.path.clone(),
            requirements: self.requirements.as_ref().map(|r| r.clone_ref(py)),
        })
    }
}

#[pymethods]
impl VenvRequiredWrapper {
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        args: &Bound<'_, pyo3::types::PyTuple>,
        kwargs: Option<&Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            // Create VirtualEnv with now=True
            let venv_class = py.get_type::<VirtualEnv>();
            let venv_kwargs = pyo3::types::PyDict::new(py);
            venv_kwargs.set_item("now", true)?;
            if let Some(reqs) = &self.requirements {
                venv_kwargs.set_item("requirements", reqs)?;
            }

            let venv = venv_class.call((&self.path,), Some(&venv_kwargs))?;

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

            call_result
        })
    }
}

/// Register the venv module
pub fn register_venv(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<VirtualEnv>()?;
    m.add_class::<VenvRequiredDecorator>()?;
    m.add_class::<VenvRequiredWrapper>()?;
    m.add_function(pyo3::wrap_pyfunction!(venv_required, m)?)?;
    Ok(())
}
