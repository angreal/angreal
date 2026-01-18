# venv


Virtual environment integration

This module provides Python bindings for virtual environment operations
using UV for fast venv creation and package installation.

## Structs

### `class VirtualEnv`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.venv.VirtualEnv](../../../angreal/python_bindings/venv.md#class-virtualenv)

Virtual environment manager

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `PathBuf` |  |
| `name` | `String` |  |
| `python_executable` | `PathBuf` |  |
| `python_version` | `Option < String >` |  |
| `requirements` | `Option < Py < PyAny > >` |  |
| `_is_activated` | `bool` |  |
| `_original_prefix` | `Option < String >` |  |
| `_original_path` | `Option < Vec < String > >` |  |
| `_original_env_path` | `Option < String >` |  |
| `_original_virtual_env` | `Option < Option < String > >` |  |

#### Methods

##### `__new__`

```rust
fn __new__ (path : Option < Py < PyAny > > , python : Option < & str > , requirements : Option < Py < PyAny > > , now : bool ,) -> PyResult < Self >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.__new__](../../../angreal/python_bindings/venv.md#__new__)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `create`

```rust
fn create (& self) -> PyResult < () >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.create](../../../angreal/python_bindings/venv.md#create)

<details>
<summary>Source</summary>

```rust
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



##### `activate`

```rust
fn activate (& mut self) -> PyResult < () >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.activate](../../../angreal/python_bindings/venv.md#activate)

<details>
<summary>Source</summary>

```rust
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



##### `remove`

```rust
fn remove (& self) -> PyResult < () >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.remove](../../../angreal/python_bindings/venv.md#remove)

<details>
<summary>Source</summary>

```rust
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



##### `__enter__`

```rust
fn __enter__ (mut slf : PyRefMut < Self >) -> PyResult < PyRefMut < Self > >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.__enter__](../../../angreal/python_bindings/venv.md#__enter__)

<details>
<summary>Source</summary>

```rust
    fn __enter__(mut slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        slf.create()?;
        slf.activate()?;
        Ok(slf)
    }
```

</details>



##### `__exit__`

```rust
fn __exit__ (& mut self , _exc_type : & Bound < '_ , PyAny > , _exc_val : & Bound < '_ , PyAny > , _exc_tb : & Bound < '_ , PyAny > ,) -> PyResult < () >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.__exit__](../../../angreal/python_bindings/venv.md#__exit__)

<details>
<summary>Source</summary>

```rust
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



##### `exists`

```rust
fn exists (& self , _py : Python) -> PyResult < bool >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.exists](../../../angreal/python_bindings/venv.md#exists)

<details>
<summary>Source</summary>

```rust
    fn exists(&self, _py: Python) -> PyResult<bool> {
        Ok(self.path.join("pyvenv.cfg").exists())
    }
```

</details>



##### `path`

```rust
fn path (& self , py : Python) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.path](../../../angreal/python_bindings/venv.md#path)

<details>
<summary>Source</summary>

```rust
    fn path(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.path.to_str().unwrap(),))?;
        Ok(result.into())
    }
```

</details>



##### `python_executable`

```rust
fn python_executable (& self , py : Python) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.python_executable](../../../angreal/python_bindings/venv.md#python_executable)

<details>
<summary>Source</summary>

```rust
    fn python_executable(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.python_executable.to_str().unwrap(),))?;
        Ok(result.into())
    }
