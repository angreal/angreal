//!  Angreal - project templating and task management
//!
//!  A package for templating code based projects and providing methods
//! for the creation and management of common operational tasks associated with the
//! project.
//!

#[macro_use]
extern crate version;
#[macro_use]
pub mod macros;

pub mod builder;
pub mod completion;
pub mod error_formatter;
pub mod git;
pub mod init;
pub mod integrations;
pub mod logger;
pub mod py_logger;
pub mod python_bindings;
pub mod task;
pub mod utils;
pub mod validation;

use builder::build_app;
use error_formatter::PythonErrorFormatter;
use integrations::uv::{UvIntegration, UvVirtualEnv};
use task::ANGREAL_TASKS;

use pyo3::types::{IntoPyDict, PyDict};
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use std::process::exit;

use pyo3::{prelude::*, wrap_pymodule, IntoPyObjectExt};
use std::collections::HashMap;
use std::fs;

use log::{debug, error, warn};

use crate::integrations::git::Git;
use crate::task::generate_path_key_from_parts;

#[pyclass]
struct PyGit {
    inner: Git,
}

#[pymethods]
impl PyGit {
    #[new]
    #[pyo3(signature = (working_dir=None))]
    fn new(working_dir: Option<&str>) -> PyResult<Self> {
        let git = Git::new(working_dir.map(Path::new))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(Self { inner: git })
    }

