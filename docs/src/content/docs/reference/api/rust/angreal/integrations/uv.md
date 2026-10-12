---
title: "angreal::integrations::uv"
description: "API reference of the Rust module angreal::integrations::uv."
editUrl: false
sidebar:
  order: 24
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

## Structs

### <span id="struct.UvIntegration"></span><code style="font-size: 0.85em">struct UvIntegration</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

#### Methods

##### <span id="UvIntegration.is_available"></span><code>is_​available</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn is_available() -> bool
```

<details>
<summary>Source</summary>

```rust
    pub fn is_available() -> bool {
        Command::new("uv")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
```

</details>

##### <span id="UvIntegration.version"></span><code>version</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn version() -> Result<String>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="UvIntegration.ensure_installed"></span><code>ensure_​installed</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn ensure_installed() -> Result<()>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="struct.ActivationInfo"></span><code style="font-size: 0.85em">class ActivationInfo</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

> **Python API**: [`angreal.ActivationInfo`](../../../../angreal/#class.ActivationInfo)

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `venv_path` | `String` |  |
| `venv_prefix` | `String` |  |
| `site_packages` | `String` |  |
| `python_executable` | `String` |  |

### <span id="struct.UvVirtualEnv"></span><code style="font-size: 0.85em">struct UvVirtualEnv</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `PathBuf` |  |

#### Methods

##### <span id="UvVirtualEnv.create"></span><code>create</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn create(path: &Path, python_version: Option<&str>) -> Result<Self>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="UvVirtualEnv.install_packages"></span><code>install_​packages</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn install_packages(&self, packages: &[String]) -> Result<()>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="UvVirtualEnv.install_requirements"></span><code>install_​requirements</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn install_requirements(&self, requirements_file: &Path) -> Result<()>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="UvVirtualEnv.python_executable"></span><code>python_​executable</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn python_executable(&self) -> PathBuf
```

<details>
<summary>Source</summary>

```rust
    pub fn python_executable(&self) -> PathBuf {
        if cfg!(windows) {
            self.path.join("Scripts").join("python.exe")
        } else {
            self.path.join("bin").join("python")
        }
    }
```

</details>

##### <span id="UvVirtualEnv.site_packages"></span><code>site_​packages</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn site_packages(&self) -> Result<PathBuf>
```

<details>
<summary>Source</summary>

```rust
    pub fn site_packages(&self) -> Result<PathBuf> {
        let python_exe = self.python_executable();

        // Ask the venv's own interpreter where it installs pure-Python packages.
        // `sysconfig` resolves this against `sys.prefix`, so inside a virtual
        // environment it is the venv's site-packages on every platform:
        // `<venv>/lib/pythonX.Y/site-packages` on Unix and
        // `<venv>\Lib\site-packages` on Windows.
        //
        // Do not derive this from `site.getsitepackages()`: on Windows its first
        // entry is `sys.prefix` itself (the venv root), so a "first path inside
        // the prefix" search returns the venv root instead of site-packages.
        let python_script = "import sysconfig; print(sysconfig.get_paths()['purelib'])";

        let output = Command::new(&python_exe)
            .arg("-c")
            .arg(python_script)
            .output()
            .context("Failed to get site-packages path")?;

        if output.status.success() {
            let site_packages_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !site_packages_str.is_empty() {
                return Ok(PathBuf::from(site_packages_str));
            }
        }

        // The interpreter did not answer: construct the path from the known
        // venv layout for the platform.
        Ok(self.fallback_site_packages())
    }
```

</details>

##### <span id="UvVirtualEnv.get_activation_info"></span><code>get_​activation_​info</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn get_activation_info(&self) -> Result<ActivationInfo>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="UvVirtualEnv.discover_pythons"></span><code>discover_​pythons</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn discover_pythons() -> Result<Vec<(String, PathBuf)>>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="UvVirtualEnv.install_python"></span><code>install_​python</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn install_python(version: &str) -> Result<PathBuf>
```

<details>
<summary>Source</summary>

```rust
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
```

</details>
