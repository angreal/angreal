---
title: "angreal"
description: "Angreal - project templating and task management"
sidebar:
  order: 1
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


Angreal - project templating and task management

A package for templating code based projects and providing methods
for the creation and management of common operational tasks associated with the
project.

## Classes

### <span id="class.PyGit"></span>`class PyGit`

> **Rust Implementation**: [angreal::PyGit](../rust/angreal/#struct.PyGit)

#### Methods

##### <span id="PyGit.new"></span>`new`

```python
def new(working_dir: Option < & str >) -> PyResult < Self >
```

> **Rust Implementation**: [angreal::PyGit::new](../rust/angreal/#PyGit.new)

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



##### <span id="PyGit.execute"></span>`execute`

```python
def execute(
    subcommand:  str,
    args: Vec < String >,
) -> PyResult < (i32 , String , String) >
```

> **Rust Implementation**: [angreal::PyGit::execute](../rust/angreal/#PyGit.execute)

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



##### <span id="PyGit.init"></span>`init`

```python
def init(bare: Option < bool >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::PyGit::init](../rust/angreal/#PyGit.init)

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



##### <span id="PyGit.add"></span>`add`

```python
def add(paths: Vec < String >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::PyGit::add](../rust/angreal/#PyGit.add)

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



##### <span id="PyGit.commit"></span>`commit`

```python
def commit(message:  str, all: Option < bool >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::PyGit::commit](../rust/angreal/#PyGit.commit)

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



##### <span id="PyGit.push"></span>`push`

```python
def push(remote: Option < & str >, branch: Option < & str >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::PyGit::push](../rust/angreal/#PyGit.push)

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



##### <span id="PyGit.pull"></span>`pull`

```python
def pull(remote: Option < & str >, branch: Option < & str >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::PyGit::pull](../rust/angreal/#PyGit.pull)

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



##### <span id="PyGit.status"></span>`status`

```python
def status(short: Option < bool >) -> PyResult < String >
```

> **Rust Implementation**: [angreal::PyGit::status](../rust/angreal/#PyGit.status)

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



##### <span id="PyGit.branch"></span>`branch`

```python
def branch(name: Option < & str >, delete: Option < bool >) -> PyResult < String >
```

> **Rust Implementation**: [angreal::PyGit::branch](../rust/angreal/#PyGit.branch)

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



##### <span id="PyGit.checkout"></span>`checkout`

```python
def checkout(branch:  str, create: Option < bool >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::PyGit::checkout](../rust/angreal/#PyGit.checkout)

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



##### <span id="PyGit.tag"></span>`tag`

```python
def tag(name:  str, message: Option < & str >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::PyGit::tag](../rust/angreal/#PyGit.tag)

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



##### <span id="PyGit.__call__"></span>`__call__`

```python
def __call__(
    command:  str,
    args: Vec < String >,
    kwargs: Option < & Bound < '_ , pyo3 :: types :: PyDict > >,
) -> PyResult < (i32 , String , String) >
```

> **Rust Implementation**: [angreal::PyGit::__call__](../rust/angreal/#PyGit.__call__)

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

### <span id="fn.git_clone"></span>`git_clone`

```python
def git_clone(remote:  str, destination: Option < & str >) -> PyResult < String >
```

> **Rust Implementation**: [angreal::git_clone](../rust/angreal/#fn.git_clone)

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



### <span id="fn.ensure_uv_installed"></span>`ensure_uv_installed`

```python
def ensure_uv_installed() -> PyResult < () >
```

> **Rust Implementation**: [angreal::ensure_uv_installed](../rust/angreal/#fn.ensure_uv_installed)

<details>
<summary>Source</summary>

```python
fn ensure_uv_installed() -> PyResult<()> {
    UvIntegration::ensure_installed()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### <span id="fn.uv_version"></span>`uv_version`

```python
def uv_version() -> PyResult < String >
```

> **Rust Implementation**: [angreal::uv_version](../rust/angreal/#fn.uv_version)

<details>
<summary>Source</summary>

```python
fn uv_version() -> PyResult<String> {
    UvIntegration::version()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>



### <span id="fn.create_virtualenv"></span>`create_virtualenv`

```python
def create_virtualenv(path:  str, python_version: Option < & str >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::create_virtualenv](../rust/angreal/#fn.create_virtualenv)

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



### <span id="fn.install_packages"></span>`install_packages`

```python
def install_packages(venv_path:  str, packages: Vec < String >) -> PyResult < () >
```

> **Rust Implementation**: [angreal::install_packages](../rust/angreal/#fn.install_packages)

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



### <span id="fn.install_requirements"></span>`install_requirements`

```python
def install_requirements(venv_path:  str, requirements_file:  str) -> PyResult < () >
```

> **Rust Implementation**: [angreal::install_requirements](../rust/angreal/#fn.install_requirements)

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



### <span id="fn.discover_pythons"></span>`discover_pythons`

```python
def discover_pythons() -> PyResult < Vec < (String , String) > >
```

> **Rust Implementation**: [angreal::discover_pythons](../rust/angreal/#fn.discover_pythons)

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



### <span id="fn.install_python"></span>`install_python`

```python
def install_python(version:  str) -> PyResult < String >
```

> **Rust Implementation**: [angreal::install_python](../rust/angreal/#fn.install_python)

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



### <span id="fn.get_venv_activation_info"></span>`get_venv_activation_info`

```python
def get_venv_activation_info(
    venv_path:  str,
) -> PyResult < integrations :: uv :: ActivationInfo >
```

> **Rust Implementation**: [angreal::get_venv_activation_info](../rust/angreal/#fn.get_venv_activation_info)

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



### <span id="fn.register_entrypoint"></span>`register_entrypoint`

```python
def register_entrypoint(name:  str) -> PyResult < () >
```

> **Rust Implementation**: [angreal::register_entrypoint](../rust/angreal/#fn.register_entrypoint)

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



### <span id="fn.list_entrypoints"></span>`list_entrypoints`

```python
def list_entrypoints() -> PyResult < Vec < String > >
```

> **Rust Implementation**: [angreal::list_entrypoints](../rust/angreal/#fn.list_entrypoints)

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



### <span id="fn.unregister_entrypoint"></span>`unregister_entrypoint`

```python
def unregister_entrypoint(name:  str) -> PyResult < () >
```

> **Rust Implementation**: [angreal::unregister_entrypoint](../rust/angreal/#fn.unregister_entrypoint)

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



### <span id="fn.cleanup_entrypoints"></span>`cleanup_entrypoints`

```python
def cleanup_entrypoints() -> PyResult < () >
```

> **Rust Implementation**: [angreal::cleanup_entrypoints](../rust/angreal/#fn.cleanup_entrypoints)

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



### <span id="fn.main"></span>`main`

```python
def main() -> PyResult < () >
```

> **Rust Implementation**: [angreal::main](../rust/angreal/#fn.main)

The main function is just an entry point to be called from the core angreal library.

<details>
<summary>Source</summary>

```python
fn main(py: Python<'_>) -> PyResult<()> {
    let handle = logger::init_logger();
    if std::env::var("ANGREAL_DEBUG").unwrap_or_default() == "true" {
        logger::update_verbosity(&handle, 2);
        warn!("Angreal application starting with debug level logging from environment");
    }
    debug!("Angreal application starting...");

    // Pull argv from Python's sys.argv rather than std::env::args(). musl does
    // not pass argc/argv to .init_array constructors in dlopened shared objects,
    // so the Rust runtime never populates its ARGV static and std::env::args()
    // returns empty — the historical split_off(2) then panicked with
    // "2 > 0". sys.argv is owned by the Python interpreter and is always
    // populated, regardless of libc. Drop sys.argv[0] (the script path); the
    // remaining elements are the user args. clap's app has NoBinaryName set,
    // so no synthetic argv[0] is needed.
    let sys = py.import("sys")?;
    let py_argv: Vec<String> = sys.getattr("argv")?.extract()?;
    let argvs: Vec<String> = py_argv.into_iter().skip(1).collect();

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
            _sub_matches.is_present("in_place"),
            _sub_matches.is_present("insecure"),
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
        Some(("mcp", _)) => {
            if !in_angreal_project {
                error!("This doesn't appear to be an angreal project.");
                exit(1);
            }

            mcp::serve();
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

            // Generate the logical path key for command lookup
            let command_path = generate_path_key_from_parts(&command_groups, &task);
            let tasks_registry = ANGREAL_TASKS.lock().unwrap();

            debug!("Looking up command with path: {}", command_path);
            // Find the command by its logical path (registry keys include a
            // unique suffix to prevent collisions during decoration).
            let (registry_key, command) = match tasks_registry
                .iter()
                .find(|(_, cmd)| generate_command_path_key(cmd) == command_path)
            {
                None => {
                    error!("Command '{}' not found.", task);
                    app_copy.print_help().unwrap_or(());
                    exit(1)
                }
                Some((key, found_command)) => (key.clone(), found_command),
            };

            debug!(
                "Executing command: {} (registry key: {})",
                task, registry_key
            );

            let args = builder::select_args(&registry_key);
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
                    Ok(r_value) => {
                        // Check bool before int — in Python, bool is a subtype of int
                        // (True == 1, False == 0), so extract::<i32> would match bools
                        if let Ok(val) = r_value.extract::<bool>(py) {
                            if !val {
                                exit(1);
                            }
                        } else if let Ok(code) = r_value.extract::<i32>(py) {
                            if code != 0 {
                                exit(code);
                            }
                        }
                        // None, True, or other → success
                        debug!("Successfully executed Python command: {}", task);
                    }
                    Err(err) => {
                        // SystemExit → exit with the original code
                        let is_sys_exit = err
                            .value(py)
                            .get_type()
                            .name()
                            .map(|n| n == "SystemExit")
                            .unwrap_or(false);
                        if is_sys_exit {
                            let code = err
                                .value(py)
                                .getattr("code")
                                .and_then(|c| c.extract::<i32>())
                                .unwrap_or(1);
                            exit(code);
                        }
                        error!("Failed to execute Python command: {}", task);
                        let formatter = PythonErrorFormatter::new(err);
                        println!("{}", formatter);
                        exit(56);
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
