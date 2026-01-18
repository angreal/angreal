# docker_compose


Docker Compose integration using subprocess execution for reliability

This module provides a high-level interface to Docker Compose commands,
using subprocess execution to ensure compatibility with all Docker Compose versions.

## Structs

### `struct DockerCompose`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Clone`, `Debug`

Docker Compose integration using subprocess execution

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `compose_file` | `PathBuf` |  |
| `working_dir` | `PathBuf` |  |
| `project_name` | `Option < String >` |  |

#### Methods

##### `new`


```rust
fn new < P : AsRef < Path > > (compose_file : P) -> Result < Self >
```

Create a new Docker Compose instance

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `with_project_name`


```rust
fn with_project_name < S : Into < String > > (mut self , name : S) -> Self
```

Set a custom project name

<details>
<summary>Source</summary>

```rust
    pub fn with_project_name<S: Into<String>>(mut self, name: S) -> Self {
        self.project_name = Some(name.into());
        self
    }
```

</details>



##### `is_available`


```rust
fn is_available () -> bool
```

Check if docker-compose is available

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `compose_file`


```rust
fn compose_file (& self) -> & Path
```

Get the compose file path

<details>
<summary>Source</summary>

```rust
    pub fn compose_file(&self) -> &Path {
        &self.compose_file
    }
```

</details>



##### `working_dir`


```rust
fn working_dir (& self) -> & Path
```

Get the working directory

<details>
<summary>Source</summary>

```rust
    pub fn working_dir(&self) -> &Path {
        &self.working_dir
    }
```

</details>



##### `project_name`


```rust
fn project_name (& self) -> Option < & str >
```

Get the project name

<details>
<summary>Source</summary>

```rust
    pub fn project_name(&self) -> Option<&str> {
        self.project_name.as_deref()
    }
```

</details>



##### `execute_command`


```rust
fn execute_command (& self , args : & [& str]) -> Result < ComposeOutput >
```

Execute a docker-compose command

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `up`


```rust
fn up (& self , options : UpOptions) -> Result < ComposeOutput >
```

Start services (docker-compose up)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `down`


```rust
fn down (& self , options : DownOptions) -> Result < ComposeOutput >
```

Stop and remove services (docker-compose down)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `restart`


```rust
fn restart (& self , options : RestartOptions) -> Result < ComposeOutput >
```

Restart services (docker-compose restart)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `logs`


```rust
fn logs (& self , options : LogsOptions) -> Result < ComposeOutput >
```

View service logs (docker-compose logs)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `ps`


```rust
fn ps (& self , options : PsOptions) -> Result < ComposeOutput >
```

List running services (docker-compose ps)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `build`


```rust
fn build (& self , options : BuildOptions) -> Result < ComposeOutput >
```

Build services (docker-compose build)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `start`


```rust
fn start (& self , services : & [String]) -> Result < ComposeOutput >
```

Start services (docker-compose start)

<details>
<summary>Source</summary>

```rust
    pub fn start(&self, services: &[String]) -> Result<ComposeOutput> {
        let mut args = vec!["start"];
        for service in services {
            args.push(service);
        }
        self.execute_command(&args)
    }
```

</details>



##### `stop`


```rust
fn stop (& self , options : StopOptions) -> Result < ComposeOutput >
```

Stop services (docker-compose stop)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `exec`


```rust
fn exec (& self , service : & str , command : & [String] , options : ExecOptions ,) -> Result < ComposeOutput >
```

Execute a command in a service container (docker-compose exec)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `pull`


```rust
fn pull (& self , services : & [String]) -> Result < ComposeOutput >
```

Pull service images (docker-compose pull)

<details>
<summary>Source</summary>

```rust
    pub fn pull(&self, services: &[String]) -> Result<ComposeOutput> {
        let mut args = vec!["pull"];
        for service in services {
            args.push(service);
        }
        self.execute_command(&args)
    }
```

</details>



##### `config`


```rust
fn config (& self , options : ConfigOptions) -> Result < ComposeOutput >
```

Validate and view the compose configuration (docker-compose config)

<details>
<summary>Source</summary>

```rust
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
```

</details>





### `struct ComposeOutput`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Clone`

Result structure for Docker Compose operations

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `success` | `bool` |  |
| `exit_code` | `i32` |  |
| `stdout` | `String` |  |
| `stderr` | `String` |  |



### `struct UpOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose up command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `detach` | `bool` |  |
| `build` | `bool` |  |
| `remove_orphans` | `bool` |  |
| `force_recreate` | `bool` |  |
| `no_recreate` | `bool` |  |
| `services` | `Vec < String >` |  |



### `struct DownOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose down command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `volumes` | `bool` |  |
| `remove_orphans` | `bool` |  |
| `timeout` | `Option < String >` |  |



### `struct RestartOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose restart command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `timeout` | `Option < String >` |  |
| `services` | `Vec < String >` |  |



### `struct LogsOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose logs command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `follow` | `bool` |  |
| `timestamps` | `bool` |  |
| `tail` | `Option < String >` |  |
| `since` | `Option < String >` |  |
| `services` | `Vec < String >` |  |



### `struct PsOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose ps command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `all` | `bool` |  |
| `quiet` | `bool` |  |
| `services` | `bool` |  |
| `filter_services` | `Vec < String >` |  |



### `struct BuildOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose build command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `no_cache` | `bool` |  |
| `pull` | `bool` |  |
| `parallel` | `bool` |  |
| `services` | `Vec < String >` |  |



### `struct StopOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose stop command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `timeout` | `Option < String >` |  |
| `services` | `Vec < String >` |  |



### `struct ExecOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose exec command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `detach` | `bool` |  |
| `tty` | `bool` |  |
| `user` | `Option < String >` |  |
| `workdir` | `Option < String >` |  |
| `env` | `HashMap < String , String >` |  |



### `struct ConfigOptions`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


**Derives:** `Debug`, `Default`, `Clone`

Options for docker-compose config command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `quiet` | `bool` |  |
| `services` | `bool` |  |
| `volumes` | `bool` |  |
