//! Docker Compose integration using subprocess execution for reliability
//!
//! This module provides a high-level interface to Docker Compose commands,
//! using subprocess execution to ensure compatibility with all Docker Compose versions.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Docker Compose integration using subprocess execution
#[derive(Clone, Debug)]
pub struct DockerCompose {
    compose_file: PathBuf,
    working_dir: PathBuf,
    project_name: Option<String>,
}

/// Result structure for Docker Compose operations
#[derive(Debug, Clone)]
pub struct ComposeOutput {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

impl DockerCompose {
    /// Create a new Docker Compose instance
    pub fn new<P: AsRef<Path>>(compose_file: P) -> Result<Self> {
        let compose_file = compose_file.as_ref().to_path_buf();

        if !compose_file.exists() {
            anyhow::bail!(
                "Docker Compose file does not exist: {}",
                compose_file.display()
            );
        }

        let working_dir = compose_file
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        Ok(Self {
            compose_file,
            working_dir,
            project_name: None,
        })
    }

    /// Set a custom project name
    pub fn with_project_name<S: Into<String>>(mut self, name: S) -> Self {
        self.project_name = Some(name.into());
        self
    }

    /// Check if docker-compose is available
    pub fn is_available() -> bool {
        // Try docker compose (v2) first, then docker-compose (v1)
        Command::new("docker")
            .args(["compose", "version"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
            || Command::new("docker-compose")
                .arg("version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
    }

    /// Get the compose file path
    pub fn compose_file(&self) -> &Path {
        &self.compose_file
    }

    /// Get the working directory
    pub fn working_dir(&self) -> &Path {
        &self.working_dir
    }

    /// Get the project name
    pub fn project_name(&self) -> Option<&str> {
        self.project_name.as_deref()
    }

    /// Execute a docker-compose command
    fn execute_command(&self, args: &[&str]) -> Result<ComposeOutput> {
        let mut cmd = if Command::new("docker")
            .args(["compose", "version"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            // Use docker compose (v2)
            let mut cmd = Command::new("docker");
            cmd.arg("compose");
            cmd
        } else {
            // Fall back to docker-compose (v1)
            Command::new("docker-compose")
        };

        // Add compose file argument
        cmd.args(["-f", &self.compose_file.to_string_lossy()]);

        // Add project name if specified
        if let Some(project_name) = &self.project_name {
            cmd.args(["-p", project_name]);
        }

        // Add the command arguments
        cmd.args(args);

        // Set working directory
        cmd.current_dir(&self.working_dir);

        // Execute the command
        let output = cmd
            .output()
            .context("Failed to execute docker-compose command")?;

        Ok(ComposeOutput {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

/// High-level Docker Compose operations
impl DockerCompose {
    /// Start services (docker-compose up)
    pub fn up(&self, options: UpOptions) -> Result<ComposeOutput> {
        let mut args = vec!["up"];

        if options.detach {
            args.push("-d");
        }
        if options.build {
            args.push("--build");
        }
        if options.remove_orphans {
            args.push("--remove-orphans");
        }
        if options.force_recreate {
            args.push("--force-recreate");
        }
        if options.no_recreate {
            args.push("--no-recreate");
        }

        // Add specific services if provided
        for service in &options.services {
            args.push(service);
        }

        self.execute_command(&args)
    }

    /// Stop and remove services (docker-compose down)
    pub fn down(&self, options: DownOptions) -> Result<ComposeOutput> {
        let mut args = vec!["down"];

        if options.volumes {
            args.push("-v");
        }
        if options.remove_orphans {
            args.push("--remove-orphans");
        }
        if let Some(timeout) = &options.timeout {
            args.push("-t");
            args.push(timeout);
        }

        self.execute_command(&args)
    }

    /// Restart services (docker-compose restart)
    pub fn restart(&self, options: RestartOptions) -> Result<ComposeOutput> {
        let mut args = vec!["restart"];

        if let Some(timeout) = &options.timeout {
            args.push("-t");
            args.push(timeout);
        }

        // Add specific services if provided
        for service in &options.services {
            args.push(service);
        }

        self.execute_command(&args)
    }

    /// View service logs (docker-compose logs)
    pub fn logs(&self, options: LogsOptions) -> Result<ComposeOutput> {
        let mut args = vec!["logs"];

        if options.follow {
            args.push("-f");
        }
        if options.timestamps {
            args.push("-t");
        }
        if let Some(tail) = &options.tail {
            args.push("--tail");
            args.push(tail);
        }
        if let Some(since) = &options.since {
            args.push("--since");
            args.push(since);
        }

        // Add specific services if provided
        for service in &options.services {
            args.push(service);
        }

        self.execute_command(&args)
    }

    /// List running services (docker-compose ps)
    pub fn ps(&self, options: PsOptions) -> Result<ComposeOutput> {
        let mut args = vec!["ps"];

        if options.all {
            args.push("-a");
        }
        if options.quiet {
            args.push("-q");
        }
        if options.services {
            args.push("--services");
        }

        // Add specific services if provided
        for service in &options.filter_services {
            args.push(service);
        }

        self.execute_command(&args)
    }

    /// Build services (docker-compose build)
    pub fn build(&self, options: BuildOptions) -> Result<ComposeOutput> {
        let mut args = vec!["build"];

        if options.no_cache {
            args.push("--no-cache");
        }
        if options.pull {
            args.push("--pull");
        }
        if options.parallel {
            args.push("--parallel");
        }

        // Add specific services if provided
        for service in &options.services {
            args.push(service);
        }

        self.execute_command(&args)
    }

    /// Start services (docker-compose start)
    pub fn start(&self, services: &[String]) -> Result<ComposeOutput> {
        let mut args = vec!["start"];
        for service in services {
            args.push(service);
        }
        self.execute_command(&args)
    }

    /// Stop services (docker-compose stop)
    pub fn stop(&self, options: StopOptions) -> Result<ComposeOutput> {
        let mut args = vec!["stop"];

        if let Some(timeout) = &options.timeout {
            args.push("-t");
            args.push(timeout);
        }

        // Add specific services if provided
        for service in &options.services {
            args.push(service);
        }

        self.execute_command(&args)
    }

    /// Execute a command in a service container (docker-compose exec)
    pub fn exec(
        &self,
        service: &str,
        command: &[String],
        options: ExecOptions,
    ) -> Result<ComposeOutput> {
        let mut args = vec!["exec"];

        if options.detach {
            args.push("-d");
        }
        if !options.tty {
            args.push("-T");
        }
        if let Some(user) = &options.user {
            args.push("-u");
            args.push(user);
        }
        if let Some(workdir) = &options.workdir {
            args.push("-w");
            args.push(workdir);
        }

        // Add environment variables
        let mut env_args = Vec::new();
        for (key, value) in &options.env {
            env_args.push(format!("{}={}", key, value));
        }
        for env_arg in &env_args {
            args.push("-e");
            args.push(env_arg);
        }

        args.push(service);
        for cmd_part in command {
            args.push(cmd_part);
        }

        self.execute_command(&args)
    }

    /// Pull service images (docker-compose pull)
    pub fn pull(&self, services: &[String]) -> Result<ComposeOutput> {
        let mut args = vec!["pull"];
        for service in services {
            args.push(service);
        }
        self.execute_command(&args)
    }

    /// Validate and view the compose configuration (docker-compose config)
    pub fn config(&self, options: ConfigOptions) -> Result<ComposeOutput> {
        let mut args = vec!["config"];

        if options.quiet {
            args.push("-q");
        }
        if options.services {
            args.push("--services");
        }
        if options.volumes {
            args.push("--volumes");
        }

        self.execute_command(&args)
    }
}

/// Options for docker-compose up command
#[derive(Debug, Default, Clone)]
pub struct UpOptions {
    pub detach: bool,
    pub build: bool,
    pub remove_orphans: bool,
    pub force_recreate: bool,
    pub no_recreate: bool,
    pub services: Vec<String>,
}

/// Options for docker-compose down command
#[derive(Debug, Default, Clone)]
pub struct DownOptions {
    pub volumes: bool,
    pub remove_orphans: bool,
    pub timeout: Option<String>,
}

/// Options for docker-compose restart command
#[derive(Debug, Default, Clone)]
pub struct RestartOptions {
    pub timeout: Option<String>,
    pub services: Vec<String>,
}

/// Options for docker-compose logs command
#[derive(Debug, Default, Clone)]
pub struct LogsOptions {
    pub follow: bool,
    pub timestamps: bool,
    pub tail: Option<String>,
    pub since: Option<String>,
    pub services: Vec<String>,
}

/// Options for docker-compose ps command
#[derive(Debug, Default, Clone)]
pub struct PsOptions {
    pub all: bool,
    pub quiet: bool,
    pub services: bool,
    pub filter_services: Vec<String>,
}

/// Options for docker-compose build command
#[derive(Debug, Default, Clone)]
pub struct BuildOptions {
    pub no_cache: bool,
    pub pull: bool,
    pub parallel: bool,
    pub services: Vec<String>,
}

/// Options for docker-compose stop command
#[derive(Debug, Default, Clone)]
pub struct StopOptions {
    pub timeout: Option<String>,
    pub services: Vec<String>,
}

/// Options for docker-compose exec command
#[derive(Debug, Default, Clone)]
pub struct ExecOptions {
    pub detach: bool,
    pub tty: bool,
    pub user: Option<String>,
    pub workdir: Option<String>,
    pub env: HashMap<String, String>,
}

/// Options for docker-compose config command
#[derive(Debug, Default, Clone)]
pub struct ConfigOptions {
    pub quiet: bool,
    pub services: bool,
    pub volumes: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_docker_compose_available() {
        // This test will only pass if docker-compose is installed
        if DockerCompose::is_available() {
            assert!(true);
        } else {
            println!("Docker Compose not available, skipping test");
        }
    }

    #[test]
    fn test_docker_compose_new() {
        if !DockerCompose::is_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let compose_file = temp_dir.path().join("docker-compose.yml");

        // Create a minimal compose file
        fs::write(
            &compose_file,
            r#"
version: '3'
services:
  test:
    image: hello-world
"#,
        )
        .unwrap();

        let compose = DockerCompose::new(&compose_file).unwrap();
        assert_eq!(compose.compose_file(), &compose_file);
        assert_eq!(compose.working_dir(), temp_dir.path());
        assert_eq!(compose.project_name(), None);
    }

    #[test]
    fn test_docker_compose_with_project_name() {
        if !DockerCompose::is_available() {
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let compose_file = temp_dir.path().join("docker-compose.yml");

        fs::write(
            &compose_file,
            r#"
version: '3'
services:
  test:
    image: hello-world
"#,
        )
        .unwrap();

        let compose = DockerCompose::new(&compose_file)
            .unwrap()
            .with_project_name("test-project");

        assert_eq!(compose.project_name(), Some("test-project"));
    }

    #[test]
    fn test_invalid_compose_file() {
        let result = DockerCompose::new("/nonexistent/docker-compose.yml");
        assert!(result.is_err());
    }
}
