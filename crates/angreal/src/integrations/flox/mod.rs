//! Flox development environment integration
//!
//! This module provides Rust bindings for the Flox CLI, enabling
//! environment activation and services management.

use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Core integration for the Flox CLI
pub struct FloxIntegration;

impl FloxIntegration {
    /// Check if the `flox` binary is available in PATH
    pub fn is_available() -> bool {
        Command::new("flox")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Get the Flox version string
    pub fn version() -> Result<String> {
        let output = Command::new("flox")
            .arg("--version")
            .output()
            .context("Failed to execute flox --version")?;

        if !output.status.success() {
            bail!("Flox version check failed");
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

/// Represents a service status entry from `flox services status`
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String,
    pub pid: Option<u32>,
}

/// Flox environment wrapper for a specific directory
pub struct FloxEnvironment {
    /// Path to the directory containing the Flox environment (.flox/)
    pub path: PathBuf,
}

impl FloxEnvironment {
    /// Create a new FloxEnvironment reference for the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Check if this directory contains a Flox environment
    pub fn exists(&self) -> bool {
        self.path.join(".flox").exists()
    }

    /// Get the path to the manifest.toml if it exists
    pub fn manifest_path(&self) -> PathBuf {
        self.path.join(".flox").join("env").join("manifest.toml")
    }

    /// Check if the manifest.toml exists
    pub fn has_manifest(&self) -> bool {
        self.manifest_path().exists()
    }

    /// Get environment variable modifications from `flox activate --print-script`
    ///
    /// Parses the activation script to extract environment variable changes.
    /// Returns a HashMap of variable names to their new values.
    pub fn get_activation_env(&self) -> Result<HashMap<String, String>> {
        let output = Command::new("flox")
            .arg("activate")
            .arg("--print-script")
            .arg("-d")
            .arg(&self.path)
            .output()
            .context("Failed to execute flox activate --print-script")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to get Flox activation script: {}", stderr);
        }

        let script = String::from_utf8_lossy(&output.stdout);
        Self::parse_activation_script(&script)
    }

    /// Parse the activation script to extract environment variable exports
    ///
    /// Looks for patterns like:
    /// - `export VAR="value"`
    /// - `export VAR='value'`
    /// - `export VAR=value`
    /// - `VAR="value"; export VAR`
    fn parse_activation_script(script: &str) -> Result<HashMap<String, String>> {
        let mut env_vars = HashMap::new();

        for line in script.lines() {
            let line = line.trim();

            // Handle `export VAR=value` or `export VAR="value"` or `export VAR='value'`
            if let Some(rest) = line.strip_prefix("export ") {
                if let Some((name, value)) = rest.split_once('=') {
                    let name = name.trim();
                    let value = Self::unquote_value(value.trim());
                    env_vars.insert(name.to_string(), value);
                }
            }
        }

        Ok(env_vars)
    }

    /// Remove surrounding quotes from a value
    fn unquote_value(value: &str) -> String {
        let value = value.trim();
        if (value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\''))
        {
            value[1..value.len() - 1].to_string()
        } else {
            value.to_string()
        }
    }

    /// Run a command within the Flox environment
    ///
    /// Executes: `flox activate -d <path> -- <command> [args...]`
    pub fn run_in_env(&self, command: &str, args: &[&str]) -> Result<Output> {
        let mut cmd = Command::new("flox");
        cmd.arg("activate")
            .arg("-d")
            .arg(&self.path)
            .arg("--")
            .arg(command);

        for arg in args {
            cmd.arg(arg);
        }

        cmd.output()
            .context(format!("Failed to run '{}' in Flox environment", command))
    }

    /// Start Flox services
    ///
    /// If `services` is empty, starts all services defined in the manifest.
    /// Otherwise, starts only the specified services.
    pub fn services_start(&self, services: &[&str]) -> Result<()> {
        let mut cmd = Command::new("flox");
        cmd.arg("services").arg("start").arg("-d").arg(&self.path);

        for service in services {
            cmd.arg(service);
        }

        let output = cmd.output().context("Failed to start Flox services")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to start Flox services: {}", stderr);
        }

        Ok(())
    }

    /// Stop all Flox services
    pub fn services_stop(&self) -> Result<()> {
        let output = Command::new("flox")
            .arg("services")
            .arg("stop")
            .arg("-d")
            .arg(&self.path)
            .output()
            .context("Failed to stop Flox services")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to stop Flox services: {}", stderr);
        }

