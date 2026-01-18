# flox


Flox development environment integration

This module provides Rust bindings for the Flox CLI, enabling
environment activation and services management.

## Structs

### `struct FloxIntegration`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Core integration for the Flox CLI

#### Methods

##### `is_available`


```rust
fn is_available () -> bool
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



##### `version`


```rust
fn version () -> Result < String >
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





### `struct ServiceStatus`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`, `PartialEq`

Represents a service status entry from `flox services status`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `status` | `String` |  |
| `pid` | `Option < u32 >` |  |



### `struct FloxEnvironment`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


Flox environment wrapper for a specific directory

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `path` | `PathBuf` | Path to the directory containing the Flox environment (.flox/) |

#### Methods

##### `new`


```rust
fn new < P : AsRef < Path > > (path : P) -> Self
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



##### `exists`


```rust
fn exists (& self) -> bool
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



##### `manifest_path`


```rust
fn manifest_path (& self) -> PathBuf
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



##### `has_manifest`


```rust
fn has_manifest (& self) -> bool
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



##### `get_activation_env`


```rust
fn get_activation_env (& self) -> Result < HashMap < String , String > >
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



##### `parse_activation_script`


```rust
fn parse_activation_script (script : & str) -> Result < HashMap < String , String > >
```

Parse the activation script to extract environment variable exports

Looks for patterns like:
- `export VAR="value"`
- `export VAR='value'`
- `export VAR=value`
- `VAR="value"; export VAR`

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `unquote_value`


```rust
fn unquote_value (value : & str) -> String
```

Remove surrounding quotes from a value

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `run_in_env`


```rust
fn run_in_env (& self , command : & str , args : & [& str]) -> Result < Output >
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



##### `services_start`


```rust
fn services_start (& self , services : & [& str]) -> Result < () >
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



##### `services_stop`


```rust
fn services_stop (& self) -> Result < () >
```

Stop all Flox services

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `services_status`


```rust
fn services_status (& self) -> Result < Vec < ServiceStatus > >
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



##### `parse_services_status`


```rust
fn parse_services_status (output : & str) -> Result < Vec < ServiceStatus > >
```

Parse the output of `flox services status`

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `services_logs`


```rust
fn services_logs (& self , service : & str , follow : bool , tail : Option < u32 >) -> Result < String >
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



##### `services_restart`


```rust
fn services_restart (& self , services : & [& str]) -> Result < () >
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
