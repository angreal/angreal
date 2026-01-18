# git


Git integration bindings

## Structs

### `class GitException`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.integrations.git.GitException](../../../../angreal/python_bindings/integrations/git.md#class-gitexception)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `message` | `String` |  |

#### Methods

##### `new`

```rust
fn new (message : String) -> Self
```

> **Python API**: [angreal.python_bindings.integrations.git.GitException.new](../../../../angreal/python_bindings/integrations/git.md#new)

<details>
<summary>Source</summary>

```rust
    fn new(message: String) -> Self {
        Self { message }
    }
```

</details>



##### `__str__`

```rust
fn __str__ (& self) -> String
```

> **Python API**: [angreal.python_bindings.integrations.git.GitException.__str__](../../../../angreal/python_bindings/integrations/git.md#__str__)

<details>
<summary>Source</summary>

```rust
    fn __str__(&self) -> String {
        self.message.clone()
    }
```

</details>





### `class Git`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.PyGit](../../../../angreal.md#class-pygit)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `inner` | `Git` |  |

#### Methods

##### `new`

```rust
fn new (working_dir : Option < & str >) -> PyResult < Self >
```

> **Python API**: [angreal.PyGit.new](../../../../angreal.md#new)

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



##### `execute`

```rust
fn execute (& self , subcommand : & str , args : Vec < String > ,) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.execute](../../../../angreal.md#execute)

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



##### `init`

```rust
fn init (& self , bare : Option < bool >) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.init](../../../../angreal.md#init)

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



##### `add`

```rust
fn add (& self , paths : & Bound < '_ , pyo3 :: types :: PyTuple > ,) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.add](../../../../angreal.md#add)

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



##### `commit`

```rust
fn commit (& self , message : & str , all : Option < bool >) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.commit](../../../../angreal.md#commit)

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



##### `push`

```rust
fn push (& self , remote : Option < & str > , branch : Option < & str > ,) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.push](../../../../angreal.md#push)

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



##### `pull`

```rust
fn pull (& self , remote : Option < & str > , branch : Option < & str > ,) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.pull](../../../../angreal.md#pull)

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



##### `status`

```rust
fn status (& self , short : Option < bool >) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.status](../../../../angreal.md#status)

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



##### `branch`

```rust
fn branch (& self , name : Option < & str > , delete : Option < bool > ,) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.branch](../../../../angreal.md#branch)

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



##### `checkout`

```rust
fn checkout (& self , branch : & str , create : Option < bool > ,) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.checkout](../../../../angreal.md#checkout)

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



##### `tag`

```rust
fn tag (& self , name : & str , message : Option < & str >) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.tag](../../../../angreal.md#tag)

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



##### `__call__`

```rust
fn __call__ (& self , command : & str , args : & Bound < '_ , pyo3 :: types :: PyTuple > , kwargs : Option < & Bound < '_ , PyDict > > ,) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Python API**: [angreal.PyGit.__call__](../../../../angreal.md#__call__)

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



##### `working_dir`

```rust
fn working_dir (& self) -> String
```

> **Python API**: [angreal.PyGit.working_dir](../../../../angreal.md#working_dir)

<details>
<summary>Source</summary>

```rust
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>



##### `__getattr__`

```rust
fn __getattr__ (& self , _py : Python , name : & str) -> PyResult < Py < PyAny > >
```

> **Python API**: [angreal.PyGit.__getattr__](../../../../angreal.md#__getattr__)

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

### `fn git_clone`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.git_clone](../../../../angreal.md#git_clone)

```rust
fn git_clone (remote : & str , destination : Option < & str >) -> PyResult < String >
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



### `fn git_integration`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn git_integration (_py : Python , m : & Bound < '_ , PyModule >) -> PyResult < () >
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