```

</details>



##### `deactivate`

```rust
fn deactivate (& mut self) -> PyResult < () >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.deactivate](../../../angreal/python_bindings/venv.md#deactivate)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `install_requirements`

```rust
fn install_requirements (& self) -> PyResult < () >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.install_requirements](../../../angreal/python_bindings/venv.md#install_requirements)

<details>
<summary>Source</summary>

```rust
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



##### `install`

```rust
fn install (& self , packages : Py < PyAny >) -> PyResult < () >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.install](../../../angreal/python_bindings/venv.md#install)

<details>
<summary>Source</summary>

```rust
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



##### `discover_available_pythons`

```rust
fn discover_available_pythons (_cls : & Bound < '_ , PyType >) -> PyResult < Vec < (String , String) > >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.discover_available_pythons](../../../angreal/python_bindings/venv.md#discover_available_pythons)

<details>
<summary>Source</summary>

```rust
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



##### `ensure_python`

```rust
fn ensure_python (_cls : & Bound < '_ , PyType > , version : & str) -> PyResult < String >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.ensure_python](../../../angreal/python_bindings/venv.md#ensure_python)

<details>
<summary>Source</summary>

```rust
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



##### `version`

```rust
fn version (_cls : & Bound < '_ , PyType >) -> PyResult < String >
```

> **Python API**: [angreal.python_bindings.venv.VirtualEnv.version](../../../angreal/python_bindings/venv.md#version)

<details>
<summary>Source</summary>

```rust
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





### `class VenvRequiredDecorator`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.venv.VenvRequiredDecorator](../../../angreal/python_bindings/venv.md#class-venvrequireddecorator)

A Python callable that wraps the venv_required decorator logic

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `String` |  |
| `requirements` | `Option < Py < PyAny > >` |  |

#### Methods

##### `__call__`

```rust
fn __call__ (& self , py : Python , func : Py < PyAny >) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VenvRequiredDecorator.__call__](../../../angreal/python_bindings/venv.md#__call__)

<details>
<summary>Source</summary>

```rust
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
```

</details>





### `class VenvRequiredWrapper`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.venv.VenvRequiredWrapper](../../../angreal/python_bindings/venv.md#class-venvrequiredwrapper)

The actual wrapper function that handles venv lifecycle

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `original_func` | `Py < PyAny >` |  |
| `path` | `String` |  |
| `requirements` | `Option < Py < PyAny > >` |  |

#### Methods

##### `__call__`

```rust
fn __call__ (& self , args : & Bound < '_ , pyo3 :: types :: PyTuple > , kwargs : Option < & Bound < '_ , pyo3 :: types :: PyDict > > ,) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VenvRequiredWrapper.__call__](../../../angreal/python_bindings/venv.md#__call__)

<details>
<summary>Source</summary>

```rust
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



##### `get_arguments`

```rust
fn get_arguments (& self) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VenvRequiredWrapper.get_arguments](../../../angreal/python_bindings/venv.md#get_arguments)

<details>
<summary>Source</summary>

```rust
    fn get_arguments(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__arguments")
                .or_else(|_| Ok(py.None()))
        })
    }
```

</details>



##### `__name__`

```rust
fn __name__ (& self) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VenvRequiredWrapper.__name__](../../../angreal/python_bindings/venv.md#__name__)

<details>
<summary>Source</summary>

```rust
    fn __name__(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__name__")
                .or_else(|_| Ok(py.None()))
        })
    }
```

</details>



##### `__doc__`

```rust
fn __doc__ (& self) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VenvRequiredWrapper.__doc__](../../../angreal/python_bindings/venv.md#__doc__)

<details>
<summary>Source</summary>

```rust
    fn __doc__(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            self.original_func
                .getattr(py, "__doc__")
                .or_else(|_| Ok(py.None()))
        })
    }
```

</details>



##### `__getattr__`

```rust
fn __getattr__ (& self , name : & str) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.python_bindings.venv.VenvRequiredWrapper.__getattr__](../../../angreal/python_bindings/venv.md#__getattr__)

<details>
<summary>Source</summary>

```rust
    fn __getattr__(&self, name: &str) -> PyResult<Py<PyAny>> {
        Python::attach(|py| self.original_func.getattr(py, name))
    }
```

</details>





## Functions

### `fn venv_required`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.venv.venv_required](../../../angreal/python_bindings/venv.md#venv_required)

```rust
fn venv_required (path : & str , requirements : Option < Py < PyAny > > ,) -> PyResult < VenvRequiredDecorator >
```

Decorator that wraps a function to run in a virtual environment

This is equivalent to the Python @venv_required decorator

<details>
<summary>Source</summary>

```rust
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



### `fn register_venv`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn register_venv (_py : Python , m : & Bound < '_ , PyModule >) -> PyResult < () >
```

Register the venv module

<details>
<summary>Source</summary>

```rust
pub fn register_venv(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<VirtualEnv>()?;
    m.add_class::<VenvRequiredDecorator>()?;
    m.add_class::<VenvRequiredWrapper>()?;
    m.add_function(pyo3::wrap_pyfunction!(venv_required, m)?)?;
    Ok(())
}
```

</details>
