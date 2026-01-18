# flox


Flox environment integration submodule

This module provides the flox submodule for angreal.integrations.flox

## Classes

### `class Flox`

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox](../../../rust/angreal/python_bindings/integrations/flox.md#class-flox)

Flox environment manager

Provides environment activation and services management for Flox environments.

#### Methods

##### `__new__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__new__</span>(path: Option &lt; Py &lt; PyAny &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Self &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::__new__](../../../rust/angreal/python_bindings/integrations/flox.md#__new__)

<details>
<summary>Source</summary>

```python
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
```

</details>



##### `exists`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">exists</span>() -> <span style="color: var(--md-default-fg-color--light);">bool</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::exists](../../../rust/angreal/python_bindings/integrations/flox.md#exists)

Check if the Flox environment exists (.flox/ directory)

<details>
<summary>Source</summary>

```python
    fn exists(&self) -> bool {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.exists()
    }
```

</details>



##### `has_manifest`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">has_manifest</span>() -> <span style="color: var(--md-default-fg-color--light);">bool</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::has_manifest](../../../rust/angreal/python_bindings/integrations/flox.md#has_manifest)

Check if the manifest.toml exists

<details>
<summary>Source</summary>

```python
    fn has_manifest(&self) -> bool {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.has_manifest()
    }
```

</details>



##### `path`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">path</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::path](../../../rust/angreal/python_bindings/integrations/flox.md#path)

Get the path as a Python Path object

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



##### `activate`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">activate</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::activate](../../../rust/angreal/python_bindings/integrations/flox.md#activate)

Activate the Flox environment

Applies environment variable modifications from `flox activate --print-script`
to the current Python process's os.environ.

<details>
<summary>Source</summary>

```python
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
```

</details>



##### `deactivate`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">deactivate</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::deactivate](../../../rust/angreal/python_bindings/integrations/flox.md#deactivate)

Deactivate the Flox environment

Restores the original environment state.

<details>
<summary>Source</summary>

```python
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
```

</details>



##### `__enter__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__enter__</span>(slf: PyRefMut &lt; Self &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyRefMut &lt; Self &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::__enter__](../../../rust/angreal/python_bindings/integrations/flox.md#__enter__)

Context manager entry - activates the environment

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `slf` | `PyRefMut < Self >` |  |


<details>
<summary>Source</summary>

```python
    fn __enter__(mut slf: PyRefMut<Self>) -> PyResult<PyRefMut<Self>> {
        slf.activate()?;
        Ok(slf)
    }
```

</details>



##### `__exit__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__exit__</span>(_exc_type:  Bound &lt; &#x27;_ , PyAny &gt;, _exc_val:  Bound &lt; &#x27;_ , PyAny &gt;, _exc_tb:  Bound &lt; &#x27;_ , PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::__exit__](../../../rust/angreal/python_bindings/integrations/flox.md#__exit__)

Context manager exit - deactivates the environment

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `_exc_type` | ` Bound < '_ , PyAny >` |  |
| `_exc_val` | ` Bound < '_ , PyAny >` |  |
| `_exc_tb` | ` Bound < '_ , PyAny >` |  |


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



##### `is_available`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">is_available</span>(_cls:  Bound &lt; &#x27;_ , PyType &gt;) -> <span style="color: var(--md-default-fg-color--light);">bool</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::is_available](../../../rust/angreal/python_bindings/integrations/flox.md#is_available)

Check if the Flox CLI is available

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `_cls` | ` Bound < '_ , PyType >` |  |


<details>
<summary>Source</summary>

```python
    fn is_available(_cls: &Bound<'_, PyType>) -> bool {
        FloxIntegration::is_available()
    }
```

</details>



##### `version`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">version</span>(_cls:  Bound &lt; &#x27;_ , PyType &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::version](../../../rust/angreal/python_bindings/integrations/flox.md#version)

Get the Flox version string

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `_cls` | ` Bound < '_ , PyType >` |  |


<details>
<summary>Source</summary>

```python
    fn version(_cls: &Bound<'_, PyType>) -> PyResult<String> {
        FloxIntegration::version().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get Flox version: {}",
                e
            ))
        })
    }
```

</details>



##### `run`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">run</span>(command:  str, args: Option &lt; Vec &lt; String &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , String , String) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::run](../../../rust/angreal/python_bindings/integrations/flox.md#run)

Run a command within the Flox environment

Executes: `flox activate -d <path> -- <command> [args...]`

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `command` | ` str` |  |
| `args` | `Option < Vec < String > >` |  |


<details>
<summary>Source</summary>

```python
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
```

</details>



##### `services`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">services</span>() -> <span style="color: var(--md-default-fg-color--light);">FloxServices</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::Flox::services](../../../rust/angreal/python_bindings/integrations/flox.md#services)

Get a FloxServices manager for this environment

<details>
<summary>Source</summary>

```python
    fn services(&self) -> FloxServices {
        FloxServices {
            path: self.path.clone(),
        }
    }
```

</details>





### `class ServiceInfo`

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::ServiceInfo](../../../rust/angreal/python_bindings/integrations/flox.md#class-serviceinfo)

Information about a single service

#### Methods

##### `__repr__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__repr__</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::ServiceInfo::__repr__](../../../rust/angreal/python_bindings/integrations/flox.md#__repr__)

<details>
<summary>Source</summary>

```python
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
```

</details>



##### `as_tuple`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">as_tuple</span>() -> <span style="color: var(--md-default-fg-color--light);">(String , String , Option &lt; u32 &gt;)</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::ServiceInfo::as_tuple](../../../rust/angreal/python_bindings/integrations/flox.md#as_tuple)

Convert to a tuple (name, status, pid)

<details>
<summary>Source</summary>

```python
    fn as_tuple(&self) -> (String, String, Option<u32>) {
        (self.name.clone(), self.status.clone(), self.pid)
    }
```

</details>





### `class FloxServices`

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices](../../../rust/angreal/python_bindings/integrations/flox.md#class-floxservices)

Flox services manager

Provides methods for starting, stopping, and monitoring Flox services.

#### Methods

##### `__new__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__new__</span>(path: Py &lt; PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Self &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices::__new__](../../../rust/angreal/python_bindings/integrations/flox.md#__new__)

<details>
<summary>Source</summary>

```python
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
```

</details>



##### `path`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">path</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices::path](../../../rust/angreal/python_bindings/integrations/flox.md#path)

Get the path as a Python Path object

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



##### `start`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">start</span>(services:  Bound &lt; &#x27;_ , PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; FloxServiceHandle &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices::start](../../../rust/angreal/python_bindings/integrations/flox.md#start)

Start services

If no service names are provided, starts all services defined in the manifest.
Returns a FloxServiceHandle that can be used to stop services later.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | ` Bound < '_ , PyAny >` |  |


<details>
<summary>Source</summary>

```python
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
```

</details>



##### `stop`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">stop</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices::stop](../../../rust/angreal/python_bindings/integrations/flox.md#stop)

Stop all services

<details>
<summary>Source</summary>

```python
    fn stop(&self) -> PyResult<()> {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.services_stop().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to stop services: {}",
                e
            ))
        })
    }
```

</details>



##### `status`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">status</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Vec &lt; ServiceInfo &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices::status](../../../rust/angreal/python_bindings/integrations/flox.md#status)

Get status of all services

Returns a list of ServiceInfo objects.

<details>
<summary>Source</summary>

```python
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
```

</details>



##### `logs`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">logs</span>(service:  str, follow: bool, tail: Option &lt; u32 &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices::logs](../../../rust/angreal/python_bindings/integrations/flox.md#logs)

Get logs for a specific service

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `service` | ` str` |  |
| `follow` | `bool` |  |
| `tail` | `Option < u32 >` |  |


<details>
<summary>Source</summary>

```python
    fn logs(&self, service: &str, follow: bool, tail: Option<u32>) -> PyResult<String> {
        let flox_env = FloxEnvironment::new(&self.path);
        flox_env.services_logs(service, follow, tail).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get logs for service '{}': {}",
                service, e
            ))
        })
    }
```

</details>



##### `restart`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">restart</span>(services:  Bound &lt; &#x27;_ , PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServices::restart](../../../rust/angreal/python_bindings/integrations/flox.md#restart)

Restart services

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | ` Bound < '_ , PyAny >` |  |


<details>
<summary>Source</summary>

```python
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
```

</details>





### `class FloxServiceHandle`

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle](../../../rust/angreal/python_bindings/integrations/flox.md#class-floxservicehandle)

Handle to started services for persistence and cleanup

Can be saved to JSON and loaded later to stop services across sessions.

#### Methods

##### `flox_env_path`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">flox_env_path</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle::flox_env_path](../../../rust/angreal/python_bindings/integrations/flox.md#flox_env_path)

Get the Flox environment path

<details>
<summary>Source</summary>

```python
    fn flox_env_path(&self, py: Python) -> PyResult<Py<PyAny>> {
        let pathlib = py.import("pathlib")?;
        let path_class = pathlib.getattr("Path")?;
        let result = path_class.call1((self.flox_env_path.to_str().unwrap(),))?;
        Ok(result.into())
    }
```

</details>



##### `services`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">services</span>() -> <span style="color: var(--md-default-fg-color--light);">Vec &lt; ServiceInfo &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle::services](../../../rust/angreal/python_bindings/integrations/flox.md#services)

Get list of services

<details>
<summary>Source</summary>

```python
    fn services(&self) -> Vec<ServiceInfo> {
        self.services.clone()
    }
```

</details>



##### `started_at`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">started_at</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle::started_at](../../../rust/angreal/python_bindings/integrations/flox.md#started_at)

Get the started_at timestamp

<details>
<summary>Source</summary>

```python
    fn started_at(&self) -> String {
        self.started_at.clone()
    }
```

</details>



##### `stop`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">stop</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle::stop](../../../rust/angreal/python_bindings/integrations/flox.md#stop)

Stop the services tracked by this handle

<details>
<summary>Source</summary>

```python
    fn stop(&self) -> PyResult<()> {
        let flox_env = FloxEnvironment::new(&self.flox_env_path);
        flox_env.services_stop().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to stop services: {}",
                e
            ))
        })
    }
```

</details>



##### `save`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">save</span>(path: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle::save](../../../rust/angreal/python_bindings/integrations/flox.md#save)

Save handle to a JSON file

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `path` | `Option < & str >` |  |


<details>
<summary>Source</summary>

```python
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
```

</details>



##### `load`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">load</span>(_cls:  Bound &lt; &#x27;_ , PyType &gt;, path: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; FloxServiceHandle &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle::load](../../../rust/angreal/python_bindings/integrations/flox.md#load)

Load handle from a JSON file

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `_cls` | ` Bound < '_ , PyType >` |  |
| `path` | `Option < & str >` |  |


<details>
<summary>Source</summary>

```python
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
```

</details>



##### `__repr__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__repr__</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxServiceHandle::__repr__](../../../rust/angreal/python_bindings/integrations/flox.md#__repr__)

<details>
<summary>Source</summary>

```python
    fn __repr__(&self) -> String {
        let service_names: Vec<&str> = self.services.iter().map(|s| s.name.as_str()).collect();
        format!(
            "FloxServiceHandle(services={:?}, started_at='{}')",
            service_names, self.started_at
        )
    }
```

</details>





### `class FloxRequiredDecorator`

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredDecorator](../../../rust/angreal/python_bindings/integrations/flox.md#class-floxrequireddecorator)

A Python callable that wraps the flox_required decorator logic

#### Methods

##### `__call__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__call__</span>(func: Py &lt; PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredDecorator::__call__](../../../rust/angreal/python_bindings/integrations/flox.md#__call__)

<details>
<summary>Source</summary>

```python
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
```

</details>





### `class FloxRequiredWrapper`

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredWrapper](../../../rust/angreal/python_bindings/integrations/flox.md#class-floxrequiredwrapper)

The actual wrapper function that handles Flox lifecycle

#### Methods

##### `__call__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__call__</span>(args:  Bound &lt; &#x27;_ , pyo3 :: types :: PyTuple &gt;, kwargs: Option &lt; &amp; Bound &lt; &#x27;_ , pyo3 :: types :: PyDict &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredWrapper::__call__](../../../rust/angreal/python_bindings/integrations/flox.md#__call__)

<details>
<summary>Source</summary>

```python
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
```

</details>



##### `get_arguments`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">get_arguments</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredWrapper::get_arguments](../../../rust/angreal/python_bindings/integrations/flox.md#get_arguments)

<details>
<summary>Source</summary>

```python
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

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__name__</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredWrapper::__name__](../../../rust/angreal/python_bindings/integrations/flox.md#__name__)

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



##### `__doc__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__doc__</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredWrapper::__doc__](../../../rust/angreal/python_bindings/integrations/flox.md#__doc__)

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



##### `__getattr__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__getattr__</span>(name:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::FloxRequiredWrapper::__getattr__](../../../rust/angreal/python_bindings/integrations/flox.md#__getattr__)

<details>
<summary>Source</summary>

```python
    fn __getattr__(&self, name: &str) -> PyResult<Py<PyAny>> {
        Python::attach(|py| self.original_func.getattr(py, name))
    }
```

</details>





## Functions

### `flox_required`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">flox_required</span>(path: Option &lt; Py &lt; PyAny &gt; &gt;, services: Option &lt; Vec &lt; String &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; FloxRequiredDecorator &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::flox::flox_required](../../../rust/angreal/python_bindings/integrations/flox.md#fn-flox_required)

Decorator that wraps a function to run in a Flox environment

This is equivalent to the Python @flox_required decorator

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `path` | `Option < Py < PyAny > >` |  |
| `services` | `Option < Vec < String > >` |  |


<details>
<summary>Source</summary>

```python
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
```

</details>
