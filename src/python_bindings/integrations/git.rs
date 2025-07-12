//! Git integration bindings

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::path::Path;

use crate::integrations::git::Git;

#[pyclass(extends=pyo3::exceptions::PyException)]
pub struct GitException {
    message: String,
}

#[pymethods]
impl GitException {
    #[new]
    fn new(message: String) -> Self {
        Self { message }
    }
    
    fn __str__(&self) -> String {
        self.message.clone()
    }
}

#[pyclass(name = "Git")]
pub struct PyGit {
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

    fn execute(&self, subcommand: &str, args: Vec<&str>) -> PyResult<(i32, String, String)> {
        let output = self
            .inner
            .execute(subcommand, &args)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok((output.exit_code, output.stderr, output.stdout))
    }

    fn init(&self, bare: Option<bool>) -> PyResult<()> {
        self.inner
            .init(bare.unwrap_or(false))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn add(&self, paths: Vec<&str>) -> PyResult<()> {
        self.inner
            .add(&paths)
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
        args: Vec<&str>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<(i32, String, String)> {
        let output = if let Some(dict) = kwargs {
            // Convert PyDict to HashMap<&str, &str>
            let mut options = HashMap::new();
            for (key, value) in dict.iter() {
                let key_str = key.extract::<&str>()?;
                let value_str = if value.is_true()? {
                    "" // For boolean flags like --bare
                } else {
                    value.extract::<&str>()?
                };
                options.insert(key_str, value_str);
            }
            self.inner.execute_with_options(command, options, &args)
        } else {
            self.inner.execute(command, &args)
        }
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        Ok((output.exit_code, output.stderr, output.stdout))
    }
    
    fn __getattr__(&self, py: Python, name: &str) -> PyResult<PyObject> {
        // For any unknown method, raise GitException
        Err(PyErr::new::<GitException, _>(format!("Git command '{}' not found", name)))
    }
}

#[pyfunction]
#[pyo3(signature = (remote, destination=None))]
pub fn git_clone(remote: &str, destination: Option<&str>) -> PyResult<String> {
    let dest = Git::clone(remote, destination.map(Path::new))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(dest.display().to_string())
}

/// Git integration module
/// 
/// This will be exposed as angreal.integrations.git in Python
#[pymodule]
pub fn git_integration(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GitException>()?;
    m.add_class::<PyGit>()?;
    m.add_function(wrap_pyfunction!(git_clone, m)?)?;
    Ok(())
}