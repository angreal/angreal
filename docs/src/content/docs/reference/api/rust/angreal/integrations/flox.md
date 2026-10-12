---
title: "angreal::integrations::flox"
description: "Flox development environment integration"
editUrl: false
sidebar:
  order: 21
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Flox development environment integration

This module provides Rust bindings for the Flox CLI, enabling
environment activation and services management.

## Structs

### <span id="struct.FloxIntegration"></span><code style="font-size: 0.85em">struct FloxIntegration</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Core integration for the Flox CLI

#### Methods

##### <span id="FloxIntegration.is_available"></span><code>is_​available</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn is_available() -> bool
```

Check if the `flox` binary is available in PATH

<details>
<summary>Source</summary>

```rust
    pub fn is_available() -> bool {
        Command::new("flox")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
```

</details>

##### <span id="FloxIntegration.version"></span><code>version</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn version() -> Result<String>
```

Get the Flox version string

<details>
<summary>Source</summary>

```rust
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
```

</details>

### <span id="struct.ServiceStatus"></span><code style="font-size: 0.85em">struct ServiceStatus</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

**Derives:** `Debug`, `Clone`, `PartialEq`

Represents a service status entry from `flox services status`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `status` | `String` |  |
| `pid` | `Option<u32>` |  |

### <span id="struct.FloxEnvironment"></span><code style="font-size: 0.85em">struct FloxEnvironment</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

Flox environment wrapper for a specific directory

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `PathBuf` | Path to the directory containing the Flox environment (.flox/) |

#### Methods

##### <span id="FloxEnvironment.new"></span><code>new</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn new<P: AsRef<Path>>(path: P) -> Self
```

Create a new FloxEnvironment reference for the given path

<details>
<summary>Source</summary>

```rust
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
```

</details>

##### <span id="FloxEnvironment.exists"></span><code>exists</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn exists(&self) -> bool
```

Check if this directory contains a Flox environment

<details>
<summary>Source</summary>

```rust
    pub fn exists(&self) -> bool {
        self.path.join(".flox").exists()
    }
```

</details>

##### <span id="FloxEnvironment.manifest_path"></span><code>manifest_​path</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn manifest_path(&self) -> PathBuf
```

Get the path to the manifest.toml if it exists

<details>
<summary>Source</summary>

```rust
    pub fn manifest_path(&self) -> PathBuf {
        self.path.join(".flox").join("env").join("manifest.toml")
    }
```

</details>

##### <span id="FloxEnvironment.has_manifest"></span><code>has_​manifest</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn has_manifest(&self) -> bool
```

Check if the manifest.toml exists

<details>
<summary>Source</summary>

```rust
    pub fn has_manifest(&self) -> bool {
        self.manifest_path().exists()
    }
```

</details>

##### <span id="FloxEnvironment.get_activation_env"></span><code>get_​activation_​env</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn get_activation_env(&self) -> Result<HashMap<String, String>>
```

Get environment variable modifications from `flox activate --print-script`

Parses the activation script to extract environment variable changes.
Returns a HashMap of variable names to their new values.

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="FloxEnvironment.run_in_env"></span><code>run_​in_​env</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn run_in_env(&self, command: &str, args: &[&str]) -> Result<Output>
```

Run a command within the Flox environment

Executes: `flox activate -d <path> -- <command> [args...]`

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="FloxEnvironment.services_start"></span><code>services_​start</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn services_start(&self, services: &[&str]) -> Result<()>
```

Start Flox services

If `services` is empty, starts all services defined in the manifest.
Otherwise, starts only the specified services.

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="FloxEnvironment.services_stop"></span><code>services_​stop</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn services_stop(&self, services: &[&str]) -> Result<()>
```

Stop Flox services

If `services` is empty, stops all services.
Otherwise, stops only the specified services.

<details>
<summary>Source</summary>

```rust
    pub fn services_stop(&self, services: &[&str]) -> Result<()> {
        let mut cmd = Command::new("flox");
        cmd.arg("services").arg("stop").arg("-d").arg(&self.path);

        for service in services {
            cmd.arg(service);
        }

        let output = cmd.output().context("Failed to stop Flox services")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to stop Flox services: {}", stderr);
        }

        Ok(())
    }
```

</details>

##### <span id="FloxEnvironment.services_status"></span><code>services_​status</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn services_status(&self) -> Result<Vec<ServiceStatus>>
```

Get status of all Flox services

Parses the output of `flox services status` which looks like:
```text
NAME      STATUS   PID
postgres  Running  12345
redis     Running  12346
```

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="FloxEnvironment.services_logs"></span><code>services_​logs</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn services_logs(
    &self,
    service: &str,
    follow: bool,
    tail: Option<u32>,
) -> Result<String>
```

Get logs for a specific service

<details>
<summary>Source</summary>

```rust
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
```

</details>

##### <span id="FloxEnvironment.services_restart"></span><code>services_​restart</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn services_restart(&self, services: &[&str]) -> Result<()>
```

Restart services

<details>
<summary>Source</summary>

```rust
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
```

</details>
