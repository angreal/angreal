---
title: "angreal.python_bindings.venv"
description: "Virtual environment integration"
sidebar:
  order: 9
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


Virtual environment integration

This module provides Python bindings for virtual environment operations
using UV for fast venv creation and package installation.

## Classes

### <span id="class.VirtualEnv"></span>`class VirtualEnv`

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv](../../../rust/angreal/python_bindings/venv/#struct.VirtualEnv)

Virtual environment manager

#### Methods

##### <span id="VirtualEnv.__new__"></span>`__new__`

```python
def __new__(
    path: Option < Py < PyAny > >,
    python: Option < & str >,
    requirements: Option < Py < PyAny > >,
    now: bool,
) -> PyResult < Self >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::__new__](../../../rust/angreal/python_bindings/venv/#VirtualEnv.__new__)

<details>
<summary>Source</summary>

```python
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

            // Expand tilde to home directory using Python's Path.expanduser()
            let path_str = if path_str.starts_with('~') {
                let pathlib = py.import("pathlib")?;
                let path_class = pathlib.getattr("Path")?;
                let py_path = path_class.call1((&path_str,))?;
                let expanded = py_path.call_method0("expanduser")?;
                expanded.call_method0("__str__")?.extract::<String>()?
            } else {
                path_str
            };

            // Convert to Path and resolve to absolute path
            let path_buf = if path_str.starts_with('/') {
                // Absolute path
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
                _original_exec_prefix: None,
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
```

</details>



##### <span id="VirtualEnv.create"></span>`create`

```python
def create() -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::create](../../../rust/angreal/python_bindings/venv/#VirtualEnv.create)

<details>
<summary>Source</summary>

```python
    fn create(&self) -> PyResult<()> {
        if self.path.exists() {
            return Ok(());
        }

        // Use UvVirtualEnv for creation - it handles UV installation and fallback
        UvVirtualEnv::create(&self.path, self.python_version.as_deref()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create virtual environment: {}",
                e
            ))
        })?;

        Ok(())
    }
```

</details>



##### <span id="VirtualEnv.activate"></span>`activate`

```python
def activate() -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::activate](../../../rust/angreal/python_bindings/venv/#VirtualEnv.activate)

<details>
<summary>Source</summary>

```python
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
            let current_exec_prefix = sys.getattr("exec_prefix")?.extract::<String>()?;
            let current_path = sys.getattr("path")?.extract::<Vec<String>>()?;

            self._original_prefix = Some(current_prefix);
            self._original_exec_prefix = Some(current_exec_prefix);
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

            // Set new prefix and exec_prefix
            sys.setattr("prefix", self.path.to_str().unwrap())?;
            sys.setattr("exec_prefix", self.path.to_str().unwrap())?;

            // Discover the actual site-packages directory from the venv.
            // Don't assume the venv's Python version matches the host — UV may
            // have created the venv with a different Python than the one running.
            let site_packages = if cfg!(windows) {
                self.path.join("Lib").join("site-packages")
            } else {
                // Find the actual python* directory under lib/
                let lib_dir = self.path.join("lib");
                let mut found = None;
                if let Ok(entries) = std::fs::read_dir(&lib_dir) {
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        if name_str.starts_with("python") {
                            let sp = entry.path().join("site-packages");
                            if sp.exists() {
                                found = Some(sp);
                                break;
                            }
                        }
                    }
                }
                found.unwrap_or_else(|| {
                    // Fallback to host Python version if discovery fails
                    lib_dir
                        .join(format!(
                            "python{}.{}",
                            py.version_info().major,
                            py.version_info().minor
                        ))
                        .join("site-packages")
                })
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

            // Use platform-specific PATH separator
            let path_sep = if cfg!(windows) { ";" } else { ":" };
            let new_path = format!(
                "{}{}{}",
                bin_dir.to_string_lossy(),
                path_sep,
                current_env_path
            );
            environ.set_item("PATH", new_path)?;

            // Set VIRTUAL_ENV
            environ.set_item("VIRTUAL_ENV", self.path.to_str().unwrap())?;

            self._is_activated = true;
            Ok(())
        })
    }
