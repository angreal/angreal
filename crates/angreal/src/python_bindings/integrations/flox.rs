//! Flox environment integration submodule
//!
//! This module provides the flox submodule for angreal.integrations.flox

use crate::integrations::flox::{FloxEnvironment, FloxIntegration};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyType};
use std::collections::HashMap;
use std::path::PathBuf;

/// Flox environment manager
///
/// Provides environment activation and services management for Flox environments.
#[pyclass(name = "Flox")]
pub struct Flox {
    /// Path to the directory containing the Flox environment
    pub path: PathBuf,
    #[pyo3(get)]
    pub _is_activated: bool,
    /// Original environment state for restoration
    pub _original_env: Option<HashMap<String, String>>,
    /// Keys that were added during activation (to remove on deactivate)
    pub _added_keys: Option<Vec<String>>,
}

#[pymethods]
impl Flox {
    #[new]
    #[pyo3(signature = (path=None))]
    fn __new__(path: Option<Py<PyAny>>) -> PyResult<Self> {
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
                ".".to_string()
            };

            // Convert to Path and resolve to absolute path
            let path_buf = if path_str.starts_with('/') || path_str.starts_with('~') {
                PathBuf::from(&path_str)
            } else {
                std::env::current_dir()
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                            "Cannot get current directory: {}",
                            e
                        ))
                    })?
                    .join(&path_str)
            };

            // Resolve path using Python's pathlib for consistency
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

            Ok(Flox {
                path: resolved_path,
                _is_activated: false,
                _original_env: None,
                _added_keys: None,
            })
        })
    }

    /// Check if the Flox environment exists (.flox/ directory)
    #[getter]
    fn exists(&self) -> bool {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.exists()
    }

    /// Check if the manifest.toml exists
    #[getter]
    fn has_manifest(&self) -> bool {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.has_manifest()
    }

    /// Get the path as a Python Path object
    #[getter]
    fn path(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.path.to_str().unwrap(),))?;
        Ok(result.into())
    }

    /// Activate the Flox environment
    ///
    /// Applies environment variable modifications from `flox activate --print-script`
    /// to the current Python process's os.environ.
    fn activate(&mut self) -> PyResult<()> {
        if self._is_activated {
            return Ok(()); // Already activated
        }

        Python::attach(|py| {
            // Check if environment exists
            if !self.exists() {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Flox environment at {} does not exist",
                    self.path.display()
                )));
            }

            // Get activation environment from Flox
            let flox_env = FloxEnvironment::new(&self.path);
            let activation_env = flox_env.get_activation_env().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to get Flox activation environment: {}",
                    e
                ))
            })?;

            // Save current environment state
            let os = py.import("os")?;
            let environ = os.getattr("environ")?;

            let mut original_env = HashMap::new();
            let mut added_keys = Vec::new();

            // For each variable we're going to set, save the original value
            for key in activation_env.keys() {
                if let Ok(value) = environ.get_item(key) {
                    original_env.insert(key.clone(), value.extract::<String>()?);
                } else {
                    // Key doesn't exist, we'll need to remove it on deactivate
                    added_keys.push(key.clone());
                }
            }

            self._original_env = Some(original_env);
            self._added_keys = Some(added_keys);

            // Apply Flox environment variables
            for (key, value) in &activation_env {
                environ.set_item(key, value)?;
            }

            self._is_activated = true;
            Ok(())
        })
    }

    /// Deactivate the Flox environment
    ///
    /// Restores the original environment state.
    fn deactivate(&mut self) -> PyResult<()> {
        if !self._is_activated {
            return Ok(()); // Not activated
        }

        Python::attach(|py| {
            let os = py.import("os")?;
            let environ = os.getattr("environ")?;

            // Restore original values
            if let Some(ref original_env) = self._original_env {
                for (key, value) in original_env {
                    environ.set_item(key, value)?;
                }
            }

            // Remove keys that were added during activation
            if let Some(ref added_keys) = self._added_keys {
                for key in added_keys {
                    let _ = environ.call_method1("pop", (key, py.None()));
                }
            }

            self._is_activated = false;
            self._original_env = None;
            self._added_keys = None;
            Ok(())
        })
    }

    /// Context manager entry - activates the environment
    fn __enter__(mut slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        slf.activate()?;
        Ok(slf)
    }

    /// Context manager exit - deactivates the environment
    fn __exit__(
        &mut self,
        _exc_type: &Bound<'_, PyAny>,
        _exc_val: &Bound<'_, PyAny>,
        _exc_tb: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        self.deactivate()?;
        Ok(())
    }

    /// Check if the Flox CLI is available
    #[classmethod]
    fn is_available(_cls: &Bound<'_, PyType>) -> bool {
        FloxIntegration::is_available()
    }

    /// Get the Flox version string
    #[classmethod]
    fn version(_cls: &Bound<'_, PyType>) -> PyResult<String> {
        FloxIntegration::version().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get Flox version: {}",
                e
            ))
        })
    }

    /// Run a command within the Flox environment
    ///
    /// Executes: `flox activate -d <path> -- <command> [args...]`
    #[pyo3(signature = (command, args = None))]
    fn run(&self, command: &str, args: Option<Vec<String>>) -> PyResult<(i32, String, String)> {
        let flox_env = FloxEnvironment::new(&self.path);
        let args_refs: Vec<&str> = args
            .as_ref()
            .map(|a| a.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default();

        let output = flox_env.run_in_env(command, &args_refs).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to run command in Flox environment: {}",
                e
            ))
        })?;

        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok((exit_code, stdout, stderr))
    }

    /// Get a FloxServices manager for this environment
    #[getter]
    fn services(&self) -> FloxServices {
        FloxServices {
            path: self.path.clone(),
        }
    }
}

