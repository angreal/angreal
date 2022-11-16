//!  Angreal - project templating and task management
//!
//!  A package for templating code based projects and providing methods
//! for the creation and management of common operational tasks associated with the
//! project.
//!
pub mod utils;

use log::{debug, error, info, warn};
use std::process::exit;

use pyo3::prelude::*;

/// The main entry point to be called from python.
#[pyfunction]
fn main() -> PyResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    info!("Angreal main function entered.");

    let in_angreal_project = utils::is_angreal_project();

    let angreal_path = match in_angreal_project {
        Ok(path) => path,
        Err(_) => exit(1),
    };

    let angreal_tasks_to_load = utils::get_task_files(angreal_path);

    let _angreal_tasks_to_load = match angreal_tasks_to_load {
        Ok(tasks) => tasks,
        Err(_) => exit(1),
    };

    Ok(())
}

/// registering the angreal namespace for import from python
#[pymodule]
fn angreal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}