```

</details>



##### <span id="VirtualEnv.remove"></span>`remove`

```python
def remove() -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::remove](../../../rust/angreal/python_bindings/venv/#VirtualEnv.remove)

<details>
<summary>Source</summary>

```python
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
```

</details>



##### <span id="VirtualEnv.__enter__"></span>`__enter__`

```python
def __enter__(slf: PyRefMut < Self >) -> PyResult < PyRefMut < Self > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::__enter__](../../../rust/angreal/python_bindings/venv/#VirtualEnv.__enter__)

<details>
<summary>Source</summary>

```python
    fn __enter__(mut slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        slf.create()?;
        slf.install_requirements()?;
        slf.activate()?;
        Ok(slf)
    }
```

</details>



##### <span id="VirtualEnv.__exit__"></span>`__exit__`

```python
def __exit__(
    _exc_type:  Bound < '_ , PyAny >,
    _exc_val:  Bound < '_ , PyAny >,
    _exc_tb:  Bound < '_ , PyAny >,
) -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::__exit__](../../../rust/angreal/python_bindings/venv/#VirtualEnv.__exit__)

<details>
<summary>Source</summary>

```python
    fn __exit__(
        &mut self,
        _exc_type: &Bound<'_, PyAny>,
        _exc_val: &Bound<'_, PyAny>,
        _exc_tb: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        self.deactivate()?;
        Ok(())
    }
```

</details>



##### <span id="VirtualEnv.exists"></span>`exists`

```python
def exists(_py: Python) -> PyResult < bool >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::exists](../../../rust/angreal/python_bindings/venv/#VirtualEnv.exists)

<details>
<summary>Source</summary>

```python
    fn exists(&self, _py: Python) -> PyResult<bool> {
        Ok(self.path.join("pyvenv.cfg").exists())
    }
```

</details>



##### <span id="VirtualEnv.path"></span>`path`

```python
def path() -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::path](../../../rust/angreal/python_bindings/venv/#VirtualEnv.path)

<details>
<summary>Source</summary>

```python
    fn path(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.path.to_str().unwrap(),))?;
        Ok(result.into())
    }
```

</details>



##### <span id="VirtualEnv.python_executable"></span>`python_executable`

```python
def python_executable() -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::python_executable](../../../rust/angreal/python_bindings/venv/#VirtualEnv.python_executable)

<details>
<summary>Source</summary>

```python
    fn python_executable(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.python_executable.to_str().unwrap(),))?;
        Ok(result.into())
    }
