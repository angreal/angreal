---
title: "angreal.python_bindings.integrations.git"
description: "Git integration bindings"
sidebar:
  order: 7
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


Git integration bindings

## Classes

### <span id="class.GitException"></span>`class GitException`

> **Rust Implementation**: [angreal::python_bindings::integrations::git::GitException](../../../../rust/angreal/python_bindings/integrations/git/#struct.GitException)

#### Methods

##### <span id="GitException.new"></span>`new`

```python
def new(message: str) -> Self
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::GitException::new](../../../../rust/angreal/python_bindings/integrations/git/#GitException.new)

<details>
<summary>Source</summary>

```python
    fn new(message: String) -> Self {
        Self { message }
    }
```

</details>



##### <span id="GitException.__str__"></span>`__str__`

```python
def __str__() -> str
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::GitException::__str__](../../../../rust/angreal/python_bindings/integrations/git/#GitException.__str__)

<details>
<summary>Source</summary>

```python
    fn __str__(&self) -> String {
        self.message.clone()
    }
```

</details>





### <span id="class.Git"></span>`class Git`

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit](../../../../rust/angreal/python_bindings/integrations/git/#struct.PyGit)

#### Methods

##### <span id="Git.new"></span>`new`

```python
def new(working_dir: Option < & str >) -> PyResult < Self >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::new](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.new)

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



##### <span id="Git.execute"></span>`execute`

```python
def execute(
    subcommand:  str,
    args: Vec < String >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::execute](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.execute)

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



##### <span id="Git.init"></span>`init`

```python
def init(bare: Option < bool >) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::init](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.init)

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



##### <span id="Git.add"></span>`add`

```python
def add(
    paths:  Bound < '_ , pyo3 :: types :: PyTuple >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::add](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.add)

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



##### <span id="Git.commit"></span>`commit`

```python
def commit(
    message:  str,
    all: Option < bool >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::commit](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.commit)

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



##### <span id="Git.push"></span>`push`

```python
def push(
    remote: Option < & str >,
    branch: Option < & str >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::push](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.push)

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



##### <span id="Git.pull"></span>`pull`

```python
def pull(
    remote: Option < & str >,
    branch: Option < & str >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::pull](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.pull)

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



##### <span id="Git.status"></span>`status`

```python
def status(short: Option < bool >) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::status](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.status)

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



##### <span id="Git.branch"></span>`branch`

```python
def branch(
    name: Option < & str >,
    delete: Option < bool >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::branch](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.branch)

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



##### <span id="Git.checkout"></span>`checkout`

```python
def checkout(
    branch:  str,
    create: Option < bool >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::checkout](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.checkout)

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



##### <span id="Git.tag"></span>`tag`

```python
def tag(
    name:  str,
    message: Option < & str >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::tag](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.tag)

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



##### <span id="Git.__call__"></span>`__call__`

```python
def __call__(
    command:  str,
    args:  Bound < '_ , pyo3 :: types :: PyTuple >,
    kwargs: Option < & Bound < '_ , PyDict > >,
) -> PyResult < (i32 , Py < PyAny > , Py < PyAny >) >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::__call__](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.__call__)

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



##### <span id="Git.working_dir"></span>`working_dir`

```python
def working_dir() -> str
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::working_dir](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.working_dir)

<details>
<summary>Source</summary>

```python
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>



##### <span id="Git.__getattr__"></span>`__getattr__`

```python
def __getattr__(_py: Python, name:  str) -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::git::PyGit::__getattr__](../../../../rust/angreal/python_bindings/integrations/git/#PyGit.__getattr__)

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

### <span id="fn.git_clone"></span>`git_clone`

```python
def git_clone(remote:  str, destination: Option < & str >) -> PyResult < String >
```

> **Rust Implementation**: [angreal::git_clone](../../../../rust/angreal/#fn.git_clone)

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
