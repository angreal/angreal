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
pub mod error_formatter;
pub mod git;
pub mod init;
pub mod logger;
pub mod py_logger;
pub mod task;
pub mod utils;
pub mod validation;

use builder::build_app;
use error_formatter::PythonErrorFormatter;
use task::ANGREAL_TASKS;

use pyo3::types::{IntoPyDict, PyDict};
use std::ops::Not;
use std::vec::Vec;

use std::process::exit;

use pyo3::{prelude::*, wrap_pymodule};

use log::{debug, error, warn};

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

    debug!("Checking if binary is up to date...");
    match utils::check_up_to_date() {
        Ok(()) => (),
        Err(e) => warn!(
            "An error occurred while checking if our binary is up to date. {}",
            e.to_string()
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

            let some_command = ANGREAL_TASKS.lock().unwrap().clone();
            let some_command = some_command.iter().find(|&x| {
                x.name == task.as_str()
                    && x.group
                        .clone()
                        .unwrap()
                        .iter()
                        .map(|x| x.name.to_string())
                        .collect::<Vec<String>>()
                        == command_groups
            });

            debug!("Executing command: {}", task);
            let command = match some_command {
                None => {
                    error!("Command '{}' not found.", task);
                    app_copy.print_help().unwrap_or(());
                    exit(1)
                }
                Some(some_command) => some_command,
            };

            let args = builder::select_args(task.as_str());
            Python::with_gil(|py| {
                debug!("Starting Python execution for command: {}", task);
                let mut kwargs: Vec<(&str, PyObject)> = Vec::new();

                for arg in args.into_iter() {
                    let n = Box::leak(Box::new(arg.name));
                    // unable to find the value of the passed arg with sub_m when its been wrapped
                    // in a command group

                    if arg.is_flag.unwrap() {
                        let v = arg_matches.get_flag(&n.clone());
                        kwargs.push((n.as_str(), v.to_object(py)));
                    } else {
                        let v = arg_matches.value_of(n.clone());
                        match v {
                            None => {
                                // We need to handle "boolean flags" that are present w/o a value
                                // should probably test that the name is a "boolean type also"
                                kwargs.push((n.as_str(), v.to_object(py)));
                            }
                            Some(v) => match arg.python_type.unwrap().as_str() {
                                "str" => kwargs.push((n.as_str(), v.to_object(py))),
                                "int" => kwargs
                                    .push((n.as_str(), v.parse::<i32>().unwrap().to_object(py))),
                                "float" => kwargs
                                    .push((n.as_str(), v.parse::<f32>().unwrap().to_object(py))),
                                _ => kwargs.push((n.as_str(), v.to_object(py))),
                            },
                        }
                    }
                }

                let r_value = command.func.call(py, (), Some(kwargs.into_py_dict(py)));

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

#[pymodule]
fn angreal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    py_logger::register();
    m.add_function(wrap_pyfunction!(main, m)?)?;
    task::register(_py, m)?;
    utils::register(_py, m)?;

    m.add_wrapped(wrap_pymodule!(_integrations))?;

    let sys = PyModule::import(_py, "sys")?;
    let sys_modules: &PyDict = sys.getattr("modules")?.downcast()?;
    sys_modules.set_item("angreal._integrations", m.getattr("_integrations")?)?;
    sys_modules.set_item(
        "angreal._integrations.docker",
        m.getattr("_integrations")?.getattr("docker")?,
    )?;

    sys_modules.set_item(
        "angreal._integrations.docker.image",
        m.getattr("_integrations")?
            .getattr("docker")?
            .getattr("image")?,
    )?;
    sys_modules.set_item(
        "angreal._integrations.docker.container",
        m.getattr("_integrations")?
            .getattr("docker")?
            .getattr("container")?,
    )?;
    sys_modules.set_item(
        "angreal._integrations.docker.network",
        m.getattr("_integrations")?
            .getattr("docker")?
            .getattr("network")?,
    )?;
    sys_modules.set_item(
        "angreal._integrations.docker.volume",
        m.getattr("_integrations")?
            .getattr("docker")?
            .getattr("volume")?,
    )?;
    Ok(())
}

#[pymodule]
fn _integrations(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(docker))?;
    Ok(())
}

#[pymodule]
fn docker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<docker_pyo3::Pyo3Docker>()?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::image::image))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::container::container))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::network::network))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::volume::volume))?;
    Ok(())
}
