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
pub mod task;
pub mod utils;


use builder::build_app;
use pyo3::types::IntoPyDict;
use crate::task::ANGREAL_TASKS;
use log::{debug, error};
use std::vec::Vec;



use std::ops::Deref;
use std::process::exit;

use pyo3::prelude::*;




/// The main function is just an entry point to be called from the core angreal library.
#[pyfunction]
fn main() -> PyResult<()> {
    // we have to do this because we're calling the main function through python - when lib+bin build support is available, we can factor away
    let mut argvs: Vec<String> = std::env::args().collect();
    argvs.remove(0);
    argvs.remove(0);

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();



    // Load any angreal task assets that are available to us
    let in_angreal_project = utils::is_angreal_project().is_ok();
    let angreal_path = utils::is_angreal_project().unwrap();

    if in_angreal_project {
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
            utils::load_python(task.clone());
        }
    }

    
    let app = build_app();
    let mut app_copy = app.clone();
    let sub_command = app.get_matches_from(&argvs);

    match sub_command.subcommand() {
        Some(("init", _sub_matches)) => {
            println!("INIT");
        }
        Some((task, sub_m)) => {
            let some_command = ANGREAL_TASKS.lock().unwrap().clone();
            let some_command = some_command.iter().find(|&x| x.name == task);


            let command = match some_command {
                None => {
                    error!("Task {}, not found.", task.clone());
                    app_copy.print_help();
                    exit(1)
                }
                Some(some_command) => some_command,
            };

            let args = builder::select_args(task.to_string());

            let mut kwargs: Vec<(&str,&str)> = Vec::new();

            for arg in args.into_iter(){
                let n = Box::leak(Box::new(arg.name));
                let v = sub_m.value_of(n.clone().as_str());

                match v {
                    None => (),
                    Some(v) => kwargs.push((n.as_str(),v)),
                }
                
            }

            Python::with_gil(|py| {
                command.func.call(py, (), Some(kwargs.into_py_dict(py)));
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
    Ok(())
}