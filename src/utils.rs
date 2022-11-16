//! Just  random utilities we need
//!
//!
//!

use glob::glob;
use log::{debug, error, info};
use std::env;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use pyo3::PyResult;
use std::fs;

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

pub fn is_angreal_project() -> Result<PathBuf, &'static str> {
    let angreal_path = Path::new(".angreal");

    let mut check_dir = env::current_dir().unwrap();

    check_dir.push(angreal_path);

    let found = loop {
        debug!("checking for {}", check_dir.display());
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
        info!(".angreal found at : {:?}", check_dir.display());
        Ok(check_dir)
    } else {
        error!("This doesn't appear to be an angreal project.");
        Err("This doesn't appear to be an angreal project.")
    }
}

pub fn load_python(file: &PathBuf) -> Result<(), PyErr> {
    let mut dir = file.clone();
    dir.pop();

    let dir = dir.to_str();
    let file = fs::read_to_string(file).unwrap();

    let r_value = Python::with_gil(|py| -> PyResult<()> {
        // Allow the file to search for modules it might be importing
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &dir)?;

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
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    
    #[test]
    fn test_is_not_angreal_project() {
        fs::create_dir(Path::new(".angreal")).unwrap();
        let mut is_angreal = crate::utils::is_angreal_project();
        fs::remove_dir(Path::new(".angreal")).unwrap();

        let mut current_dir = env::current_dir().unwrap();
        current_dir.push(Path::new(".angreal"));

        assert!(is_angreal.is_ok());

        is_angreal = crate::utils::is_angreal_project();
        assert!(is_angreal.is_err())
    }

    #[test]
    fn test_get_task_files() {
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push(Path::new(".angreal"));
        let mut file_1 = tmp_dir.clone();
        let mut file_2 = tmp_dir.clone();
        let mut file_3 = tmp_dir.clone();

        fs::create_dir(tmp_dir.clone()).unwrap();
        file_1.push(Path::new("task_test_task.py"));
        file_2.push(Path::new("not_this_file.py"));
        file_3.push(Path::new("task_not_this.txt"));

        let files_should_find = vec![file_1.clone()];
        fs::File::create(file_1.clone()).unwrap();
        fs::File::create(file_2.clone()).unwrap();
        fs::File::create(file_3.clone()).unwrap();

        let files_found = crate::utils::get_task_files(tmp_dir.clone()).unwrap();

        fs::remove_dir_all(tmp_dir.clone()).unwrap();

        assert_eq!(files_found, files_should_find);

        for f in files_found.iter() {
            println!("{:?}", f.display());
        }
    }
}
