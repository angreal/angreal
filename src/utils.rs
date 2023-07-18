//! Filesystem utilities

use glob::glob;
use log::{debug, error, info};
use std::env;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use pyo3::PyResult;
use std::fs;

use reqwest;
use version_compare::Version;

pub fn check_up_to_date() {
    let body = reqwest::blocking::get("https://pypi.org/pypi/angreal/json")
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();

    let upstream = body["info"]["version"].as_str().unwrap();

    let current = env!("CARGO_PKG_VERSION");
    let current = Version::from(current).unwrap();
    let upstream = Version::from(upstream).unwrap();

    if upstream > current {
        println!("A newer version of angreal is available, use pip install --upgrade angreal to upgrade.")
    };
}

/// Get a list of task files in given a path
///
/// # Examples
///
/// ```
/// use angreal::utils::get_task_files;
/// use std::path::PathBuf;
///
/// let task_files = get_task_files(PathBuf::new("."))
/// ```
pub fn get_task_files(path: PathBuf) -> Result<Vec<PathBuf>, &'static str> {
    let mut tasks = Vec::new();

    let mut pattern = path;
    pattern.push("task_*.py");
    let mut have_tasks = false;

    for entry in glob(pattern.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                info!("Found task {:?}", path.display());
                tasks.push(path);
                have_tasks = true;
            }
            Err(e) => error!("{:?}", e),
        }
    }

    if have_tasks {
        Ok(tasks)
    } else {
        error!("No tasks found for execution.");
        Err("No tasks found for execution.")
    }
}

/// Registers the Command and Arg structs to the python api in the `angreal` module
pub fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_root, m)?)?;
    Ok(())
}

/// Get the root path of a current angreal project.
///
/// The root is the actual location of the .angreal file that houses task files
/// # Examples
/// ```python
/// import angreal
/// angreal_root = angreal.get_root()
/// ```
#[pyfunction]
fn get_root() -> PyResult<String> {
    let angreal_root = is_angreal_project().unwrap();
    Ok(String::from(angreal_root.to_string_lossy()))
}

/// Tests whether or not a current path is an angreal project
///
/// An angreal project is detected by attempting to find a `.angreal` file
/// anywhere in the current and parent directories.
/// # Examples
/// ```
/// use angreal::utils::is_angreal_project
///
/// let project_path = is_angreal_project()
/// ```
pub fn is_angreal_project() -> Result<PathBuf, &'static str> {
    let angreal_path = Path::new(".angreal");

    let mut check_dir = env::current_dir().unwrap();
    check_dir.push(angreal_path);

    let found = loop {
        if check_dir.is_dir() {
            break true;
        }

        let mut next_dir = check_dir.clone();
        next_dir.pop();
        next_dir.pop();
        next_dir.push(angreal_path);

        if next_dir == check_dir {
            break false;
        }

        check_dir = next_dir.clone();
    };

    if found {
        Ok(check_dir)
    } else {
        Err("This doesn't appear to be an angreal project.")
    }
}

/// Loads a python file as a pyo3 PyModule
///
/// # Example
/// ```
/// use angreal::utils::load_python
/// use std::path::PathBuf;
///
/// load_python(PathBuf::new("python_file.py"))?;
/// ```
pub fn load_python(file: PathBuf) -> Result<(), PyErr> {
    let mut dir = file.clone();
    dir.pop();

    let dir = dir.to_str();
    let file = fs::read_to_string(file).unwrap();

    let r_value = Python::with_gil(|py| -> PyResult<()> {
        // Allow the file to search for modules it might be importing
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, dir)?;

        // Import the file.
        let result = PyModule::from_code(py, &file, "", "");

        match result {
            Ok(_result) => {
                debug!("Successfully loaded {:?}", &file);
                Ok(())
            }
            Err(err) => {
                error!(
                    "{:?} failed to load with the following error\n{}",
                    &file, err
                );
                Err(err)
            }
        }
    });

    match r_value {
        Ok(_ok) => Ok(()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
#[path = "../tests"]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    mod common;

    #[test]
    fn test_load_python() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let should_pass = [
            "tests/common/test_assets/good_init.py",
            "tests/common/test_assets/good_task.py",
            "tests/common/test_assets/no_func_init.py",
            "tests/common/test_assets/no_func_task.py",
            "tests/common/test_assets/exception_init.py",
            "tests/common/test_assets/exception_task.py",
        ];

        for f_name in &should_pass {
            let file = PathBuf::from(String::from(*f_name));
            let rv = crate::utils::load_python(root.join(file)).is_ok();
            assert!(rv);
        }

        let shouldnt_pass = [
            "tests/common/test_assets/bad_import_init.py",
            "tests/common/test_assets/bad_import_task.py",
        ];

        for f_name in &shouldnt_pass {
            let file = PathBuf::from(String::from(*f_name));
            let rv = crate::utils::load_python(root.join(file)).is_err();
            assert!(rv);
        }
    }

    #[test]
    fn test_is_angreal_project() {
        let starting_dir = std::env::current_dir().unwrap();
        let tmp_dir = common::make_tmp_dir();
        std::env::set_current_dir(&tmp_dir).unwrap_or(());

        assert!(crate::utils::is_angreal_project().is_err());

        std::env::set_current_dir(starting_dir).unwrap_or(());
        fs::remove_dir_all(&tmp_dir).unwrap_or(());
    }

    #[test]
    fn test_is_not_angreal_project() {
        let starting_dir = std::env::current_dir().unwrap();
        let tmp_dir = common::make_tmp_dir();
        std::env::set_current_dir(&tmp_dir).unwrap_or(());

        fs::create_dir(Path::new(".angreal")).unwrap_or(());
        assert!(crate::utils::is_angreal_project().is_ok());

        std::env::set_current_dir(starting_dir).unwrap_or(());
        fs::remove_dir_all(&tmp_dir).unwrap_or(());
    }

    #[test]
    fn test_get_task_files() {
        let starting_dir = std::env::current_dir().unwrap();
        let tmp_dir = common::make_tmp_dir();
        std::env::set_current_dir(&tmp_dir).unwrap_or(());
        fs::create_dir(Path::new(".angreal")).unwrap_or(());

        let files_to_make = ["task_test_task.py", "not_this_file.py", "task_not_this.txt"];

        for f_name in &files_to_make {
            let mut f_path = tmp_dir.clone();
            f_path.push(Path::new(".angreal"));
            f_path.push(Path::new(f_name));
            let _ = fs::File::create(&f_path);
        }

        let files_should_find = vec![tmp_dir.join(".angreal").join("task_test_task.py")];

        let files_found = crate::utils::get_task_files(tmp_dir.join(".angreal")).unwrap();

        assert_eq!(files_found, files_should_find);

        std::env::set_current_dir(starting_dir).unwrap_or(());
        fs::remove_dir_all(&tmp_dir).unwrap_or(());
    }
}
