//! OCI registry integration bindings.
//!
//! Exposes a minimal `angreal.integrations.oci.Oci` surface — pull, push, tags,
//! login — over the shared OCI core (`crate::integrations::oci`). All heavy
//! lifting (registry I/O, auth, the template artifact convention) lives in the
//! core so this binding stays a thin adapter and matches the `init oci://…` path.

#![allow(non_local_definitions)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use pyo3::prelude::*;

use crate::integrations::oci::{self as core, Oci, OciAuth};

fn to_pyerr(e: anyhow::Error) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
}

/// OCI registry client for angreal template artifacts.
///
/// Pull and push templates, list tags, and authenticate against registries.
/// Credentials set via `login()` are held for the lifetime of the instance and
/// take precedence over Docker `config.json` credentials for that registry.
#[pyclass(name = "Oci")]
pub struct PyOci {
    /// Per-registry credentials registered via `login()`.
    credentials: Mutex<HashMap<String, OciAuth>>,
}

impl PyOci {
    /// Resolve auth for a reference: a `login()` credential for its registry if
    /// present, otherwise the core default (Docker config → anonymous).
    fn auth_for(&self, reference: &str) -> OciAuth {
        if let Ok(registry) = core::reference_registry(reference) {
            if let Some(auth) = self
                .credentials
                .lock()
                .expect("oci credentials mutex poisoned")
                .get(&registry)
                .cloned()
            {
                return auth;
            }
        }
        core::resolve_auth(reference)
    }
}

#[pymethods]
impl PyOci {
    #[new]
    fn new() -> Self {
        Self {
            credentials: Mutex::new(HashMap::new()),
        }
    }

    /// Store Basic credentials for `registry` (e.g. `ghcr.io`), used by
    /// subsequent pull/push/tags calls against that registry.
    fn login(&self, registry: &str, username: String, password: String) -> PyResult<()> {
        self.credentials
            .lock()
            .expect("oci credentials mutex poisoned")
            .insert(registry.to_string(), OciAuth::Basic { username, password });
        Ok(())
    }

    /// Pull a template artifact to `dest` (default: a directory named after the
    /// repository, under the current directory). Returns the destination path.
    #[pyo3(signature = (reference, dest=None))]
    fn pull(&self, reference: &str, dest: Option<PathBuf>) -> PyResult<String> {
        let dest = match dest {
            Some(d) => d,
            None => {
                let name = core::reference_basename(reference).map_err(to_pyerr)?;
                std::env::current_dir()?.join(name)
            }
        };
        let auth = self.auth_for(reference);
        let out = Oci::pull_artifact(reference, &dest, &auth).map_err(to_pyerr)?;
        Ok(out.display().to_string())
    }

    /// Package the directory at `path` as a template artifact and push it to
    /// `reference`.
    fn push(&self, reference: &str, path: PathBuf) -> PyResult<()> {
        let auth = self.auth_for(reference);
        Oci::push_artifact(reference, &path, &auth).map_err(to_pyerr)
    }

    /// List the tags available for `repository`.
    fn tags(&self, repository: &str) -> PyResult<Vec<String>> {
        let auth = self.auth_for(repository);
        Oci::list_tags(repository, &auth).map_err(to_pyerr)
    }
}

/// OCI integration module, exposed as `angreal.integrations.oci` in Python.
#[pymodule]
pub fn oci(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyOci>()?;
    Ok(())
}
