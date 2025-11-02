//! PyO3 bindings for Docker Compose integration

use crate::integrations::docker_compose::{
    BuildOptions, ComposeOutput, ConfigOptions, DockerCompose as RustDockerCompose, DownOptions,
    ExecOptions, LogsOptions, PsOptions, RestartOptions, StopOptions, UpOptions,
};
use pyo3::exceptions::{PyIOError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Python wrapper for Docker Compose operations
#[pyclass(name = "DockerCompose")]
pub struct PyDockerCompose {
    inner: RustDockerCompose,
}

/// Python wrapper for compose command output
#[pyclass(name = "ComposeResult")]
#[derive(Clone)]
pub struct PyComposeOutput {
    #[pyo3(get)]
    pub success: bool,
    #[pyo3(get)]
    pub exit_code: i32,
    #[pyo3(get)]
    pub stdout: String,
    #[pyo3(get)]
    pub stderr: String,
}

impl From<ComposeOutput> for PyComposeOutput {
    fn from(output: ComposeOutput) -> Self {
        Self {
            success: output.success,
            exit_code: output.exit_code,
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}

#[pymethods]
impl PyDockerCompose {
    /// Create a new DockerCompose instance
    #[new]
    #[pyo3(signature = (file, project_name=None))]
    fn new(file: &str, project_name: Option<&str>) -> PyResult<Self> {
        let mut compose = RustDockerCompose::new(file).map_err(|e| {
            PyIOError::new_err(format!("Failed to create Docker Compose instance: {}", e))
        })?;

        if let Some(name) = project_name {
            compose = compose.with_project_name(name);
        }

        Ok(Self { inner: compose })
    }

    /// Check if Docker Compose is available
    #[staticmethod]
    fn is_available() -> bool {
        RustDockerCompose::is_available()
    }

    /// Get the compose file path
    #[getter]
    fn compose_file(&self) -> String {
        self.inner.compose_file().display().to_string()
    }

    /// Get the working directory
    #[getter]
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }

    /// Get the project name
    #[getter]
    fn project_name(&self) -> Option<String> {
        self.inner.project_name().map(|s| s.to_string())
    }

    /// Start services (docker-compose up)
    #[pyo3(signature = (detach=true, build=false, remove_orphans=false, force_recreate=false, no_recreate=false, services=None))]
    fn up(
        &self,
        detach: bool,
        build: bool,
        remove_orphans: bool,
        force_recreate: bool,
        no_recreate: bool,
        services: Option<Vec<String>>,
    ) -> PyResult<PyComposeOutput> {
        let options = UpOptions {
            detach,
            build,
            remove_orphans,
            force_recreate,
            no_recreate,
            services: services.unwrap_or_default(),
        };

        let result = self
            .inner
            .up(options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose up failed: {}", e)))?;

        Ok(result.into())
    }

    /// Stop and remove services (docker-compose down)
    #[pyo3(signature = (volumes=false, remove_orphans=false, timeout=None))]
    fn down(
        &self,
        volumes: bool,
        remove_orphans: bool,
        timeout: Option<String>,
    ) -> PyResult<PyComposeOutput> {
        let options = DownOptions {
            volumes,
            remove_orphans,
            timeout,
        };

        let result = self
            .inner
            .down(options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose down failed: {}", e)))?;

        Ok(result.into())
    }

    /// Restart services (docker-compose restart)
    #[pyo3(signature = (services=None, timeout=None))]
    fn restart(
        &self,
        services: Option<Vec<String>>,
        timeout: Option<String>,
    ) -> PyResult<PyComposeOutput> {
        let options = RestartOptions {
            timeout,
            services: services.unwrap_or_default(),
        };

        let result = self.inner.restart(options).map_err(|e| {
            PyRuntimeError::new_err(format!("Docker Compose restart failed: {}", e))
        })?;

        Ok(result.into())
    }

    /// View service logs (docker-compose logs)
    #[pyo3(signature = (services=None, follow=false, timestamps=false, tail=None, since=None))]
    fn logs(
        &self,
        services: Option<Vec<String>>,
        follow: bool,
        timestamps: bool,
        tail: Option<String>,
        since: Option<String>,
    ) -> PyResult<PyComposeOutput> {
        let options = LogsOptions {
            follow,
            timestamps,
            tail,
            since,
            services: services.unwrap_or_default(),
        };

        let result = self
            .inner
            .logs(options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose logs failed: {}", e)))?;

        Ok(result.into())
    }

    /// List running services (docker-compose ps)
    #[pyo3(signature = (all=false, quiet=false, services=false, filter_services=None))]
    fn ps(
        &self,
        all: bool,
        quiet: bool,
        services: bool,
        filter_services: Option<Vec<String>>,
    ) -> PyResult<PyComposeOutput> {
        let options = PsOptions {
            all,
            quiet,
            services,
            filter_services: filter_services.unwrap_or_default(),
        };

        let result = self
            .inner
            .ps(options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose ps failed: {}", e)))?;

        Ok(result.into())
    }

    /// Build services (docker-compose build)
    #[pyo3(signature = (services=None, no_cache=false, pull=false, parallel=false))]
    fn build(
        &self,
        services: Option<Vec<String>>,
        no_cache: bool,
        pull: bool,
        parallel: bool,
    ) -> PyResult<PyComposeOutput> {
        let options = BuildOptions {
            no_cache,
            pull,
            parallel,
            services: services.unwrap_or_default(),
        };

        let result = self
            .inner
            .build(options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose build failed: {}", e)))?;

        Ok(result.into())
    }

    /// Start services (docker-compose start)
    #[pyo3(signature = (services=None))]
    fn start(&self, services: Option<Vec<String>>) -> PyResult<PyComposeOutput> {
        let services = services.unwrap_or_default();
        let result = self
            .inner
            .start(&services)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose start failed: {}", e)))?;

        Ok(result.into())
    }

    /// Stop services (docker-compose stop)
    #[pyo3(signature = (services=None, timeout=None))]
    fn stop(
        &self,
        services: Option<Vec<String>>,
        timeout: Option<String>,
    ) -> PyResult<PyComposeOutput> {
        let options = StopOptions {
            timeout,
            services: services.unwrap_or_default(),
        };

        let result = self
            .inner
            .stop(options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose stop failed: {}", e)))?;

        Ok(result.into())
    }

    /// Execute a command in a service container (docker-compose exec)
    #[pyo3(signature = (service, command, detach=false, tty=true, user=None, workdir=None, env=None))]
    #[allow(clippy::too_many_arguments)]
    fn exec(
        &self,
        service: &str,
        command: Vec<String>,
        detach: bool,
        tty: bool,
        user: Option<String>,
        workdir: Option<String>,
        env: Option<HashMap<String, String>>,
    ) -> PyResult<PyComposeOutput> {
        if command.is_empty() {
            return Err(PyValueError::new_err("Command cannot be empty"));
        }

        let options = ExecOptions {
            detach,
            tty,
            user,
            workdir,
            env: env.unwrap_or_default(),
        };

        let result = self
            .inner
            .exec(service, &command, options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose exec failed: {}", e)))?;

        Ok(result.into())
    }

    /// Pull service images (docker-compose pull)
    #[pyo3(signature = (services=None))]
    fn pull(&self, services: Option<Vec<String>>) -> PyResult<PyComposeOutput> {
        let services = services.unwrap_or_default();
        let result = self
            .inner
            .pull(&services)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose pull failed: {}", e)))?;

        Ok(result.into())
    }

    /// Validate and view the compose configuration (docker-compose config)
    #[pyo3(signature = (quiet=false, services=false, volumes=false))]
    fn config(&self, quiet: bool, services: bool, volumes: bool) -> PyResult<PyComposeOutput> {
        let options = ConfigOptions {
            quiet,
            services,
            volumes,
        };

        let result = self
            .inner
            .config(options)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose config failed: {}", e)))?;

        Ok(result.into())
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "DockerCompose(file='{}', project_name={:?})",
            self.inner.compose_file().display(),
            self.inner.project_name()
        )
    }
}

/// Convenience function to create a DockerCompose instance
#[pyfunction]
#[pyo3(signature = (file, project_name=None))]
pub fn compose(file: &str, project_name: Option<&str>) -> PyResult<PyDockerCompose> {
    PyDockerCompose::new(file, project_name)
}

/// Compose integration module
#[pymodule]
pub fn compose_integration(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDockerCompose>()?;
    m.add_class::<PyComposeOutput>()?;
    m.add_function(wrap_pyfunction!(compose, m)?)?;
    Ok(())
}
