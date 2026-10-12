---
title: "angreal::python_bindings::integrations::git"
description: "Git integration bindings"
editUrl: false
sidebar:
  order: 34
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Git integration bindings

## Structs

### <span id="struct.GitException"></span><code style="font-size: 0.85em">class GitException</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

> **Python API**: [`angreal.integrations.git_integration.GitException`](../../../../../angreal/integrations/git_integration/#class.GitException)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `message` | `String` |  |

#### Methods

##### <span id="GitException.new"></span><code>new</code>

```rust
fn new(message: String) -> Self
```

> **Python API**: [`angreal.integrations.git_integration.GitException.new`](../../../../../angreal/integrations/git_integration/#GitException.new)

<details>
<summary>Source</summary>

```rust
    fn new(message: String) -> Self {
        Self { message }
    }
```

</details>

##### <span id="GitException.__str__"></span><code>_​_​str_​_​</code>

```rust
fn __str__(&self) -> String
```

> **Python API**: [`angreal.integrations.git_integration.GitException.__str__`](../../../../../angreal/integrations/git_integration/#GitException.__str__)

<details>
<summary>Source</summary>

```rust
    fn __str__(&self) -> String {
        self.message.clone()
    }
```

</details>

### <span id="struct.PyGit"></span><code style="font-size: 0.85em">class Git</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `inner` | `Git` |  |

#### Methods

##### <span id="PyGit.new"></span><code>new</code>

```rust
fn new(working_dir: Option<&str>) -> PyResult<Self>
```

<details>
<summary>Source</summary>

```rust
    fn new(working_dir: Option<&str>) -> PyResult<Self> {
        let git = Git::new(working_dir.map(Path::new))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { inner: git })
    }
```

</details>

##### <span id="PyGit.execute"></span><code>execute</code>

```rust
fn execute(
    &self,
    subcommand: &str,
    args: Vec<String>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn execute(
        &self,
        subcommand: &str,
        args: Vec<String>,
    ) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            let output = self
                .inner
                .execute(subcommand, &arg_refs)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.init"></span><code>init</code>

```rust
fn init(
    &self) -> PyResult<(i32, Py<PyAny>, Py<PyAny>,
    bare: Option<bool>) -> PyResult<(i32, Py<PyAny>, Py<PyAny>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn init(&self, bare: Option<bool>) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let output = if bare.unwrap_or(false) {
                self.inner.execute("init", &["--bare"])
            } else {
                self.inner.execute("init", &[])
            }
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.add"></span><code>add</code>

```rust
fn add(
    &self,
    paths: &Bound<'_, pyo3::types::PyTuple>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn add(
        &self,
        paths: &Bound<'_, pyo3::types::PyTuple>,
    ) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let path_strs: Vec<String> = paths
                .iter()
                .map(|p| p.extract::<String>())
                .collect::<Result<Vec<_>, _>>()?;
            let path_refs: Vec<&str> = path_strs.iter().map(|s| s.as_str()).collect();
            let output = self
                .inner
                .execute("add", &path_refs)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.commit"></span><code>commit</code>

```rust
fn commit(
    &self,
    message: &str,
    all: Option<bool>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn commit(&self, message: &str, all: Option<bool>) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let args = if all.unwrap_or(false) {
                vec!["-m", message, "-a"]
            } else {
                vec!["-m", message]
            };
            let output = self
                .inner
                .execute("commit", &args)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.push"></span><code>push</code>

```rust
fn push(
    &self,
    remote: Option<&str>,
    branch: Option<&str>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn push(
        &self,
        remote: Option<&str>,
        branch: Option<&str>,
    ) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let mut args = vec![];
            if let Some(r) = remote {
                args.push(r);
            }
            if let Some(b) = branch {
                args.push(b);
            }
            let output = self
                .inner
                .execute("push", &args)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.pull"></span><code>pull</code>

```rust
fn pull(
    &self,
    remote: Option<&str>,
    branch: Option<&str>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn pull(
        &self,
        remote: Option<&str>,
        branch: Option<&str>,
    ) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let mut args = vec![];
            if let Some(r) = remote {
                args.push(r);
            }
            if let Some(b) = branch {
                args.push(b);
            }
            let output = self
                .inner
                .execute("pull", &args)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.status"></span><code>status</code>

```rust
fn status(
    &self) -> PyResult<(i32, Py<PyAny>, Py<PyAny>,
    short: Option<bool>) -> PyResult<(i32, Py<PyAny>, Py<PyAny>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn status(&self, short: Option<bool>) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let args = if short.unwrap_or(false) {
                vec!["--short"]
            } else {
                vec![]
            };
            let output = self
                .inner
                .execute("status", &args)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.branch"></span><code>branch</code>

```rust
fn branch(
    &self,
    name: Option<&str>,
    delete: Option<bool>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn branch(
        &self,
        name: Option<&str>,
        delete: Option<bool>,
    ) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let mut args = vec![];
            if delete.unwrap_or(false) {
                args.push("-d");
            }
            if let Some(n) = name {
                args.push(n);
            }
            let output = self
                .inner
                .execute("branch", &args)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.checkout"></span><code>checkout</code>

```rust
fn checkout(
    &self,
    branch: &str,
    create: Option<bool>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn checkout(
        &self,
        branch: &str,
        create: Option<bool>,
    ) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let args = if create.unwrap_or(false) {
                vec!["-b", branch]
            } else {
                vec![branch]
            };
            let output = self
                .inner
                .execute("checkout", &args)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.tag"></span><code>tag</code>

```rust
fn tag(
    &self,
    name: &str,
    message: Option<&str>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn tag(&self, name: &str, message: Option<&str>) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            let args = if let Some(msg) = message {
                vec!["-m", msg, name]
            } else {
                vec![name]
            };
            let output = self
                .inner
                .execute("tag", &args)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.__call__"></span><code>_​_​call_​_​</code>

```rust
fn __call__(
    &self,
    command: &str,
    args: &Bound<'_, pyo3::types::PyTuple>,
    kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)>
```

<details>
<summary>Source</summary>

```rust
    fn __call__(
        &self,
        command: &str,
        args: &Bound<'_, pyo3::types::PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<(i32, Py<PyAny>, Py<PyAny>)> {
        Python::attach(|py| {
            // Convert args to Vec<String>
            let arg_strs: Vec<String> = args
                .iter()
                .map(|p| p.extract::<String>())
                .collect::<Result<Vec<_>, _>>()?;
            let arg_refs: Vec<&str> = arg_strs.iter().map(|s| s.as_str()).collect();

            let output = if let Some(dict) = kwargs {
                // Convert PyDict to HashMap<String, String>
                let mut options_owned = HashMap::new();
                for (key, value) in dict.iter() {
                    let key_str = key.extract::<String>()?;
                    let value_str = if value.is_truthy()? {
                        "".to_string() // For boolean flags like --bare
                    } else {
                        value.extract::<String>()?
                    };
                    options_owned.insert(key_str, value_str);
                }
                // Convert to HashMap<&str, &str> for execute_with_options
                let options: HashMap<&str, &str> = options_owned
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                self.inner.execute_with_options(command, options, &arg_refs)
            } else {
                self.inner.execute(command, &arg_refs)
            }
            .map_err(|e| {
                // Check if this is an unsupported command error and convert to GitException
                if e.to_string().contains("not supported") {
                    PyErr::new::<GitException, _>(e.to_string())
                } else {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
                }
            })?;

            Ok((
                output.exit_code,
                pyo3::types::PyBytes::new(py, output.stderr.as_bytes()).into(),
                pyo3::types::PyBytes::new(py, output.stdout.as_bytes()).into(),
            ))
        })
    }
```

</details>

##### <span id="PyGit.working_dir"></span><code>working_​dir</code>

```rust
fn working_dir(&self) -> String
```

<details>
<summary>Source</summary>

```rust
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>

##### <span id="PyGit.__getattr__"></span><code>_​_​getattr_​_​</code>

```rust
fn __getattr__(&self, _py: Python, name: &str) -> PyResult<Py<PyAny>>
```

<details>
<summary>Source</summary>

```rust
    fn __getattr__(&self, _py: Python, name: &str) -> PyResult<Py<PyAny>> {
        // For any unknown method, raise GitException
        Err(PyErr::new::<GitException, _>(format!(
            "Git command '{}' not found",
            name
        )))
    }
```

</details>

## Functions

### <span id="fn.git_clone"></span><code style="font-size: 0.85em">fn git_​clone</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

```rust
fn git_clone(
    remote: &str,
    destination: Option<&str>,
) -> PyResult<String>
```

<details>
<summary>Source</summary>

```rust
pub fn git_clone(remote: &str, destination: Option<&str>) -> PyResult<String> {
    let dest = Git::clone(remote, destination.map(Path::new))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(dest.display().to_string())
}
```

</details>

### <span id="fn.git_integration"></span><code style="font-size: 0.85em">fn git_​integration</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn git_integration(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()>
```

Git integration module

This will be exposed as angreal.integrations.git in Python

<details>
<summary>Source</summary>

```rust
pub fn git_integration(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GitException>()?;
    // Export PyGit as "Git" to match the expected interface
    m.add("Git", _py.get_type::<PyGit>())?;
    // Export git_clone as "clone" to match the expected interface
    m.add("clone", wrap_pyfunction!(git_clone, m)?)?;
    Ok(())
}
```

</details>
