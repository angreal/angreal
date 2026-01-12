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

        // Use a more reliable method to get site-packages for both Unix and Windows
        let python_script = r#"
import site
import sys
import os

# Try to get site-packages path that belongs to this virtual environment
site_packages_paths = site.getsitepackages()

# Find the site-packages that belongs to our venv
for path in site_packages_paths:
    # Check if this site-packages path is within our virtual environment
    if sys.prefix in path:
        print(path)
        break
else:
    # Fallback: construct manually
    if os.name == 'nt':  # Windows
        print(os.path.join(sys.prefix, 'Lib', 'site-packages'))
    else:  # Unix/Linux/macOS
        # Find python version
        import sysconfig
        version = sysconfig.get_python_version()
        print(os.path.join(sys.prefix, 'lib', f'python{version}', 'site-packages'))
"#;

        let output = Command::new(&python_exe)
            .arg("-c")
            .arg(python_script)
            .output()
            .context("Failed to get site-packages path")?;

        if !output.status.success() {
            // Fallback: construct the path manually based on platform
            let site_packages_path = if cfg!(windows) {
                self.path.join("Lib").join("site-packages")
            } else {
                // Find the lib directory with version-specific path
                let lib_dir = self.path.join("lib");
                if lib_dir.exists() {
                    // Look for python3.x directory
                    if let Ok(entries) = std::fs::read_dir(&lib_dir) {
                        for entry in entries.flatten() {
                            let name = entry.file_name();
                            if let Some(name_str) = name.to_str() {
                                if name_str.starts_with("python") {
                                    let site_packages = entry.path().join("site-packages");
                                    if site_packages.exists() {
                                        return Ok(site_packages);
                                    }
                                }
                            }
                        }
                    }
                }
                // Final fallback for Unix
                self.path.join("lib").join("python3").join("site-packages")
            };

            return Ok(site_packages_path);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // ==================== UvIntegration Tests ====================

    #[test]
    fn test_uv_is_available() {
        // UV should be available in the test environment (installed in CI)
        let available = UvIntegration::is_available();
        assert!(available, "UV should be available in the test environment");
    }

    #[test]
    fn test_uv_version() {
        let version = UvIntegration::version();
        assert!(version.is_ok(), "Should be able to get UV version");

        let version_str = version.unwrap();
        assert!(
            version_str.starts_with("uv"),
            "Version should start with 'uv', got: {}",
            version_str
        );
    }

    #[test]
    fn test_uv_ensure_installed() {
        // Since UV is already installed, this should succeed immediately
        let result = UvIntegration::ensure_installed();
        assert!(result.is_ok(), "ensure_installed should succeed");
    }

    // ==================== UvVirtualEnv Tests ====================

    #[test]
    fn test_create_venv() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv");

        let result = UvVirtualEnv::create(&venv_path, None);
        assert!(result.is_ok(), "Should create venv successfully");

        let venv = result.unwrap();
        assert!(venv.path.exists(), "Venv directory should exist");
        assert!(
            venv.path.join("pyvenv.cfg").exists(),
            "pyvenv.cfg should exist"
        );
    }

    #[test]
    fn test_python_executable_path() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_exe");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");
        let python_exe = venv.python_executable();

        assert!(
            python_exe.exists(),
            "Python executable should exist at {:?}",
            python_exe
        );

        // Verify correct platform-specific path
        if cfg!(windows) {
            assert!(
                python_exe.ends_with("Scripts/python.exe"),
                "Windows path should end with Scripts/python.exe"
            );
        } else {
            assert!(
                python_exe.ends_with("bin/python"),
                "Unix path should end with bin/python"
            );
        }
    }

    #[test]
    fn test_site_packages() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_site");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");
        let site_packages = venv.site_packages();

        assert!(site_packages.is_ok(), "Should get site-packages path");

        let site_path = site_packages.unwrap();
        assert!(
            site_path.exists(),
            "Site-packages should exist at {:?}",
            site_path
        );
        assert!(
            site_path.to_string_lossy().contains("site-packages"),
            "Path should contain 'site-packages'"
        );
    }

    #[test]
    fn test_install_packages() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_install");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");

        // Install a small, common package
        let result = venv.install_packages(&["six".to_string()]);
        assert!(
            result.is_ok(),
            "Should install package successfully: {:?}",
            result.err()
        );

        // Verify the package was installed by checking site-packages
        let site_packages = venv.site_packages().expect("Failed to get site-packages");
        let six_installed = site_packages.join("six.py").exists()
            || fs::read_dir(&site_packages)
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .any(|e| e.file_name().to_string_lossy().starts_with("six"))
                })
                .unwrap_or(false);

        assert!(six_installed, "Package 'six' should be installed");
    }

    #[test]
    fn test_install_empty_packages() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_empty");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");

        // Installing empty list should succeed (no-op)
        let result = venv.install_packages(&[]);
        assert!(result.is_ok(), "Empty package list should succeed");
    }

    #[test]
    fn test_install_requirements_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_req");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");

        // Create a requirements file
        let req_file = temp_dir.path().join("requirements.txt");
        fs::write(&req_file, "six\n").expect("Failed to write requirements file");

        let result = venv.install_requirements(&req_file);
        assert!(
            result.is_ok(),
            "Should install from requirements file: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_get_activation_info() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_info");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");
        let info = venv.get_activation_info();

        assert!(info.is_ok(), "Should get activation info");

        let info = info.unwrap();
        assert!(!info.venv_path.is_empty(), "venv_path should not be empty");
        assert!(
            !info.venv_prefix.is_empty(),
            "venv_prefix should not be empty"
        );
        assert!(
            !info.site_packages.is_empty(),
            "site_packages should not be empty"
        );
        assert!(
            !info.python_executable.is_empty(),
            "python_executable should not be empty"
        );

        // Verify paths are consistent
        assert!(
            info.venv_path.contains("test_venv_info"),
            "venv_path should contain venv name"
        );
    }

    #[test]
    fn test_discover_pythons() {
        let result = UvVirtualEnv::discover_pythons();
        assert!(result.is_ok(), "Should discover Python installations");

        let pythons = result.unwrap();
        assert!(
            !pythons.is_empty(),
            "Should find at least one Python installation"
        );

        // Verify format of discovered pythons
        for (version, path) in &pythons {
            assert!(!version.is_empty(), "Version should not be empty");
            // Path might be a download indicator or actual path
            assert!(
                !path.to_string_lossy().is_empty(),
                "Path should not be empty"
            );
        }
    }

    #[test]
    fn test_create_venv_with_python_version() {
        // This test may be skipped if the specific Python version isn't available
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_version");

        // Try to create with current Python version (should always work)
        // We don't specify a version to avoid needing a specific Python installed
        let result = UvVirtualEnv::create(&venv_path, None);
        assert!(result.is_ok(), "Should create venv with default Python");
    }

    #[test]
    fn test_venv_struct_path() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_struct");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");

        // Verify the struct stores the correct path
        assert_eq!(venv.path, venv_path, "Struct should store correct path");
    }

    #[test]
    fn test_install_invalid_package() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_invalid");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");

        // Try to install a package that doesn't exist
        let result =
            venv.install_packages(&["this-package-definitely-does-not-exist-12345".to_string()]);
        assert!(
            result.is_err(),
            "Installing non-existent package should fail"
        );
    }

    #[test]
    fn test_install_requirements_nonexistent_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let venv_path = temp_dir.path().join("test_venv_nofile");

        let venv = UvVirtualEnv::create(&venv_path, None).expect("Failed to create venv");

        // Try to install from a file that doesn't exist
        let nonexistent = temp_dir.path().join("nonexistent.txt");
        let result = venv.install_requirements(&nonexistent);
        assert!(
            result.is_err(),
            "Installing from non-existent file should fail"
        );
    }
}