/// Information about a single service
#[pyclass(name = "ServiceInfo")]
#[derive(Clone)]
pub struct ServiceInfo {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub status: String,
    #[pyo3(get)]
    pub pid: Option<u32>,
}

#[pymethods]
impl ServiceInfo {
    fn __repr__(&self) -> String {
        match self.pid {
            Some(pid) => format!(
                "ServiceInfo(name='{}', status='{}', pid={})",
                self.name, self.status, pid
            ),
            None => format!(
                "ServiceInfo(name='{}', status='{}')",
                self.name, self.status
            ),
        }
    }

    /// Convert to a tuple (name, status, pid)
    fn as_tuple(&self) -> (String, String, Option<u32>) {
        (self.name.clone(), self.status.clone(), self.pid)
    }
}

/// Flox services manager
///
/// Provides methods for starting, stopping, and monitoring Flox services.
#[pyclass(name = "FloxServices")]
pub struct FloxServices {
    /// Path to the Flox environment
    pub path: PathBuf,
}

#[pymethods]
impl FloxServices {
    #[new]
    fn __new__(path: Py<PyAny>) -> PyResult<Self> {
        Python::attach(|py| {
            // Convert path to string - handle both str and Path objects
            let path_str = if let Ok(s) = path.extract::<String>(py) {
                s
            } else {
                // Try to convert Path object to string
                path.call_method0(py, "__str__")?.extract::<String>(py)?
            };

            // Convert to Path and resolve
            let path_buf = if path_str.starts_with('/') || path_str.starts_with('~') {
                PathBuf::from(&path_str)
            } else {
                std::env::current_dir()
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                            "Cannot get current directory: {}",
                            e
                        ))
                    })?
                    .join(&path_str)
            };

            // Resolve path using Python's pathlib for consistency
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

            Ok(FloxServices {
                path: resolved_path,
            })
        })
    }

    /// Get the path as a Python Path object
    #[getter]
    fn path(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.path.to_str().unwrap(),))?;
        Ok(result.into())
    }

    /// Start services
    ///
    /// If no service names are provided, starts all services defined in the manifest.
    /// Returns a FloxServiceHandle that can be used to stop services later.
    #[pyo3(signature = (*services))]
    fn start(&self, services: &Bound<'_, PyAny>) -> PyResult<FloxServiceHandle> {
        let flox_env = FloxEnvironment::new(&self.path);

        // Extract service names from *args
        let service_names: Vec<String> = if services.len()? > 0 {
            services.extract()?
        } else {
            Vec::new()
        };

        let service_refs: Vec<&str> = service_names.iter().map(|s| s.as_str()).collect();

        flox_env.services_start(&service_refs).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to start services: {}",
                e
            ))
        })?;

        // Get status to capture PIDs
        let statuses = flox_env.services_status().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get service status: {}",
                e
            ))
        })?;

        // Filter to only started services if specific ones were requested
        let service_infos: Vec<ServiceInfo> = statuses
            .into_iter()
            .filter(|s| {
                service_names.is_empty() || service_names.iter().any(|name| name == &s.name)
            })
            .map(|s| ServiceInfo {
                name: s.name,
                status: s.status,
                pid: s.pid,
            })
            .collect();

        Ok(FloxServiceHandle {
            flox_env_path: self.path.clone(),
            services: service_infos,
            started_at: chrono_now(),
        })
    }

    /// Stop all services
    fn stop(&self) -> PyResult<()> {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.services_stop().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to stop services: {}",
                e
            ))
        })
    }

    /// Get status of all services
    ///
    /// Returns a list of ServiceInfo objects.
    fn status(&self) -> PyResult<Vec<ServiceInfo>> {
        let flox_env = FloxEnvironment::new(&self.path);
        let statuses = flox_env.services_status().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get service status: {}",
                e
            ))
        })?;

        Ok(statuses
            .into_iter()
            .map(|s| ServiceInfo {
                name: s.name,
                status: s.status,
                pid: s.pid,
            })
            .collect())
    }

    /// Get logs for a specific service
    #[pyo3(signature = (service, follow=false, tail=None))]
    fn logs(&self, service: &str, follow: bool, tail: Option<u32>) -> PyResult<String> {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.services_logs(service, follow, tail).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get logs for service '{}': {}",
                service, e
            ))
        })
    }

    /// Restart services
    #[pyo3(signature = (*services))]
    fn restart(&self, services: &Bound<'_, PyAny>) -> PyResult<()> {
        let flox_env = FloxEnvironment::new(&self.path);

        let service_names: Vec<String> = if services.len()? > 0 {
            services.extract()?
        } else {
            Vec::new()
        };

        let service_refs: Vec<&str> = service_names.iter().map(|s| s.as_str()).collect();

        flox_env.services_restart(&service_refs).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to restart services: {}",
                e
            ))
        })
    }
}