    fn execute(&self, subcommand: &str, args: Vec<String>) -> PyResult<(i32, String, String)> {
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let output = self
            .inner
            .execute(subcommand, &arg_refs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok((output.exit_code, output.stderr, output.stdout))
    }

    fn init(&self, bare: Option<bool>) -> PyResult<()> {
        self.inner
            .init(bare.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn add(&self, paths: Vec<String>) -> PyResult<()> {
        let path_refs: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
        self.inner
            .add(&path_refs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn commit(&self, message: &str, all: Option<bool>) -> PyResult<()> {
        self.inner
            .commit(message, all.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn push(&self, remote: Option<&str>, branch: Option<&str>) -> PyResult<()> {
        self.inner
            .push(remote, branch)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn pull(&self, remote: Option<&str>, branch: Option<&str>) -> PyResult<()> {
        self.inner
            .pull(remote, branch)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn status(&self, short: Option<bool>) -> PyResult<String> {
        self.inner
            .status(short.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn branch(&self, name: Option<&str>, delete: Option<bool>) -> PyResult<String> {
        self.inner
            .branch(name, delete.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn checkout(&self, branch: &str, create: Option<bool>) -> PyResult<()> {
        self.inner
            .checkout(branch, create.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn tag(&self, name: &str, message: Option<&str>) -> PyResult<()> {
        self.inner
            .tag(name, message)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

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
}

#[pyfunction]
#[pyo3(signature = (remote, destination=None))]
fn git_clone(remote: &str, destination: Option<&str>) -> PyResult<String> {
    let dest = Git::clone(remote, destination.map(Path::new))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(dest.display().to_string())
}

#[pyfunction]
fn ensure_uv_installed() -> PyResult<()> {
    UvIntegration::ensure_installed()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn uv_version() -> PyResult<String> {
    UvIntegration::version()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn create_virtualenv(path: &str, python_version: Option<&str>) -> PyResult<()> {
    UvVirtualEnv::create(Path::new(path), python_version)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(())
}

#[pyfunction]
fn install_packages(venv_path: &str, packages: Vec<String>) -> PyResult<()> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.install_packages(&packages)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn install_requirements(venv_path: &str, requirements_file: &str) -> PyResult<()> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.install_requirements(Path::new(requirements_file))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
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

#[pyfunction]
fn install_python(version: &str) -> PyResult<String> {
    UvVirtualEnv::install_python(version)
        .map(|path| path.display().to_string())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn get_venv_activation_info(venv_path: &str) -> PyResult<integrations::uv::ActivationInfo> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.get_activation_info()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
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

#[pyfunction]
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

#[pyfunction]
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

#[pyfunction]
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

/// The main function is just an entry point to be called from the core angreal library.
#[pyfunction]
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

/// Initialize Python bindings and load angreal tasks for external tools
/// This function should be called by any external tool that needs to discover angreal commands
pub fn initialize_python_tasks() -> Result<(), Box<dyn std::error::Error>> {
    use pyo3::types::PyDict;

    debug!("Initializing Python bindings for angreal tasks");

    // First, ensure the angreal module is registered in Python
    Python::attach(|py| -> PyResult<()> {
        // Get sys.modules
        let sys = PyModule::import(py, "sys")?;
        let modules_attr = sys.getattr("modules")?;
        let modules = modules_attr.downcast::<PyDict>()?;

        // Check if angreal module is already available
        if !modules.contains("angreal")? {
            debug!("Registering angreal module in Python sys.modules");

            // Create the angreal module manually
            let angreal_module = PyModule::new(py, "angreal")?;

            // Register the module components (from the pymodule function)
            angreal_module.add("__version__", env!("CARGO_PKG_VERSION"))?;

            // Register logger
            py_logger::register();

            // Register core components
            task::register(py, &angreal_module)?;
            utils::register(py, &angreal_module)?;
            python_bindings::decorators::register_decorators(py, &angreal_module)?;

            // Register integrations submodule (from the full pymodule function)
            angreal_module
                .add_wrapped(wrap_pymodule!(python_bindings::integrations::integrations))?;

            // Set up sys.modules entries for integrations (matching the pymodule function)
            modules.set_item(
                "angreal.integrations",
                angreal_module.getattr("integrations")?,
            )?;
            modules.set_item(
                "angreal.integrations.docker",
                angreal_module
                    .getattr("integrations")?
                    .getattr("docker_integration")?,
            )?;
            modules.set_item(
                "angreal.integrations.git",
                angreal_module
                    .getattr("integrations")?
                    .getattr("git_integration")?,
            )?;

            // Register the main module in sys.modules
            modules.set_item("angreal", angreal_module)?;

            debug!("Successfully registered angreal module in Python");
        } else {
            debug!("Angreal module already available in sys.modules");
        }

        Ok(())
    })?;

    // Check if we're in an angreal project
    let angreal_path =
        utils::is_angreal_project().map_err(|e| format!("Not in angreal project: {}", e))?;

    debug!("Found angreal project at: {}", angreal_path.display());

    // Get task files
    let task_files = utils::get_task_files(angreal_path)
        .map_err(|e| format!("Failed to get task files: {}", e))?;

    debug!("Found {} task files to load", task_files.len());

    // Load each Python task file to populate ANGREAL_TASKS registry
    for task_file in task_files.iter() {
        debug!("Loading Python task file: {}", task_file.display());

        match utils::load_python(task_file.clone()) {
            Ok(_) => debug!("Successfully loaded task file: {}", task_file.display()),
            Err(e) => {
                warn!("Failed to load task file {}: {}", task_file.display(), e);
                // Continue loading other files even if one fails
            }
        }
    }

    let task_count = ANGREAL_TASKS.lock().unwrap().len();
    debug!("Successfully initialized {} angreal tasks", task_count);

    Ok(())
}

#[pymodule]
fn angreal(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    py_logger::register();
    m.add_function(wrap_pyfunction!(main, m)?)?;
    task::register(m.py(), m)?;
    utils::register(m.py(), m)?;

    // Register decorators from our new python_bindings module
    python_bindings::decorators::register_decorators(m.py(), m)?;

    // UV integration functions
    m.add_function(wrap_pyfunction!(ensure_uv_installed, m)?)?;
    m.add_function(wrap_pyfunction!(uv_version, m)?)?;
    m.add_function(wrap_pyfunction!(create_virtualenv, m)?)?;
    m.add_function(wrap_pyfunction!(install_packages, m)?)?;
    m.add_function(wrap_pyfunction!(install_requirements, m)?)?;
    m.add_function(wrap_pyfunction!(discover_pythons, m)?)?;
    m.add_function(wrap_pyfunction!(install_python, m)?)?;
    m.add_function(wrap_pyfunction!(get_venv_activation_info, m)?)?;
    m.add_class::<integrations::uv::ActivationInfo>()?;

    // Entrypoint registration functions
    m.add_function(wrap_pyfunction!(register_entrypoint, m)?)?;
    m.add_function(wrap_pyfunction!(list_entrypoints, m)?)?;
    m.add_function(wrap_pyfunction!(unregister_entrypoint, m)?)?;
    m.add_function(wrap_pyfunction!(cleanup_entrypoints, m)?)?;

    let integrations_module = PyModule::new(m.py(), "integrations")?;
    python_bindings::integrations::integrations(m.py(), &integrations_module)?;
    m.add_submodule(&integrations_module)?;

    let sys = PyModule::import(m.py(), "sys")?;
    let modules_attr = sys.getattr("modules")?;
    let sys_modules = modules_attr.downcast::<PyDict>()?;
    sys_modules.set_item("angreal.integrations", m.getattr("integrations")?)?;
    sys_modules.set_item(
        "angreal.integrations.docker",
        m.getattr("integrations")?.getattr("docker_integration")?,
    )?;

    sys_modules.set_item(
        "angreal.integrations.docker.image",
        m.getattr("integrations")?
            .getattr("docker_integration")?
            .getattr("image")?,
    )?;
    sys_modules.set_item(
        "angreal.integrations.docker.container",
        m.getattr("integrations")?
            .getattr("docker_integration")?
            .getattr("container")?,
    )?;
    sys_modules.set_item(
        "angreal.integrations.docker.network",
        m.getattr("integrations")?
            .getattr("docker_integration")?
            .getattr("network")?,
    )?;
    sys_modules.set_item(
        "angreal.integrations.docker.volume",
        m.getattr("integrations")?
            .getattr("docker_integration")?
            .getattr("volume")?,
    )?;

    sys_modules.set_item(
        "angreal.integrations.git",
        m.getattr("integrations")?.getattr("git_integration")?,
    )?;

    Ok(())
}

#[pymodule]
fn _integrations(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let docker_module = pyo3::wrap_pymodule!(docker)(m.py());
    m.add_submodule(docker_module.bind(m.py()))?;
    let git = pyo3::wrap_pymodule!(git_module)(m.py());
    m.add_submodule(git.bind(m.py()))?;
    Ok(())
}

#[pymodule]
fn docker(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<docker_pyo3::Pyo3Docker>()?;

    let image_module = PyModule::new(m.py(), "image")?;
    docker_pyo3::image::image(m.py(), &image_module)?;
    m.add_submodule(&image_module)?;

    let container_module = PyModule::new(m.py(), "container")?;
    docker_pyo3::container::container(m.py(), &container_module)?;
    m.add_submodule(&container_module)?;

    let network_module = PyModule::new(m.py(), "network")?;
    docker_pyo3::network::network(m.py(), &network_module)?;
    m.add_submodule(&network_module)?;

    let volume_module = PyModule::new(m.py(), "volume")?;
    docker_pyo3::volume::volume(m.py(), &volume_module)?;
    m.add_submodule(&volume_module)?;
    Ok(())
}

#[pymodule]
fn git_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGit>()?;
    m.add_function(wrap_pyfunction!(git_clone, m)?)?;
    Ok(())
}
