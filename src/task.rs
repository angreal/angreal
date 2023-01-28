use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

pub fn register(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<AngrealCommand>()?;
    m.add_class::<AngrealArg>()?;
    Ok(())
}

pub static ANGREAL_TASKS: Lazy<Mutex<Vec<AngrealCommand>>> = Lazy::new(|| Mutex::new(vec![]));

pub static ANGREAL_ARGS: Lazy<Mutex<Vec<AngrealArg>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(Clone, Debug)]
#[pyclass(name = "Command")]
pub struct AngrealCommand {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub about: Option<String>,
    #[pyo3(get)]
    pub long_about: Option<String>,
    #[pyo3(get)]
    pub func: Py<PyAny>,
}

#[pymethods]
impl AngrealCommand {
    #[new]
    #[args(about = "None", long_about = "None")]
    fn __new__(name: &str, func: Py<PyAny>, about: Option<&str>, long_about: Option<&str>) -> Self {
        let cmd = AngrealCommand {
            name: name.to_string(),
            about: about.map(|i| i.to_string()),
            long_about: long_about.map(|i| i.to_string()),
            func,
        };
        ANGREAL_TASKS.lock().unwrap().push(cmd.clone());
        cmd
    }
}

#[derive(Clone, Debug)]
#[pyclass(name = "Arg")]
pub struct AngrealArg {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub command_name: String,
    #[pyo3(get)]
    pub takes_value: Option<bool>,
    #[pyo3(get)]
    pub default_value: Option<String>,
    #[pyo3(get)]
    pub require_equals: Option<bool>,
    #[pyo3(get)]
    pub multiple_values: Option<bool>,
    #[pyo3(get)]
    pub number_of_values: Option<u32>,
    #[pyo3(get)]
    pub max_values: Option<u32>,
    #[pyo3(get)]
    pub min_values: Option<u32>,
    #[pyo3(get)]
    pub python_type: Option<String>,
    #[pyo3(get)]
    pub short: Option<char>,
    #[pyo3(get)]
    pub long: Option<String>,
    #[pyo3(get)]
    pub long_help: Option<String>,
    #[pyo3(get)]
    pub help: Option<String>,
    #[pyo3(get)]
    pub required: Option<bool>,
}

#[pymethods]
impl AngrealArg {
    #[new]
    #[args(
        python_type = "\"str\"",
        takes_value = "true",
        default_value = "None",
        require_equals = "None",
        multiple_values = "None",
        number_of_values = "None",
        max_values = "None",
        min_values = "None",
        short = "None",
        long = "None",
        long_help = "None",
        help = "None",
        required = "None"
    )]
    #[allow(clippy::too_many_arguments)]
    fn __new__(
        name: &str,
        command_name: &str,
        takes_value: Option<bool>,
        default_value: Option<&str>,
        require_equals: Option<bool>,
        multiple_values: Option<bool>,
        number_of_values: Option<u32>,
        max_values: Option<u32>,
        min_values: Option<u32>,
        python_type: Option<&str>,
        short: Option<char>,
        long: Option<&str>,
        long_help: Option<&str>,
        help: Option<&str>,
        required: Option<bool>,
    ) -> Self {
        let arg = AngrealArg {
            name: name.to_string(),
            command_name: command_name.to_string(),
            takes_value,
            default_value: default_value.map(|i| i.to_string()),
            require_equals,
            multiple_values,
            number_of_values,
            max_values,
            min_values,
            python_type: python_type.map(|i| i.to_string()),
            short,
            long: long.map(|i| i.to_string()),
            long_help: long_help.map(|i| i.to_string()),
            help: help.map(|i| i.to_string()),
            required,
        };
        ANGREAL_ARGS.lock().unwrap().push(arg.clone());
        arg
    }
}