/// Handle to started services for persistence and cleanup
///
/// Can be saved to JSON and loaded later to stop services across sessions.
#[pyclass(name = "FloxServiceHandle")]
#[derive(Clone)]
pub struct FloxServiceHandle {
    /// Path to the Flox environment
    pub flox_env_path: PathBuf,
    /// List of service info
    pub services: Vec<ServiceInfo>,
    /// Timestamp when services were started
    pub started_at: String,
}

/// Get current timestamp as ISO 8601 string
fn chrono_now() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    format!("{}Z", now.as_secs())
}

#[pymethods]
impl FloxServiceHandle {
    /// Get the Flox environment path
    #[getter]
    fn flox_env_path(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.flox_env_path.to_str().unwrap(),))?;
        Ok(result.into())
    }

    /// Get list of services
    #[getter]
    fn services(&self) -> Vec<ServiceInfo> {
        self.services.clone()
    }

    /// Get the started_at timestamp
    #[getter]
    fn started_at(&self) -> String {
        self.started_at.clone()
    }

    /// Stop the services tracked by this handle
    fn stop(&self) -> PyResult<()> {
        let flox_env = FloxEnvironment::new(&self.flox_env_path);
        flox_env.services_stop().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to stop services: {}",
                e
            ))
        })
    }

    /// Save handle to a JSON file
    #[pyo3(signature = (path=None))]
    fn save(&self, path: Option<&str>) -> PyResult<()> {
        let file_path = path.unwrap_or(".flox-services.json");

        Python::attach(|py| {
            let json = py.import("json")?;

            // Build the JSON structure
            let data = PyDict::new(py);
            data.set_item("flox_env_path", self.flox_env_path.to_str().unwrap())?;
            data.set_item("started_at", &self.started_at)?;

            // Build services list
            let services_list = PyList::empty(py);
            for svc in &self.services {
                let svc_dict = PyDict::new(py);
                svc_dict.set_item("name", &svc.name)?;
                svc_dict.set_item("status", &svc.status)?;
                svc_dict.set_item("pid", svc.pid)?;
                services_list.append(svc_dict)?;
            }
            data.set_item("services", services_list)?;

            // Write to file
            let builtins = py.import("builtins")?;
            let file = builtins.call_method1("open", (file_path, "w"))?;
            json.call_method1("dump", (data, &file))?;
            file.call_method0("close")?;

            Ok(())
        })
    }

    /// Load handle from a JSON file
    #[classmethod]
    #[pyo3(signature = (path=None))]
    fn load(_cls: &Bound<'_, PyType>, path: Option<&str>) -> PyResult<FloxServiceHandle> {
        let file_path = path.unwrap_or(".flox-services.json");

        Python::attach(|py| {
            let json = py.import("json")?;

            // Read from file
            let builtins = py.import("builtins")?;
            let file = builtins.call_method1("open", (file_path, "r"))?;
            let data: Bound<PyDict> = json.call_method1("load", (&file,))?.cast_into()?;
            file.call_method0("close")?;

            // Extract fields
            let flox_env_path = PathBuf::from(
                data.get_item("flox_env_path")?
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyKeyError, _>("missing flox_env_path")
                    })?
                    .extract::<String>()?,
            );

            let started_at = data
                .get_item("started_at")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("missing started_at"))?
                .extract::<String>()?;

            let services_list: Bound<PyList> = data
                .get_item("services")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("missing services"))?
                .cast_into()?;

            let mut services = Vec::new();
            for item in services_list.iter() {
                let svc_dict: Bound<PyDict> = item.cast_into()?;
                let name = svc_dict
                    .get_item("name")?
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyKeyError, _>("missing service name")
                    })?
                    .extract::<String>()?;
                let status = svc_dict
                    .get_item("status")?
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyKeyError, _>("missing service status")
                    })?
                    .extract::<String>()?;
                let pid = svc_dict
                    .get_item("pid")?
                    .map(|p| p.extract::<Option<u32>>())
                    .transpose()?
                    .flatten();

                services.push(ServiceInfo { name, status, pid });
            }

            Ok(FloxServiceHandle {
                flox_env_path,
                services,
                started_at,
            })
        })
    }

    fn __repr__(&self) -> String {
        let service_names: Vec<&str> = self.services.iter().map(|s| s.name.as_str()).collect();
        format!(
            "FloxServiceHandle(services={:?}, started_at='{}')",
            service_names, self.started_at
        )
    }
}