```

</details>



##### <span id="VirtualEnv.deactivate"></span>`deactivate`

```python
def deactivate() -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::deactivate](../../../rust/angreal/python_bindings/venv/#VirtualEnv.deactivate)

<details>
<summary>Source</summary>

```python
    fn deactivate(&mut self) -> PyResult<()> {
        if !self._is_activated {
            return Ok(()); // Not activated
        }

        Python::attach(|py| {
            // Restore original state
            if let (Some(prefix), Some(path)) = (&self._original_prefix, &self._original_path) {
                let sys = py.import("sys")?;
                sys.setattr("prefix", prefix)?;
                if let Some(exec_prefix) = &self._original_exec_prefix {
                    sys.setattr("exec_prefix", exec_prefix)?;
                }

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
            self._original_exec_prefix = None;
            self._original_path = None;
            self._original_env_path = None;
            self._original_virtual_env = None;
            Ok(())
        })
    }
```

</details>



##### <span id="VirtualEnv.install_requirements"></span>`install_requirements`

```python
def install_requirements() -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::install_requirements](../../../rust/angreal/python_bindings/venv/#VirtualEnv.install_requirements)

<details>
<summary>Source</summary>

```python
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
```

</details>



##### <span id="VirtualEnv.install"></span>`install`

```python
def install(packages: Py < PyAny >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::install](../../../rust/angreal/python_bindings/venv/#VirtualEnv.install)

<details>
<summary>Source</summary>

```python
    fn install(&self, packages: Py<PyAny>) -> PyResult<()> {
        // Create UvVirtualEnv instance for this venv
        let uv_venv = UvVirtualEnv {
            path: self.path.clone(),
        };

        Python::attach(|py| {
            // Check if packages is a string, list, or Path object
            if let Ok(package_str) = packages.extract::<String>(py) {
                // Single package or requirements file
                if package_str.ends_with(".txt") {
                    // Requirements file - use UV's install_requirements
                    uv_venv
                        .install_requirements(std::path::Path::new(&package_str))
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                                "Failed to install requirements: {}",
                                e
                            ))
                        })?;
                } else {
                    // Single package - use UV's install_packages
                    uv_venv.install_packages(&[package_str]).map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to install package: {}",
                            e
                        ))
                    })?;
                }
            } else if let Ok(package_list) = packages.extract::<Vec<String>>(py) {
                // List of packages - use UV's install_packages
                uv_venv.install_packages(&package_list).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to install packages: {}",
                        e
                    ))
                })?;
            } else {
                // Try to convert to string (for Path objects) - treat as requirements file
                let package_str = packages
                    .call_method0(py, "__str__")?
                    .extract::<String>(py)?;
                uv_venv
                    .install_requirements(std::path::Path::new(&package_str))
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to install requirements: {}",
                            e
                        ))
                    })?;
            }

            Ok(())
        })
    }
```

</details>



##### <span id="VirtualEnv.discover_available_pythons"></span>`discover_available_pythons`

```python
def discover_available_pythons(
    _cls:  Bound < '_ , PyType >,
) -> PyResult < Vec < (String , String) > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::discover_available_pythons](../../../rust/angreal/python_bindings/venv/#VirtualEnv.discover_available_pythons)

<details>
<summary>Source</summary>

```python
    fn discover_available_pythons(_cls: &Bound<'_, PyType>) -> PyResult<Vec<(String, String)>> {
        // Use UV to discover available Python installations
        UvVirtualEnv::discover_pythons()
            .map(|pythons| {
                pythons
                    .into_iter()
                    .map(|(version, path)| (version, path.display().to_string()))
                    .collect()
            })
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to discover Python installations: {}",
                    e
                ))
            })
    }
```

</details>



##### <span id="VirtualEnv.ensure_python"></span>`ensure_python`

```python
def ensure_python(_cls:  Bound < '_ , PyType >, version:  str) -> PyResult < String >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::ensure_python](../../../rust/angreal/python_bindings/venv/#VirtualEnv.ensure_python)

<details>
<summary>Source</summary>

```python
    fn ensure_python(_cls: &Bound<'_, PyType>, version: &str) -> PyResult<String> {
        // Use UV to install/ensure Python version is available
        UvVirtualEnv::install_python(version)
            .map(|path| path.display().to_string())
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to ensure Python {}: {}",
                    version, e
                ))
            })
    }
```

</details>



##### <span id="VirtualEnv.version"></span>`version`

```python
def version(_cls:  Bound < '_ , PyType >) -> PyResult < String >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VirtualEnv::version](../../../rust/angreal/python_bindings/venv/#VirtualEnv.version)

<details>
<summary>Source</summary>

```python
    fn version(_cls: &Bound<'_, PyType>) -> PyResult<String> {
        // Return the actual UV version
        UvIntegration::version().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get UV version: {}",
                e
            ))
        })
    }
```

</details>





### <span id="class.VenvRequiredDecorator"></span>`class VenvRequiredDecorator`

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredDecorator](../../../rust/angreal/python_bindings/venv/#struct.VenvRequiredDecorator)

A Python callable that wraps the venv_required decorator logic

#### Methods

##### <span id="VenvRequiredDecorator.__call__"></span>`__call__`

```python
def __call__(func: Py < PyAny >) -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredDecorator::__call__](../../../rust/angreal/python_bindings/venv/#VenvRequiredDecorator.__call__)

<details>
<summary>Source</summary>

```python
    fn __call__(&self, py: Python, func: Py<PyAny>) -> PyResult<Py<PyAny>> {
        // Create a Rust-based wrapper function
        let wrapper = VenvRequiredWrapper {
            original_func: func,
            path: self.path.clone(),
            requirements: self.requirements.as_ref().map(|r| r.clone_ref(py)),
            command: None,
            arguments: None,
        };

        // Convert the Rust wrapper to a Python callable
        Ok(Py::new(py, wrapper)?.into())
    }
```

</details>





### <span id="class.VenvRequiredWrapper"></span>`class VenvRequiredWrapper`

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper](../../../rust/angreal/python_bindings/venv/#struct.VenvRequiredWrapper)

#### Methods

##### <span id="VenvRequiredWrapper.__call__"></span>`__call__`

```python
def __call__(
    args:  Bound < '_ , pyo3 :: types :: PyTuple >,
    kwargs: Option < & Bound < '_ , pyo3 :: types :: PyDict > >,
) -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::__call__](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.__call__)

<details>
<summary>Source</summary>

```python
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
```

</details>



##### <span id="VenvRequiredWrapper.get_command"></span>`get_command`

```python
def get_command() -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::get_command](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.get_command)