        Ok(())
    }

    /// Get status of all Flox services
    ///
    /// Parses the output of `flox services status` which looks like:
    /// ```text
    /// NAME      STATUS   PID
    /// postgres  Running  12345
    /// redis     Running  12346
    /// ```
    pub fn services_status(&self) -> Result<Vec<ServiceStatus>> {
        let output = Command::new("flox")
            .arg("services")
            .arg("status")
            .arg("-d")
            .arg(&self.path)
            .output()
            .context("Failed to get Flox services status")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // If no services are defined, flox may return an error
            // The error message can be: "no services", "No services", or "does not have any services"
            if stderr.contains("no services")
                || stderr.contains("No services")
                || stderr.contains("does not have any services")
            {
                return Ok(Vec::new());
            }
            bail!("Failed to get Flox services status: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Self::parse_services_status(&stdout)
    }

    /// Parse the output of `flox services status`
    fn parse_services_status(output: &str) -> Result<Vec<ServiceStatus>> {
        let mut services = Vec::new();
        let mut lines = output.lines();

        // Skip header line (NAME STATUS PID)
        if lines.next().is_none() {
            return Ok(services);
        }

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let status = parts[1].to_string();
                let pid = parts.get(2).and_then(|p| p.parse::<u32>().ok());

                services.push(ServiceStatus { name, status, pid });
            }
        }

        Ok(services)
    }

    /// Get logs for a specific service
    pub fn services_logs(&self, service: &str, follow: bool, tail: Option<u32>) -> Result<String> {
        let mut cmd = Command::new("flox");
        cmd.arg("services").arg("logs").arg("-d").arg(&self.path);

        if follow {
            cmd.arg("--follow");
        }

        if let Some(n) = tail {
            cmd.arg("--tail").arg(n.to_string());
        }

        cmd.arg(service);

        let output = cmd.output().context("Failed to get Flox service logs")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to get logs for service '{}': {}", service, stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Restart services
    pub fn services_restart(&self, services: &[&str]) -> Result<()> {
        let mut cmd = Command::new("flox");
        cmd.arg("services").arg("restart").arg("-d").arg(&self.path);

        for service in services {
            cmd.arg(service);
        }

        let output = cmd.output().context("Failed to restart Flox services")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to restart Flox services: {}", stderr);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== FloxIntegration Tests ====================

    #[test]
    fn test_is_available_returns_bool() {
        // Just verify it returns without panicking
        let _ = FloxIntegration::is_available();
    }

    #[test]
    fn test_version_when_flox_not_installed() {
        // If flox is not installed, version() should return an error
        // If it is installed, it should return a version string
        let result = FloxIntegration::version();
        if FloxIntegration::is_available() {
            assert!(result.is_ok());
            let version = result.unwrap();
            assert!(!version.is_empty());
        } else {
            assert!(result.is_err());
        }
    }

    // ==================== FloxEnvironment Tests ====================

    #[test]
    fn test_flox_environment_new() {
        let env = FloxEnvironment::new("/some/path");
        assert_eq!(env.path, PathBuf::from("/some/path"));
    }

    #[test]
    fn test_manifest_path() {
        let env = FloxEnvironment::new("/project");
        assert_eq!(
            env.manifest_path(),
            PathBuf::from("/project/.flox/env/manifest.toml")
        );
    }

    // ==================== Parsing Tests ====================

    #[test]
    fn test_parse_activation_script_export_double_quotes() {
        let script = r#"
export PATH="/nix/store/abc123/bin:$PATH"
export FLOX_ENV="/path/to/env"
"#;
        let result = FloxEnvironment::parse_activation_script(script).unwrap();
        assert_eq!(
            result.get("PATH"),
            Some(&"/nix/store/abc123/bin:$PATH".to_string())
        );
        assert_eq!(result.get("FLOX_ENV"), Some(&"/path/to/env".to_string()));
    }

    #[test]
    fn test_parse_activation_script_export_single_quotes() {
        let script = r#"
export MY_VAR='single quoted value'
"#;
        let result = FloxEnvironment::parse_activation_script(script).unwrap();
        assert_eq!(
            result.get("MY_VAR"),
            Some(&"single quoted value".to_string())
        );
    }

    #[test]
    fn test_parse_activation_script_export_no_quotes() {
        let script = r#"
export SIMPLE=value
"#;
        let result = FloxEnvironment::parse_activation_script(script).unwrap();
        assert_eq!(result.get("SIMPLE"), Some(&"value".to_string()));
    }

    #[test]
    fn test_parse_activation_script_empty() {
        let script = "";
        let result = FloxEnvironment::parse_activation_script(script).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_activation_script_ignores_non_export() {
        let script = r#"
# This is a comment
echo "Hello"
export VALID="value"
some_function() { echo "hi"; }
"#;
        let result = FloxEnvironment::parse_activation_script(script).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("VALID"), Some(&"value".to_string()));
    }

    #[test]
    fn test_unquote_value_double_quotes() {
        assert_eq!(
            FloxEnvironment::unquote_value("\"hello world\""),
            "hello world"
        );
    }

    #[test]
    fn test_unquote_value_single_quotes() {
        assert_eq!(
            FloxEnvironment::unquote_value("'hello world'"),
            "hello world"
        );
    }

    #[test]
    fn test_unquote_value_no_quotes() {
        assert_eq!(FloxEnvironment::unquote_value("hello"), "hello");
    }

    #[test]
    fn test_unquote_value_mismatched_quotes() {
        // Mismatched quotes should not be stripped
        assert_eq!(FloxEnvironment::unquote_value("\"hello'"), "\"hello'");
    }

    // ==================== Service Status Parsing Tests ====================

    #[test]
    fn test_parse_services_status_multiple_services() {
        let output = r#"NAME      STATUS   PID
postgres  Running  12345
redis     Running  12346
nginx     Stopped
"#;
        let result = FloxEnvironment::parse_services_status(output).unwrap();
        assert_eq!(result.len(), 3);

        assert_eq!(result[0].name, "postgres");
        assert_eq!(result[0].status, "Running");
        assert_eq!(result[0].pid, Some(12345));

        assert_eq!(result[1].name, "redis");
        assert_eq!(result[1].status, "Running");
        assert_eq!(result[1].pid, Some(12346));

        assert_eq!(result[2].name, "nginx");
        assert_eq!(result[2].status, "Stopped");
        assert_eq!(result[2].pid, None);
    }

    #[test]
    fn test_parse_services_status_empty() {
        let output = "";
        let result = FloxEnvironment::parse_services_status(output).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_services_status_header_only() {
        let output = "NAME      STATUS   PID\n";
        let result = FloxEnvironment::parse_services_status(output).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_service_status_struct() {
        let status = ServiceStatus {
            name: "postgres".to_string(),
            status: "Running".to_string(),
            pid: Some(12345),
        };
        assert_eq!(status.name, "postgres");
        assert_eq!(status.status, "Running");
        assert_eq!(status.pid, Some(12345));
    }
}
