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
pub mod git;
pub mod init;
pub mod logger;
pub mod py_logger;
pub mod task;
pub mod utils;

use builder::build_app;
use task::ANGREAL_TASKS;

use log::{debug, error};
use pyo3::types::IntoPyDict;
use std::ops::Not;
use std::vec::Vec;

use std::process::exit;

use pyo3::prelude::*;

/// The main function is just an entry point to be called from the core angreal library.
#[pyfunction]
fn main() -> PyResult<()> {
    let handle = logger::init_logger();
    // we have to do this because we're calling the main function through python - when lib+bin build support is available, we can factor away
    let mut argvs: Vec<String> = std::env::args().collect();
    argvs.remove(0);
    argvs.remove(0);

    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warning"))
    // .init();

    // Load any angreal task assets that are available to us
    let in_angreal_project = utils::is_angreal_project().is_ok();

    if in_angreal_project {
        let angreal_path = utils::is_angreal_project().unwrap();
        debug!("Angreal project detected, loading found tasks.");
        // get a list of files
        let angreal_tasks_to_load = utils::get_task_files(angreal_path);

        // Explicitly capture error with exit
        let _angreal_tasks_to_load = match angreal_tasks_to_load {
            Ok(tasks) => tasks,
            Err(_) => exit(1),
        };

        // load the files , IF a file has command or task decorators - they'll register themselves now
        for task in _angreal_tasks_to_load.iter() {
            utils::load_python(task.clone()).unwrap_or(());
        }
    }

    let app = build_app();
    let mut app_copy = app.clone();
    let sub_command = app.get_matches_from(&argvs);

    // Get our asked for verbosity and set the logger up. TODO: find a way to initialize earlier and reset after.
    let verbosity = sub_command.get_count("verbose");

    logger::update_verbosity(&handle, verbosity);

    match sub_command.subcommand() {
        Some(("init", _sub_matches)) => init::init(
            _sub_matches.value_of("template").unwrap(),
            _sub_matches.is_present("force"),
            _sub_matches.is_present("defaults").not(),
        ),
        Some((task, sub_m)) => {
            if !in_angreal_project {
                error!("This doesn't appear to be an angreal project.");
                exit(1)
            }
            let some_command = ANGREAL_TASKS.lock().unwrap().clone();
            let some_command = some_command.iter().find(|&x| x.name == task);

            let command = match some_command {
                None => {
                    error!("Task {}, not found.", <&str>::clone(&task));
                    app_copy.print_help().unwrap_or(());
                    exit(1)
                }
                Some(some_command) => some_command,
            };

            let args = builder::select_args(task.to_string());

            Python::with_gil(|py| {
                let mut kwargs: Vec<(&str, PyObject)> = Vec::new();

                for arg in args.into_iter() {
                    let n = Box::leak(Box::new(arg.name));
                    let v = sub_m.value_of(n.clone());
                    match v {
                        None => {
                            // We need to handle "boolean flags" that are present w/o a value
                            // should probably test that the name is a "boolean type also"
                            let v = sub_m.is_present(n.clone());
                            kwargs.push((n.as_str(), v.to_object(py)));
                        }
                        Some(v) => {
                            match arg.python_type.unwrap().as_str() {
                                "str" => kwargs.push((n.as_str(), v.to_object(py))),
                                "int" => kwargs
                                    .push((n.as_str(), v.parse::<i32>().unwrap().to_object(py))),
                                "float" => kwargs
                                    .push((n.as_str(), v.parse::<f32>().unwrap().to_object(py))),
                                _ => kwargs.push((n.as_str(), v.to_object(py))),
                            }
                        }
                    }
                }

                let r_value = command.func.call(py, (), Some(kwargs.into_py_dict(py)));

                match r_value {
                    Ok(_r_value) => {}
                    Err(r_value) => {
                        error!("An error occurred :");
                        error!("{:?}", r_value.traceback(py).unwrap().format());
                        exit(1);
                    }
                }
            });
        }
        _ => {
            println!("process for current context")
        }
    }

    Ok(())
}

/// registering the angreal namespace for import from python
#[pymodule]
fn angreal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    task::register(_py, m)?;
    utils::register(_py, m)?;
    py_logger::register();
    Ok(())
}
