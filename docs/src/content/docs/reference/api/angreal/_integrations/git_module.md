---
title: "angreal._integrations.git_module"
description: "API reference of the Python module angreal._integrations.git_module."
editUrl: false
sidebar:
  order: 2
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

## Classes

### <span id="class.PyGit"></span><code style="font-size: 0.85em">class PyGit</code>

> **Rust Implementation**: [`angreal::PyGit`](../../../rust/angreal/#struct.PyGit)

#### Methods

##### <span id="PyGit.new"></span><code>new</code>

```python
def new(working_dir: Optional[str]) -> Self
```

> **Rust Implementation**: [`angreal::PyGit::new`](../../../rust/angreal/#PyGit.new)

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

```python
def execute(subcommand: str, args: List[str]) -> (i32, String, String)
```

> **Rust Implementation**: [`angreal::PyGit::execute`](../../../rust/angreal/#PyGit.execute)

<details>
<summary>Source</summary>

```rust
    fn execute(&self, subcommand: &str, args: Vec<String>) -> PyResult<(i32, String, String)> {
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let output = self
            .inner
            .execute(subcommand, &arg_refs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok((output.exit_code, output.stderr, output.stdout))
    }
```

</details>

##### <span id="PyGit.init"></span><code>init</code>

```python
def init(bare: Optional[bool]) -> None
```

> **Rust Implementation**: [`angreal::PyGit::init`](../../../rust/angreal/#PyGit.init)

<details>
<summary>Source</summary>

```rust
    fn init(&self, bare: Option<bool>) -> PyResult<()> {
        self.inner
            .init(bare.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.add"></span><code>add</code>

```python
def add(paths: List[str]) -> None
```

> **Rust Implementation**: [`angreal::PyGit::add`](../../../rust/angreal/#PyGit.add)

<details>
<summary>Source</summary>

```rust
    fn add(&self, paths: Vec<String>) -> PyResult<()> {
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        self.inner
            .add(&path_refs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.commit"></span><code>commit</code>

```python
def commit(message: str, all: Optional[bool]) -> None
```

> **Rust Implementation**: [`angreal::PyGit::commit`](../../../rust/angreal/#PyGit.commit)

<details>
<summary>Source</summary>

```rust
    fn commit(&self, message: &str, all: Option<bool>) -> PyResult<()> {
        self.inner
            .commit(message, all.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.push"></span><code>push</code>

```python
def push(remote: Optional[str], branch: Optional[str]) -> None
```

> **Rust Implementation**: [`angreal::PyGit::push`](../../../rust/angreal/#PyGit.push)

<details>
<summary>Source</summary>

```rust
    fn push(&self, remote: Option<&str>, branch: Option<&str>) -> PyResult<()> {
        self.inner
            .push(remote, branch)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.pull"></span><code>pull</code>

```python
def pull(remote: Optional[str], branch: Optional[str]) -> None
```

> **Rust Implementation**: [`angreal::PyGit::pull`](../../../rust/angreal/#PyGit.pull)

<details>
<summary>Source</summary>

```rust
    fn pull(&self, remote: Option<&str>, branch: Option<&str>) -> PyResult<()> {
        self.inner
            .pull(remote, branch)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.status"></span><code>status</code>

```python
def status(short: Optional[bool]) -> str
```

> **Rust Implementation**: [`angreal::PyGit::status`](../../../rust/angreal/#PyGit.status)

<details>
<summary>Source</summary>

```rust
    fn status(&self, short: Option<bool>) -> PyResult<String> {
        self.inner
            .status(short.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.branch"></span><code>branch</code>

```python
def branch(name: Optional[str], delete: Optional[bool]) -> str
```

> **Rust Implementation**: [`angreal::PyGit::branch`](../../../rust/angreal/#PyGit.branch)

<details>
<summary>Source</summary>

```rust
    fn branch(&self, name: Option<&str>, delete: Option<bool>) -> PyResult<String> {
        self.inner
            .branch(name, delete.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.checkout"></span><code>checkout</code>

```python
def checkout(branch: str, create: Optional[bool]) -> None
```

> **Rust Implementation**: [`angreal::PyGit::checkout`](../../../rust/angreal/#PyGit.checkout)

<details>
<summary>Source</summary>

```rust
    fn checkout(&self, branch: &str, create: Option<bool>) -> PyResult<()> {
        self.inner
            .checkout(branch, create.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.tag"></span><code>tag</code>

```python
def tag(name: str, message: Optional[str]) -> None
```

> **Rust Implementation**: [`angreal::PyGit::tag`](../../../rust/angreal/#PyGit.tag)

<details>
<summary>Source</summary>

```rust
    fn tag(&self, name: &str, message: Option<&str>) -> PyResult<()> {
        self.inner
            .tag(name, message)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>

##### <span id="PyGit.__call__"></span><code>_​_​call_​_​</code>

```python
def __call__(
    command: str,
    args: List[str],
    kwargs: Optional[Bound<'_, pyo3::types::PyDict>],
) -> (i32, String, String)
```

> **Rust Implementation**: [`angreal::PyGit::__call__`](../../../rust/angreal/#PyGit.__call__)

<details>
<summary>Source</summary>

```rust
    fn __call__(
        &self,
        command: &str,
        args: Vec<String>,
        kwargs: Option<&Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<(i32, String, String)> {
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let output = if let Some(dict) = kwargs {
            // Convert PyDict to HashMap<String, String> then to HashMap<&str, &str>
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
            let options: HashMap<&str, &str> = options_owned
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            self.inner.execute_with_options(command, options, &arg_refs)
        } else {
            self.inner.execute(command, &arg_refs)
        }
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok((output.exit_code, output.stderr, output.stdout))
    }
```

</details>

## Functions

### <span id="fn.git_clone"></span><code style="font-size: 0.85em">git_​clone</code>

```python
def git_clone(remote: str, destination: Optional[str]) -> str
```

> **Rust Implementation**: [`angreal::git_clone`](../../../rust/angreal/#fn.git_clone)

<details>
<summary>Source</summary>

```rust
fn git_clone(remote: &str, destination: Option<&str>) -> PyResult<String> {
    let dest = Git::clone(remote, destination.map(Path::new))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(dest.display().to_string())
}
```

</details>
