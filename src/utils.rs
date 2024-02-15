//! Filesystem utilities
use anyhow::anyhow;
use anyhow::Result;

use glob::glob;
use std::env;
use std::ops::Not;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use tera::Context;
use toml::{map::Map, Table, Value};

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule};
use pyo3::PyResult;
use std::fs;
use std::fs::File;
use std::io::Write;

use tera::Tera;
use text_io::read;

use reqwest::{self};
use version_compare::Version;

use walkdir::WalkDir;

use log::{debug, error, info};
use pythonize::pythonize;

macro_rules! result_or_return_err {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(err) => return Err(err).map_err(Into::into),
        }
    };
}

macro_rules! value_or_return_err {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return Err(anyhow!("No value returned when one was expected.")),
        }
    };
}

// turn a tera context into a map
pub fn context_to_map(ctx: Context) -> Map<String, Value> {
    Map::try_from(ctx.into_json().as_object().unwrap().clone()).unwrap()
}

// takes a toml file and creates a Tera context for consumption
// if you wish to take input from stdin set take_input to True, otherwise it will read provided values directly.
pub fn repl_context_from_toml(toml_path: PathBuf, take_input: bool) -> Context {
    let file_contents = fs::read_to_string(&toml_path)
        .unwrap_or_else(|_| panic!("Unable to open {:?}", &toml_path));
    let extract = file_contents.parse::<Table>().unwrap();

    let mut context = Context::new();

    for (k, v) in extract.iter() {
        let value = if v.is_str()
            && v.as_str().unwrap().starts_with("{{")
            && v.as_str().unwrap().contains("}}")
        {
            let temp_value = v.clone();
            let rendered_value =
                Tera::one_off(temp_value.as_str().unwrap(), &context, false).unwrap();
            Value::from(rendered_value)
        } else {
            v.clone()
        };

        let input = if take_input {
            print!("{k}? [{value}]: ");
            read!("{}\n")
        } else {
            String::new()
        };

        if input.trim().is_empty() | take_input.not() {
            if value.is_str() {
                context.insert(k, &value.as_str().unwrap());
            }
            if value.is_integer() {
                context.insert(k, &value.as_integer().unwrap());
            }
            if value.is_bool() {
                context.insert(k, &value.as_bool().unwrap());
            }
            if value.is_float() {
                context.insert(k, &value.as_float().unwrap());
            }
        } else {
            if value.is_str() {
                context.insert(k, &input.trim());
            }
            if value.is_integer() {
                context.insert(k, &input.trim().parse::<i32>().unwrap());
            }
            if value.is_bool() {
                context.insert(k, &input.trim());
            }
            if value.is_float() {
                context.insert(k, &input.trim().parse::<f64>().unwrap());
            }
        }
    }

    context
}

// Render a templated directory to a destination given a tera context
pub fn render_dir(src: &Path, context: Context, dst: &Path, force: bool) -> Vec<String> {
    let mut rendered_paths: Vec<String> = Vec::new();
    // we create a Tera instance for an empty directory so we can extend it with our template later
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push(Path::new("angreal_tmp"));

    if tmp_dir.is_dir().not() {
        debug!("Creating tmpdir at {:?}", tmp_dir);
        fs::create_dir(&tmp_dir).unwrap();
    }

    tmp_dir.push(Path::new("*"));
    let mut tera = Tera::new(tmp_dir.to_str().unwrap()).unwrap();

    tmp_dir.pop();
    if tmp_dir.is_dir() {
        debug!("Destroying tmpdir at {:?}", tmp_dir);
        fs::remove_dir_all(&tmp_dir).unwrap();
    }

    // We glob our template directory
    let mut template_src = <&std::path::Path>::clone(&src).to_path_buf();
    template_src.push(Path::new("**/*"));

    // And build our full prefix
    let _template_name = <&std::path::Path>::clone(&src).file_name().unwrap();

    for file in glob(template_src.to_str().unwrap()).expect("Failed to read glob pattern") {
        let file_path = file.as_ref().unwrap();
        let rel_path = file_path.strip_prefix(src).unwrap().to_str().unwrap();

        if file.as_ref().unwrap().is_file() && rel_path.starts_with("{{") && rel_path.contains("}}")
        {
            debug!(
                "Adding template with relative path {:?} to tera instance.",
                rel_path
            );

            tera.add_template_file(file.as_ref().unwrap().to_str().unwrap(), Some(rel_path))
                .unwrap();
        }
    }

    // build our directory structure first
    let walker = WalkDir::new(src).into_iter();
    for entry in walker.filter_entry(|e| e.file_type().is_dir()) {
        let path_template = entry.unwrap().clone();
        let path_postfix = path_template.path();
        let path_template = path_postfix.strip_prefix(src).unwrap().to_str().unwrap();

        // we only render directories that start with a templated path, this is usually a single "root" directory that forms the top level directory of a project.
        if path_template.starts_with("{{") && path_template.contains("}}") {
            let real_path = Tera::one_off(path_template, &context, false).unwrap();

            if Path::new(real_path.as_str()).is_dir() & force.not() {
                error!(
                    "{} already exists. Will not proceed unless `--force`/force=True is used.",
                    real_path.as_str()
                )
            }
            if real_path.starts_with('.') {
                //skip any sort of top level dot files - extend with an exclusion glob in the future
                // todo: exclusion glob
                continue;
            }

            let destination = dst.join(Path::new(real_path.as_str()));
            let destination = destination.to_str().unwrap();
            debug!("Creating directory {:?}", destination);
            fs::create_dir(destination).unwrap();
            rendered_paths.push(destination.to_string());
        }
    }

    // render templates
    for template in tera.get_template_names() {
        if template == "angreal.toml" {
            // never render the angreal.toml
            // todo: exclusion glob
            continue;
        }

        if template.starts_with('.') {
            // we don't render dot files either
            // todo: exclusion glob
            continue;
        }

        let rendered = tera.render(template, &context).unwrap();
        let path = Tera::one_off(template, &context, false).unwrap();

        let destination = dst.join(Path::new(path.as_str()));
        let destination = destination.to_str().unwrap();
        debug!("Rendering file at {:?}", destination);
        let mut output = File::create(destination).unwrap();
        write!(output, "{}", rendered.as_str()).unwrap();
        rendered_paths.push(destination.to_string());
    }

    rendered_paths
}

