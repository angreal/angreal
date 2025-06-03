use anyhow::{bail, Context, Result};
use pyo3::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct UvIntegration;

impl UvIntegration {
    pub fn is_available() -> bool {
        Command::new("uv")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn version() -> Result<String> {
        let output = Command::new("uv")
            .arg("--version")
            .output()
            .context("Failed to get UV version")?;

        if !output.status.success() {
            bail!("UV version check failed");
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn ensure_installed() -> Result<()> {
        if Self::is_available() {
            return Ok(());
        }

        println!("Installing UV binary...");

        #[cfg(unix)]
        {
            let status = Command::new("sh")
                .arg("-c")
                .arg("curl -LsSf https://astral.sh/uv/install.sh | sh")
                .status()
                .context("Failed to install UV")?;

            if !status.success() {
                bail!("UV installation failed");
            }
        }

        #[cfg(windows)]
        {
            let status = Command::new("powershell")
                .arg("-c")
                .arg("irm https://astral.sh/uv/install.ps1 | iex")
                .status()
                .context("Failed to install UV")?;

            if !status.success() {
                bail!("UV installation failed");
            }
        }

        if !Self::is_available() {
            bail!("UV installation completed but binary not found in PATH");
        }

        println!("UV successfully installed: {}", Self::version()?);
        Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ActivationInfo {
    #[pyo3(get)]
    pub venv_path: String,
    #[pyo3(get)]
    pub venv_prefix: String,
    #[pyo3(get)]
    pub site_packages: String,
    #[pyo3(get)]
    pub python_executable: String,
}

pub struct UvVirtualEnv {
    pub path: PathBuf,
}

impl UvVirtualEnv {
    pub fn create(path: &Path, python_version: Option<&str>) -> Result<Self> {
        UvIntegration::ensure_installed()?;

        let mut cmd = Command::new("uv");
        cmd.arg("venv").arg(path);

        if let Some(version) = python_version {
            cmd.arg("--python").arg(version);
        }

        let output = cmd.output().context("Failed to execute UV")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to create virtual environment: {}", stderr);
        }

        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn install_packages(&self, packages: &[String]) -> Result<()> {
        if packages.is_empty() {
            return Ok(());
        }

        let python_path = self.python_executable();

        let mut cmd = Command::new("uv");
        cmd.arg("pip")
            .arg("install")
            .arg("--python")
            .arg(&python_path);

        for package in packages {
            cmd.arg(package);
        }

        let output = cmd.output().context("Failed to execute UV pip install")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to install packages: {}", stderr);
        }

        Ok(())
    }

    pub fn install_requirements(&self, requirements_file: &Path) -> Result<()> {
        let python_path = self.python_executable();

        let output = Command::new("uv")
            .arg("pip")
            .arg("install")
            .arg("--python")
            .arg(&python_path)
            .arg("-r")
            .arg(requirements_file)
            .output()
            .context("Failed to execute UV pip install")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to install requirements: {}", stderr);
        }

        Ok(())
    }

    pub fn python_executable(&self) -> PathBuf {
        if cfg!(windows) {
            self.path.join("Scripts").join("python.exe")
        } else {
            self.path.join("bin").join("python")
        }
    }

    pub fn site_packages(&self) -> Result<PathBuf> {
        let python_exe = self.python_executable();

        let output = Command::new(&python_exe)
            .arg("-c")
            .arg("import site; print(site.getsitepackages()[0])")
            .output()
            .context("Failed to get site-packages path")?;

        if !output.status.success() {
            bail!("Failed to determine site-packages location");
        }

        let site_packages_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(PathBuf::from(site_packages_str))
    }

    pub fn get_activation_info(&self) -> Result<ActivationInfo> {
        let python_exe = self.python_executable();
        let site_packages = self.site_packages()?;

        // Get the virtual environment's sys.prefix
        let output = Command::new(&python_exe)
            .arg("-c")
            .arg("import sys; print(sys.prefix)")
            .output()
            .context("Failed to get virtual environment prefix")?;

        if !output.status.success() {
            bail!("Failed to get virtual environment prefix");
        }

        let venv_prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(ActivationInfo {
            venv_path: self.path.to_string_lossy().to_string(),
            venv_prefix,
            site_packages: site_packages.to_string_lossy().to_string(),
            python_executable: python_exe.to_string_lossy().to_string(),
        })
    }

    pub fn discover_pythons() -> Result<Vec<(String, PathBuf)>> {
        UvIntegration::ensure_installed()?;

        let output = Command::new("uv")
            .arg("python")
            .arg("list")
            .output()
            .context("Failed to list Python installations")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to discover Python installations: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pythons = Vec::new();

        for line in stdout.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let version = parts[0].to_string();
                let path = PathBuf::from(parts[1]);
                pythons.push((version, path));
            }
        }

        Ok(pythons)
    }

    pub fn install_python(version: &str) -> Result<PathBuf> {
        UvIntegration::ensure_installed()?;

        let output = Command::new("uv")
            .arg("python")
            .arg("install")
            .arg(version)
            .output()
            .context("Failed to install Python")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to install Python {}: {}", version, stderr);
        }

        let pythons = Self::discover_pythons()?;

        // Handle different version formats
        // If version doesn't start with "cpython-", try to find it with cpython prefix
        let search_version = if version.starts_with("cpython-") {
            version.to_string()
        } else {
            format!("cpython-{}", version)
        };

        pythons
            .into_iter()
            .find(|(v, p)| {
                // Check if the discovered version starts with our search version
                // This handles cases like "cpython-3.11" matching "cpython-3.11.12-macos-aarch64-none"
                v.starts_with(&search_version) && !p.to_string_lossy().contains("<download")
            })
            .map(|(_, path)| path)
            .ok_or_else(|| anyhow::anyhow!("Python {} installed but not found", version))
    }
}
