# angreal


Angreal - project templating and task management

A package for templating code based projects and providing methods
for the creation and management of common operational tasks associated with the
project.

## Classes

### `class PyGit`

> **Rust Implementation**: [angreal::PyGit](rust/angreal.md#class-pygit)

#### Methods

##### `new`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">new</span>(working_dir: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Self &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::new](rust/angreal.md#new)

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
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">execute</span>(subcommand:  str, args: Vec &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , String , String) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::execute](rust/angreal.md#execute)

<details>
<summary>Source</summary>

```python
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



##### `init`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">init</span>(bare: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::init](rust/angreal.md#init)

<details>
<summary>Source</summary>

```python
    fn init(&self, bare: Option<bool>) -> PyResult<()> {
        self.inner
            .init(bare.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `add`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">add</span>(paths: Vec &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::add](rust/angreal.md#add)

<details>
<summary>Source</summary>

```python
    fn add(&self, paths: Vec<String>) -> PyResult<()> {
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        self.inner
            .add(&path_refs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `commit`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">commit</span>(message:  str, all: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::commit](rust/angreal.md#commit)

<details>
<summary>Source</summary>

```python
    fn commit(&self, message: &str, all: Option<bool>) -> PyResult<()> {
        self.inner
            .commit(message, all.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `push`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">push</span>(remote: Option &lt; &amp; str &gt;, branch: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::push](rust/angreal.md#push)

<details>
<summary>Source</summary>

```python
    fn push(&self, remote: Option<&str>, branch: Option<&str>) -> PyResult<()> {
        self.inner
            .push(remote, branch)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `pull`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">pull</span>(remote: Option &lt; &amp; str &gt;, branch: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::pull](rust/angreal.md#pull)

<details>
<summary>Source</summary>

```python
    fn pull(&self, remote: Option<&str>, branch: Option<&str>) -> PyResult<()> {
        self.inner
            .pull(remote, branch)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `status`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">status</span>(short: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::status](rust/angreal.md#status)

<details>
<summary>Source</summary>

```python
    fn status(&self, short: Option<bool>) -> PyResult<String> {
        self.inner
            .status(short.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `branch`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">branch</span>(name: Option &lt; &amp; str &gt;, delete: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::branch](rust/angreal.md#branch)

<details>
<summary>Source</summary>

```python
    fn branch(&self, name: Option<&str>, delete: Option<bool>) -> PyResult<String> {
        self.inner
            .branch(name, delete.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `checkout`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">checkout</span>(branch:  str, create: Option &lt; bool &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::checkout](rust/angreal.md#checkout)

<details>
<summary>Source</summary>

```python
    fn checkout(&self, branch: &str, create: Option<bool>) -> PyResult<()> {
        self.inner
            .checkout(branch, create.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `tag`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">tag</span>(name:  str, message: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::tag](rust/angreal.md#tag)

<details>
<summary>Source</summary>

```python
    fn tag(&self, name: &str, message: Option<&str>) -> PyResult<()> {
        self.inner
            .tag(name, message)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }
```

</details>



##### `__call__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__call__</span>(command:  str, args: Vec &lt; String &gt;, kwargs: Option &lt; &amp; Bound &lt; &#x27;_ , pyo3 :: types :: PyDict &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; (i32 , String , String) &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::PyGit::__call__](rust/angreal.md#__call__)

<details>
<summary>Source</summary>

```python
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

### `git_clone`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">git_clone</span>(remote:  str, destination: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::git_clone](rust/angreal.md#fn-git_clone)

<details>
<summary>Source</summary>

```python
fn git_clone(remote: &str, destination: Option<&str>) -> PyResult<String> {
    let dest = Git::clone(remote, destination.map(Path::new))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(dest.display().to_string())
}
```

</details>



### `ensure_uv_installed`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">ensure_uv_installed</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::ensure_uv_installed](rust/angreal.md#fn-ensure_uv_installed)

<details>
<summary>Source</summary>

```python
fn ensure_uv_installed() -> PyResult<()> {
    UvIntegration::ensure_installed()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### `uv_version`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">uv_version</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::uv_version](rust/angreal.md#fn-uv_version)

<details>
<summary>Source</summary>

```python
fn uv_version() -> PyResult<String> {
    UvIntegration::version()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### `create_virtualenv`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">create_virtualenv</span>(path:  str, python_version: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::create_virtualenv](rust/angreal.md#fn-create_virtualenv)

<details>
<summary>Source</summary>

```python
fn create_virtualenv(path: &str, python_version: Option<&str>) -> PyResult<()> {
    UvVirtualEnv::create(Path::new(path), python_version)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(())
}
```

</details>



### `install_packages`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">install_packages</span>(venv_path:  str, packages: Vec &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::install_packages](rust/angreal.md#fn-install_packages)

<details>
<summary>Source</summary>

```python
fn install_packages(venv_path: &str, packages: Vec<String>) -> PyResult<()> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.install_packages(&packages)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### `install_requirements`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">install_requirements</span>(venv_path:  str, requirements_file:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::install_requirements](rust/angreal.md#fn-install_requirements)

<details>
<summary>Source</summary>

```python
fn install_requirements(venv_path: &str, requirements_file: &str) -> PyResult<()> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.install_requirements(Path::new(requirements_file))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### `discover_pythons`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">discover_pythons</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Vec &lt; (String , String) &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::discover_pythons](rust/angreal.md#fn-discover_pythons)

<details>
<summary>Source</summary>

```python
fn discover_pythons() -> PyResult<Vec<(String, String)>> {
    UvVirtualEnv::discover_pythons()
        .map(|pythons| {
            pythons
                .into_iter()
                .map(|(version, path)| (version, path.display().to_string()))
                .collect()
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### `install_python`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">install_python</span>(version:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::install_python](rust/angreal.md#fn-install_python)

<details>
<summary>Source</summary>

```python
fn install_python(version: &str) -> PyResult<String> {
    UvVirtualEnv::install_python(version)
        .map(|path| path.display().to_string())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### `get_venv_activation_info`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">get_venv_activation_info</span>(venv_path:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; integrations :: uv :: ActivationInfo &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::get_venv_activation_info](rust/angreal.md#fn-get_venv_activation_info)

<details>
<summary>Source</summary>

```python
fn get_venv_activation_info(venv_path: &str) -> PyResult<integrations::uv::ActivationInfo> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.get_activation_info()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### `register_entrypoint`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">register_entrypoint</span>(name:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::register_entrypoint](rust/angreal.md#fn-register_entrypoint)

<details>
<summary>Source</summary>

```python
fn register_entrypoint(name: &str) -> PyResult<()> {
    use home::home_dir;
    use serde_json;

    // Get home directory, with fallback to environment variables for testing
    let home = if let Some(home_from_env) = std::env::var_os("HOME") {
        PathBuf::from(home_from_env)
    } else if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        PathBuf::from(userprofile)
    } else {
        home_dir().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cannot find home directory")
        })?
    };

    // Create directories
    let local_bin = home.join(".local").join("bin");
    fs::create_dir_all(&local_bin).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to create bin directory: {}",
            e
        ))
    })?;

    let data_dir = home.join(".angrealrc");
    fs::create_dir_all(&data_dir).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to create data directory: {}",
            e
        ))
    })?;

    // Determine script path based on platform
    #[cfg(unix)]
    let script_path = local_bin.join(name);
    #[cfg(windows)]
    let script_path = local_bin.join(format!("{}.bat", name));

    // Check for conflicts
    if script_path.exists() {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Command '{}' already exists at {}",
            name,
            script_path.display()
        )));
    }

    // Create wrapper script
    #[cfg(unix)]
    {
        let script_content = format!(
            "#!/usr/bin/env python\n# ANGREAL_ALIAS: {}\n# Auto-generated by angreal.register_entrypoint\nimport sys\ntry:\n    import angreal\n    angreal.main()\nexcept ImportError:\n    print(f\"Error: angreal not installed. Remove alias: rm {}\", file=sys.stderr)\n    sys.exit(1)\n",
            name,
            script_path.display()
        );

        fs::write(&script_path, script_content).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write script: {}",
                e
            ))
        })?;

        // Make executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to get permissions: {}",
                    e
                ))
            })?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to set permissions: {}",
                e
            ))
        })?;
    }

    #[cfg(windows)]
    {
        let script_content = format!(
            "@echo off\nREM ANGREAL_ALIAS: {}\nREM Auto-generated by angreal.register_entrypoint\npython -m angreal %*\n",
            name
        );
        fs::write(&script_path, script_content).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write script: {}",
                e
            ))
        })?;
    }

    // Update registry
    let registry_path = home.join(".angrealrc").join("aliases.json");
    let mut aliases: Vec<String> = if registry_path.exists() {
        let content = fs::read_to_string(&registry_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to read registry: {}",
                e
            ))
        })?;
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    if !aliases.contains(&name.to_string()) {
        aliases.push(name.to_string());
        let json = serde_json::to_string_pretty(&aliases).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to serialize registry: {}",
                e
            ))
        })?;
        fs::write(&registry_path, json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write registry: {}",
                e
            ))
        })?;
    }

    println!("✅ Registered '{}' as angreal alias", name);
    println!("Make sure ~/.local/bin is in your PATH");
    Ok(())
}
```

</details>



### `list_entrypoints`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">list_entrypoints</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Vec &lt; String &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::list_entrypoints](rust/angreal.md#fn-list_entrypoints)

<details>
<summary>Source</summary>

```python
fn list_entrypoints() -> PyResult<Vec<String>> {
    use home::home_dir;

    // Get home directory, with fallback to environment variables for testing
    let home = if let Some(home_from_env) = std::env::var_os("HOME") {
        PathBuf::from(home_from_env)
    } else if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        PathBuf::from(userprofile)
    } else {
        home_dir().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cannot find home directory")
        })?
    };

    let registry_path = home.join(".angrealrc").join("aliases.json");

    if !registry_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&registry_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read registry: {}", e))
    })?;

    let aliases: Vec<String> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
    Ok(aliases)
}
```

</details>



### `unregister_entrypoint`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">unregister_entrypoint</span>(name:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::unregister_entrypoint](rust/angreal.md#fn-unregister_entrypoint)

<details>
<summary>Source</summary>

```python
fn unregister_entrypoint(name: &str) -> PyResult<()> {
    use home::home_dir;

    // Get home directory, with fallback to environment variables for testing
    let home = if let Some(home_from_env) = std::env::var_os("HOME") {
        PathBuf::from(home_from_env)
    } else if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        PathBuf::from(userprofile)
    } else {
        home_dir().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cannot find home directory")
        })?
    };

    // Remove script
    let local_bin = home.join(".local").join("bin");
    #[cfg(unix)]
    let script_path = local_bin.join(name);
    #[cfg(windows)]
    let script_path = local_bin.join(format!("{}.bat", name));

    if script_path.exists() {
        fs::remove_file(&script_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to remove script: {}",
                e
            ))
        })?;
    }

    // Update registry
    let registry_path = home.join(".angrealrc").join("aliases.json");

    if registry_path.exists() {
        let content = fs::read_to_string(&registry_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to read registry: {}",
                e
            ))
        })?;

        let mut aliases: Vec<String> =
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
        aliases.retain(|alias| alias != name);

        let json = serde_json::to_string_pretty(&aliases).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to serialize registry: {}",
                e
            ))
        })?;
        fs::write(&registry_path, json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write registry: {}",
                e
            ))
        })?;
    }

    println!("✅ Unregistered '{}' alias", name);
    Ok(())
}
```

</details>



### `cleanup_entrypoints`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">cleanup_entrypoints</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::cleanup_entrypoints](rust/angreal.md#fn-cleanup_entrypoints)

<details>
<summary>Source</summary>

```python
fn cleanup_entrypoints() -> PyResult<()> {
    let aliases = list_entrypoints()?;

    for alias in aliases {
        if let Err(e) = unregister_entrypoint(&alias) {
            eprintln!("Warning: Failed to unregister '{}': {}", alias, e);
        }
    }

    println!("✅ Cleaned up all angreal aliases");
    Ok(())
}
```

</details>



### `main`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">main</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::main](rust/angreal.md#fn-main)

The main function is just an entry point to be called from the core angreal library.

<details>
<summary>Source</summary>

```python
fn main() -> PyResult<()> {
    let handle = logger::init_logger();
    if std::env::var("ANGREAL_DEBUG").unwrap_or_default() == "true" {
        logger::update_verbosity(&handle, 2);
        warn!("Angreal application starting with debug level logging from environment");
    }
    debug!("Angreal application starting...");

    // because we execute this from python main, we remove the first elements that
    // IIRC its python and angreal
    let mut argvs: Vec<String> = std::env::args().collect();
    argvs = argvs.split_off(2);

    // Auto-install shell completion on first run (before other operations)
    if let Err(e) = completion::auto_install_completion() {
        warn!("Failed to auto-install shell completion: {}", e);
    }

    debug!("Checking if binary is up to date...");
    match utils::check_up_to_date() {
        Ok(()) => (),
        Err(e) => warn!(
            "An error occurred while checking if our binary is up to date. {}",
            e
        ),
    };

    // Load any angreal task assets that are available to us
    let angreal_project_result = utils::is_angreal_project();
    let in_angreal_project = angreal_project_result.is_ok();

    if in_angreal_project {
        debug!("Angreal project detected, loading found tasks.");
        let angreal_path = angreal_project_result.expect("Expected angreal project path");
        // get a list of files
        let angreal_tasks_to_load = utils::get_task_files(angreal_path);

        // Explicitly capture error with exit
        let _angreal_tasks_to_load = match angreal_tasks_to_load {
            Ok(tasks) => tasks,
            Err(_) => {
                error!("Exiting due to unrecoverable error.");
                exit(1);
            }
        };

        // load the files , IF a file has command or task decorators - they'll register themselves now
        for task in _angreal_tasks_to_load.iter() {
            if let Err(e) = utils::load_python(task.clone()) {
                error!("Failed to load Python task: {}", e);
            }
        }
    }

    let app = build_app(in_angreal_project);
    let mut app_copy = app.clone();
    let sub_command = app.get_matches_from(&argvs);

    // Get our asked for verbosity and set the logger up. TODO: find a way to initialize earlier and reset after.
    let verbosity = sub_command.get_count("verbose");

    // If the user hasn't set the ANGREAL_DEBUG environment variable, set the verbosity from CLI settings
    if std::env::var("ANGREAL_DEBUG").is_err() {
        logger::update_verbosity(&handle, verbosity);
        debug!("Log verbosity set to level: {}", verbosity);
    }

    match sub_command.subcommand() {
        Some(("init", _sub_matches)) => init::init(
            _sub_matches.value_of("template").unwrap(),
            _sub_matches.is_present("force"),
            _sub_matches.is_present("defaults").not(),
            if _sub_matches.is_present("values_file") {
                Some(_sub_matches.value_of("values_file").unwrap())
            } else {
                None
            },
        ),
        Some(("_complete", _sub_matches)) => {
            // Hidden command for shell completion
            let args: Vec<String> = _sub_matches
                .values_of("args")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();

            match completion::generate_completions(&args) {
                Ok(completions) => {
                    for completion in completions {
                        println!("{}", completion);
                    }
                }
                Err(e) => {
                    debug!("Completion generation failed: {}", e);
                }
            }
            return Ok(());
        }
        Some(("_completion", _sub_matches)) => {
            // Hidden command for completion script generation
            let shell = _sub_matches.value_of("shell").unwrap_or("bash");
            match shell {
                "bash" => println!("{}", completion::bash::generate_completion_script()),
                "zsh" => println!("{}", completion::zsh::generate_completion_script()),
                _ => {
                    error!("Unsupported shell for completion: {}", shell);
                    exit(1);
                }
            }
            return Ok(());
        }
        Some(("alias", sub_matches)) => {
            // Handle alias subcommands
            match sub_matches.subcommand() {
                Some(("create", create_matches)) => {
                    let name = create_matches.value_of("name").unwrap();
                    Python::attach(|_py| {
                        if let Err(e) = register_entrypoint(name) {
                            error!("Failed to create alias: {}", e);
                            exit(1);
                        }
                    });
                }
                Some(("remove", remove_matches)) => {
                    let name = remove_matches.value_of("name").unwrap();
                    Python::attach(|_py| {
                        if let Err(e) = unregister_entrypoint(name) {
                            error!("Failed to remove alias: {}", e);
                            exit(1);
                        }
                    });
                }
                Some(("list", _)) => {
                    Python::attach(|_py| match list_entrypoints() {
                        Ok(aliases) => {
                            if aliases.is_empty() {
                                println!("No aliases registered.");
                            } else {
                                println!("Registered aliases:");
                                for alias in aliases {
                                    println!("  {}", alias);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to list aliases: {}", e);
                            exit(1);
                        }
                    });
                }
                _ => {
                    error!("Invalid alias subcommand. Use 'create', 'remove', or 'list'.");
                    exit(1);
                }
            }
            return Ok(());
        }
        Some(("completion", sub_matches)) => {
            // Handle completion management subcommands
            match sub_matches.subcommand() {
                Some(("install", install_matches)) => {
                    let shell = install_matches.value_of("shell");
                    match crate::completion::force_install_completion(shell) {
                        Ok(()) => {}
                        Err(e) => {
                            error!("Failed to install completion: {}", e);
                            exit(1);
                        }
                    }
                }
                Some(("uninstall", uninstall_matches)) => {
                    let shell = uninstall_matches.value_of("shell");
                    match crate::completion::uninstall_completion(shell) {
                        Ok(()) => {}
                        Err(e) => {
                            error!("Failed to uninstall completion: {}", e);
                            exit(1);
                        }
                    }
                }
                Some(("status", _)) => match crate::completion::show_completion_status() {
                    Ok(()) => {}
                    Err(e) => {
                        error!("Failed to show completion status: {}", e);
                        exit(1);
                    }
                },
                _ => {
                    error!(
                        "Invalid completion subcommand. Use 'install', 'uninstall', or 'status'."
                    );
                    exit(1);
                }
            }
            return Ok(());
        }
        Some(("tree", sub_matches)) => {
            if !in_angreal_project {
                error!("This doesn't appear to be an angreal project.");
                exit(1);
            }

            // Build command tree from registry
            let mut root = command_tree::CommandNode::new_group("angreal".to_string(), None);
            for (_, cmd) in ANGREAL_TASKS.lock().unwrap().iter() {
                root.add_command(cmd.clone());
            }

            let long = sub_matches.get_flag("long");
            tree_output::print_tree(&root, long);

            return Ok(());
        }
        Some((task, sub_m)) => {
            if !in_angreal_project {
                error!("This doesn't appear to be an angreal project.");
                exit(1)
            }

            let mut command_groups: Vec<String> = Vec::new();
            command_groups.push(task.to_string());

            // iterate matches to get our final command and get our final arg matches
            // object for applying down stream
            let mut next = sub_m.subcommand();
            let mut arg_matches = sub_m.clone();
            while next.is_some() {
                let cmd = next.unwrap();
                command_groups.push(cmd.0.to_string());
                next = cmd.1.subcommand();
                arg_matches = cmd.1.clone();
            }

            let task = command_groups.pop().unwrap();

            // Generate the full path key for command lookup
            let command_path = generate_path_key_from_parts(&command_groups, &task);
            let tasks_registry = ANGREAL_TASKS.lock().unwrap();

            debug!("Looking up command with path: {}", command_path);
            let command = match tasks_registry.get(&command_path) {
                None => {
                    error!("Command '{}' not found.", task);
                    app_copy.print_help().unwrap_or(());
                    exit(1)
                }
                Some(found_command) => found_command,
            };

            debug!("Executing command: {}", task);

            let args = builder::select_args(&command_path);
            Python::attach(|py| {
                debug!("Starting Python execution for command: {}", task);
                let mut kwargs: Vec<(&str, Py<PyAny>)> = Vec::new();

                for arg in args.into_iter() {
                    let n = Box::leak(Box::new(arg.name));
                    // unable to find the value of the passed arg with sub_m when its been wrapped
                    // in a command group

                    if arg.is_flag.unwrap() {
                        let v = arg_matches.get_flag(&n.clone());
                        kwargs.push((
                            n.as_str(),
                            v.into_bound_py_any(py)
                                .expect("Failed to convert to Python object")
                                .unbind(),
                        ));
                    } else {
                        let v = arg_matches.value_of(n.clone());
                        match v {
                            None => {
                                // We need to handle "boolean flags" that are present w/o a value
                                // should probably test that the name is a "boolean type also"
                                kwargs.push((
                                    n.as_str(),
                                    v.into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                ));
                            }
                            Some(v) => match arg.python_type.unwrap().as_str() {
                                "str" => kwargs.push((
                                    n.as_str(),
                                    v.into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                                "int" => kwargs.push((
                                    n.as_str(),
                                    v.parse::<i32>()
                                        .unwrap()
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                                "float" => kwargs.push((
                                    n.as_str(),
                                    v.parse::<f32>()
                                        .unwrap()
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                                _ => kwargs.push((
                                    n.as_str(),
                                    v.into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                            },
                        }
                    }
                }

                let kwargs_dict = match kwargs.into_py_dict(py) {
                    Ok(dict) => dict,
                    Err(err) => {
                        error!("Failed to convert kwargs to dict");
                        let formatter = PythonErrorFormatter::new(err);
                        println!("{}", formatter);
                        exit(1);
                    }
                };
                let r_value = command.func.call(py, (), Some(&kwargs_dict));

                match r_value {
                    Ok(_r_value) => debug!("Successfully executed Python command: {}", task),
                    Err(err) => {
                        error!("Failed to execute Python command: {}", task);
                        let formatter = PythonErrorFormatter::new(err);
                        println!("{}", formatter);
                        exit(1);
                    }
                }
            });
        }
        _ => {
            println!("process for current context")
        }
    }

    debug!("Angreal application completed successfully.");
    Ok(())
}
```

</details>