pub fn check_up_to_date() -> Result<()> {
    let response_result = reqwest::blocking::get("https://pypi.org/pypi/angreal/json");

    let json = match response_result {
        Ok(response) => {
            let json_result = response.json::<serde_json::Value>();
            result_or_return_err!(json_result)
        }
        Err(e) => return Err(e).map_err(Into::into),
    };

    let upstream = value_or_return_err!(json["info"]["version"].as_str());
    let current = env!("CARGO_PKG_VERSION");
    let current = value_or_return_err!(Version::from(current));
    let upstream = value_or_return_err!(Version::from(upstream));

    if upstream > current {
        println!("A newer version of angreal is available, use pip install --upgrade angreal to upgrade.")
    };
    Ok(())
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
pub fn get_task_files(path: PathBuf) -> Result<Vec<PathBuf>> {
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
        Err(anyhow!("No tasks found for execution."))
    }
}

/// Registers the Command and Arg structs to the python api in the `angreal` module
pub fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_root, m)?)?;
    m.add_function(wrap_pyfunction!(render_template, m)?)?;
    m.add_function(wrap_pyfunction!(generate_context, m)?)?;
    m.add_function(wrap_pyfunction!(render_directory, m)?)?;
    Ok(())
}

#[pyfunction]
pub fn render_directory(
    src: &str,
    dst: &str,
    force: bool,
    context: Option<&PyDict>,
) -> PyResult<Py<PyAny>> {
    let mut ctx = Context::new();
    let src = Path::new(src);
    let dst = Path::new(dst);

    if let Some(context) = context {
        for key in context.keys() {
            if let Some(value) = context.get_item(key) {
                let v = value.to_string();
                let k = key.to_string();
                ctx.insert(&k, &v);
            }
        }
    }

    let x = render_dir(src, ctx, dst, force);
    Ok(pythonize_this!(x))
    // src: &Path, context: Context, dst: &Path, force: bool
}

#[pyfunction]
/// Generate a templating context from a toml file.
///
/// # Examples
/// ```python
/// import angreal
/// angreal_root = angreal.generate_context('path/to/angreal.toml',take_input=False)
/// ```
fn generate_context(path: &str, take_input: bool) -> PyResult<PyObject> {
    let toml_path = Path::new(path).to_path_buf();
    let ctx = repl_context_from_toml(toml_path, take_input);
    let map = context_to_map(ctx);
    Ok(pythonize_this!(map))
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
    let angreal_root =
        is_angreal_project().expect("Can't find the angreal_root from where you're executing.");
    Ok(String::from(angreal_root.to_string_lossy()))
}

#[pyfunction]
fn render_template(template: &str, context: &PyDict) -> PyResult<String> {
    let mut tera = Tera::default();
    let mut ctx = tera::Context::new();
    tera.add_raw_template("template", template).unwrap();

    for (key, val) in context.iter() {
        ctx.insert(&key.to_string(), &val.to_string());
    }

    Ok(tera.render("template", &ctx).unwrap())
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
pub fn is_angreal_project() -> Result<PathBuf> {
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
        Err(anyhow!("This doesn't appear to be an angreal project."))
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
    fn test_repl_context_from_toml() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_toml = root.join("tests/common/test_assets/test_template/angreal.toml");
        let ctx = crate::utils::repl_context_from_toml(test_toml, false);

        assert_eq!(ctx.get("key_1").unwrap(), "value_1");
        assert_eq!(ctx.get("key_2").unwrap(), 1);
        assert_eq!(ctx.get("folder_variable").unwrap(), "folder_name");
        assert_eq!(
            ctx.get("variable_text").unwrap(),
            "Just some text that we want to render"
        );
    }
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