/// Decorator that wraps a function to run in a Flox environment
///
/// This is equivalent to the Python @flox_required decorator
#[pyfunction]
#[pyo3(signature = (path=None, services=None))]
pub fn flox_required(
    path: Option<Py<PyAny>>,
    services: Option<Vec<String>>,
) -> PyResult<FloxRequiredDecorator> {
    Python::attach(|py| {
        Ok(FloxRequiredDecorator {
            path: path.map(|p| p.clone_ref(py)),
            services,
        })
    })
}

/// A Python callable that wraps the flox_required decorator logic
#[pyclass]
pub struct FloxRequiredDecorator {
    path: Option<Py<PyAny>>,
    services: Option<Vec<String>>,
}

#[pymethods]
impl FloxRequiredDecorator {
    fn __call__(&self, py: Python, func: Py<PyAny>) -> PyResult<Py<PyAny>> {
        // Create a Rust-based wrapper function
        let wrapper = FloxRequiredWrapper {
            original_func: func,
            path: self.path.as_ref().map(|p| p.clone_ref(py)),
            services: self.services.clone(),
        };

        // Convert the Rust wrapper to a Python callable
        Ok(Py::new(py, wrapper)?.into())
    }
}

/// The actual wrapper function that handles Flox lifecycle
#[pyclass]
struct FloxRequiredWrapper {
    original_func: Py<PyAny>,
    path: Option<Py<PyAny>>,
    services: Option<Vec<String>>,
}