<details>
<summary>Source</summary>

```python
    fn get_command(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.command
                .as_ref()
                .map(|c| c.clone_ref(py))
                .ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyAttributeError, _>("__command not set")
                })
        })
    }
```

</details>



##### <span id="VenvRequiredWrapper.set_command"></span>`set_command`

```python
def set_command(value: Py < PyAny >)
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::set_command](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.set_command)

<details>
<summary>Source</summary>

```python
    fn set_command(&mut self, value: Py<PyAny>) {
        self.command = Some(value);
    }
```

</details>



##### <span id="VenvRequiredWrapper.get_arguments"></span>`get_arguments`

```python
def get_arguments() -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::get_arguments](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.get_arguments)

<details>
<summary>Source</summary>

```python
    fn get_arguments(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            if let Some(args) = &self.arguments {
                Ok(args.clone_ref(py))
            } else {
                self.original_func
                    .getattr(py, "__arguments")
                    .or_else(|_| Ok(py.None()))
            }
        })
    }
```

</details>



##### <span id="VenvRequiredWrapper.set_arguments"></span>`set_arguments`

```python
def set_arguments(value: Py < PyAny >)
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::set_arguments](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.set_arguments)

<details>
<summary>Source</summary>

```python
    fn set_arguments(&mut self, value: Py<PyAny>) {
        self.arguments = Some(value);
    }
```

</details>



##### <span id="VenvRequiredWrapper.__name__"></span>`__name__`

```python
def __name__() -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::__name__](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.__name__)

<details>
<summary>Source</summary>

```python
    fn __name__(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__name__")
                .or_else(|_| Ok(py.None()))
        })
    }
```

</details>



##### <span id="VenvRequiredWrapper.__doc__"></span>`__doc__`

```python
def __doc__() -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::__doc__](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.__doc__)

<details>
<summary>Source</summary>

```python
    fn __doc__(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__doc__")
                .or_else(|_| Ok(py.None()))
        })
    }
```

</details>



##### <span id="VenvRequiredWrapper.__getattr__"></span>`__getattr__`

```python
def __getattr__(name:  str) -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::venv::VenvRequiredWrapper::__getattr__](../../../rust/angreal/python_bindings/venv/#VenvRequiredWrapper.__getattr__)

<details>
<summary>Source</summary>

```python
    fn __getattr__(&self, name: &str) -> PyResult<Py<PyAny>> {
        Python::attach(|py| self.original_func.getattr(py, name))
    }
```

</details>





## Functions

### <span id="fn.venv_required"></span>`venv_required`

```python
def venv_required(
    path:  str,
    requirements: Option < Py < PyAny > >,
) -> PyResult < VenvRequiredDecorator >
```

> **Rust Implementation**: [angreal::python_bindings::venv::venv_required](../../../rust/angreal/python_bindings/venv/#fn.venv_required)

Decorator that wraps a function to run in a virtual environment

This is equivalent to the Python @venv_required decorator

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `path` | ` str` |  |
| `requirements` | `Option < Py < PyAny > >` |  |


<details>
<summary>Source</summary>

```python
pub fn venv_required(
    path: &str,
    requirements: Option<Py<PyAny>>,
) -> PyResult<VenvRequiredDecorator> {
    Ok(VenvRequiredDecorator {
        path: path.to_string(),
        requirements,
    })
}
```

</details>
