# git


Git integration bindings

## Classes

### `class GitException`

> **Rust Implementation**: [angreal::python_bindings::integrations::git::GitException](../../../rust/angreal/python_bindings/integrations/git.md#class-gitexception)

#### Methods

##### `new`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">new</span>(message: str) -> <span style="color: var(--md-default-fg-color--light);">Self</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::GitException::new](../../../rust/angreal/python_bindings/integrations/git.md#new)

<details>
<summary>Source</summary>

```python
    fn new(message: String) -> Self {
        Self { message }
    }
```

</details>



##### `__str__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__str__</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::GitException::__str__](../../../rust/angreal/python_bindings/integrations/git.md#__str__)

<details>
<summary>Source</summary>

```python
    fn __str__(&self) -> String {
        self.message.clone()
    }
```

</details>





### `class Git`

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit](../../../rust/angreal/python_bindings/integrations/git.md#class-git)

#### Methods

##### `new`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">new</span>(working_dir: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Self &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::new](../../../rust/angreal/python_bindings/integrations/git.md#new)

<details>
<summary>Source</summary>

```python
    fn new(working_dir: Option<&str>) -> PyResult<Self> {
        let git = Git::new(working_dir.map(Path::new))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { inner: git })
    }
```

</details>



##### `execute`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">execute</span>(subcommand:  str, args: Vec &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::execute](../../../rust/angreal/python_bindings/integrations/git.md#execute)

<details>
<summary>Source</summary>

```python
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



##### `init`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">init</span>(bare: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::init](../../../rust/angreal/python_bindings/integrations/git.md#init)

<details>
<summary>Source</summary>

```python
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



##### `add`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">add</span>(paths:  Bound &lt; &#x27;_ , pyo3 :: types :: PyTuple &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::add](../../../rust/angreal/python_bindings/integrations/git.md#add)

<details>
<summary>Source</summary>

```python
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



##### `commit`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">commit</span>(message:  str, all: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::commit](../../../rust/angreal/python_bindings/integrations/git.md#commit)

<details>
<summary>Source</summary>

```python
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



##### `push`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">push</span>(remote: Option &lt; &amp; str &gt;, branch: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::push](../../../rust/angreal/python_bindings/integrations/git.md#push)

<details>
<summary>Source</summary>

```python
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



##### `pull`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">pull</span>(remote: Option &lt; &amp; str &gt;, branch: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::pull](../../../rust/angreal/python_bindings/integrations/git.md#pull)

<details>
<summary>Source</summary>

```python
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



##### `status`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">status</span>(short: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::status](../../../rust/angreal/python_bindings/integrations/git.md#status)

<details>
<summary>Source</summary>

```python
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



##### `branch`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">branch</span>(name: Option &lt; &amp; str &gt;, delete: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::branch](../../../rust/angreal/python_bindings/integrations/git.md#branch)

<details>
<summary>Source</summary>

```python
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



##### `checkout`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">checkout</span>(branch:  str, create: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::checkout](../../../rust/angreal/python_bindings/integrations/git.md#checkout)

<details>
<summary>Source</summary>

```python
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



##### `tag`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">tag</span>(name:  str, message: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::tag](../../../rust/angreal/python_bindings/integrations/git.md#tag)

<details>
<summary>Source</summary>

```python
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



##### `__call__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__call__</span>(command:  str, args:  Bound &lt; &#x27;_ , pyo3 :: types :: PyTuple &gt;, kwargs: Option &lt; &amp; Bound &lt; &#x27;_ , PyDict &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , Py &lt; PyAny &gt; , Py &lt; PyAny &gt;) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::__call__](../../../rust/angreal/python_bindings/integrations/git.md#__call__)

<details>
<summary>Source</summary>

```python
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



##### `working_dir`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">working_dir</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::working_dir](../../../rust/angreal/python_bindings/integrations/git.md#working_dir)

<details>
<summary>Source</summary>

```python
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>



##### `__getattr__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__getattr__</span>(_py: Python, name:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::__getattr__](../../../rust/angreal/python_bindings/integrations/git.md#__getattr__)

<details>
<summary>Source</summary>

```python
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

### `git_clone`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">git_clone</span>(remote:  str, destination: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::git_clone](../../../rust/angreal.md#fn-git_clone)

<details>
<summary>Source</summary>

```python
pub fn git_clone(remote: &str, destination: Option<&str>) -> PyResult<String> {
    let dest = Git::clone(remote, destination.map(Path::new))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(dest.display().to_string())
}
```

</details>