impl Clone for FloxRequiredWrapper {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            original_func: self.original_func.clone_ref(py),
            path: self.path.as_ref().map(|p| p.clone_ref(py)),
            services: self.services.clone(),
        })
    }
}

#[pymethods]
impl FloxRequiredWrapper {
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        args: &Bound<'_, pyo3::types::PyTuple>,
        kwargs: Option<&Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            // Create Flox instance
            let flox_class = py.get_type::<Flox>();
            let flox = if let Some(path) = &self.path {
                flox_class.call1((path,))?
            } else {
                flox_class.call0()?
            };

            // Check if environment exists
            let exists: bool = flox.getattr("exists")?.extract()?;
            if !exists {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Flox environment does not exist. Run 'flox init' first.",
                ));
            }

            // Activate the Flox environment
            flox.call_method0("activate")?;

            // Start services if specified
            let mut services_started = false;
            if let Some(service_list) = &self.services {
                if !service_list.is_empty() {
                    let services = flox.getattr("services")?;
                    // Convert Vec<String> to tuple for Python call
                    let service_tuple = pyo3::types::PyTuple::new(py, service_list)?;
                    services.call_method1("start", service_tuple)?;
                    services_started = true;
                }
            }

            // Call the original function and ensure cleanup happens
            let call_result = if let Some(kwargs) = kwargs {
                self.original_func.call(py, args, Some(kwargs))
            } else {
                self.original_func.call(py, args, None)
            };

            // Stop services if they were started (cleanup)
            if services_started {
                let services = flox.getattr("services")?;
                let _ = services.call_method0("stop");
            }

            // Always deactivate, regardless of success or failure
            let _ = flox.call_method0("deactivate");

            call_result
        })
    }

    // Proxy attributes from the wrapped function to support angreal's introspection
    // Note: angreal uses __arguments (single underscore), not __arguments__ (dunder)
    #[getter(__arguments)]
    fn get_arguments(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__arguments")
                .or_else(|_| Ok(py.None()))
        })
    }

    #[getter]
    fn __name__(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__name__")
                .or_else(|_| Ok(py.None()))
        })
    }

    #[getter]
    fn __doc__(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__doc__")
                .or_else(|_| Ok(py.None()))
        })
    }

    // Generic attribute access for any other attributes that might be needed
    fn __getattr__(&self, name: &str) -> PyResult<Py<PyAny>> {
        Python::attach(|py| self.original_func.getattr(py, name))
    }
}

/// Create the flox submodule
#[pymodule]
pub fn flox(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Flox>()?;
    m.add_class::<FloxServices>()?;
    m.add_class::<FloxServiceHandle>()?;
    m.add_class::<ServiceInfo>()?;
    m.add_class::<FloxRequiredDecorator>()?;
    m.add_class::<FloxRequiredWrapper>()?;
    m.add_function(pyo3::wrap_pyfunction!(flox_required, m)?)?;
    Ok(())
}

/// Register the flox module (for use in parent module)
pub fn register_flox(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Flox>()?;
    m.add_class::<FloxServices>()?;
    m.add_class::<FloxServiceHandle>()?;
    m.add_class::<ServiceInfo>()?;
    m.add_class::<FloxRequiredDecorator>()?;
    m.add_class::<FloxRequiredWrapper>()?;
    m.add_function(pyo3::wrap_pyfunction!(flox_required, m)?)?;
    Ok(())
}
