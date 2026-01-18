# compose


PyO3 bindings for Docker Compose integration

## Structs

### `class DockerCompose`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose](../../../../angreal/python_bindings/integrations/compose.md#class-dockercompose)

Python wrapper for Docker Compose operations

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `inner` | `RustDockerCompose` |  |

#### Methods

##### `new`

```rust
fn new (file : & str , project_name : Option < & str >) -> PyResult < Self >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.new](../../../../angreal/python_bindings/integrations/compose.md#new)

Create a new DockerCompose instance

<details>
<summary>Source</summary>

```rust
    fn new(file: &str, project_name: Option<&str>) -> PyResult<Self> {
        let mut compose = RustDockerCompose::new(file).map_err(|e| {
            PyIOError::new_err(format!("Failed to create Docker Compose instance: {}", e))
        })?;

        if let Some(name) = project_name {
            compose = compose.with_project_name(name);
        }

        Ok(Self { inner: compose })
    }
```

</details>



##### `is_available`

```rust
fn is_available () -> bool
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.is_available](../../../../angreal/python_bindings/integrations/compose.md#is_available)

Check if Docker Compose is available

<details>
<summary>Source</summary>

```rust
    fn is_available() -> bool {
        RustDockerCompose::is_available()
    }
```

</details>



##### `compose_file`

```rust
fn compose_file (& self) -> String
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.compose_file](../../../../angreal/python_bindings/integrations/compose.md#compose_file)

Get the compose file path

<details>
<summary>Source</summary>

```rust
    fn compose_file(&self) -> String {
        self.inner.compose_file().display().to_string()
    }
```

</details>



##### `working_dir`

```rust
fn working_dir (& self) -> String
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.working_dir](../../../../angreal/python_bindings/integrations/compose.md#working_dir)

Get the working directory

<details>
<summary>Source</summary>

```rust
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>



##### `project_name`

```rust
fn project_name (& self) -> Option < String >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.project_name](../../../../angreal/python_bindings/integrations/compose.md#project_name)

Get the project name

<details>
<summary>Source</summary>

```rust
    fn project_name(&self) -> Option<String> {
        self.inner.project_name().map(|s| s.to_string())
    }
```

</details>



##### `up`

```rust
fn up (& self , detach : bool , build : bool , remove_orphans : bool , force_recreate : bool , no_recreate : bool , services : Option < Vec < String > > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.up](../../../../angreal/python_bindings/integrations/compose.md#up)

Start services (docker-compose up)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `down`

```rust
fn down (& self , volumes : bool , remove_orphans : bool , timeout : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.down](../../../../angreal/python_bindings/integrations/compose.md#down)

Stop and remove services (docker-compose down)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `restart`

```rust
fn restart (& self , services : Option < Vec < String > > , timeout : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.restart](../../../../angreal/python_bindings/integrations/compose.md#restart)

Restart services (docker-compose restart)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `logs`

```rust
fn logs (& self , services : Option < Vec < String > > , follow : bool , timestamps : bool , tail : Option < String > , since : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.logs](../../../../angreal/python_bindings/integrations/compose.md#logs)

View service logs (docker-compose logs)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `ps`

```rust
fn ps (& self , all : bool , quiet : bool , services : bool , filter_services : Option < Vec < String > > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.ps](../../../../angreal/python_bindings/integrations/compose.md#ps)

List running services (docker-compose ps)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `build`

```rust
fn build (& self , services : Option < Vec < String > > , no_cache : bool , pull : bool , parallel : bool ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.build](../../../../angreal/python_bindings/integrations/compose.md#build)

Build services (docker-compose build)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `start`

```rust
fn start (& self , services : Option < Vec < String > >) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.start](../../../../angreal/python_bindings/integrations/compose.md#start)

Start services (docker-compose start)

<details>
<summary>Source</summary>

```rust
    fn start(&self, services: Option<Vec<String>>) -> PyResult<PyComposeOutput> {
        let services = services.unwrap_or_default();
        let result = self
            .inner
            .start(&services)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose start failed: {}", e)))?;

        Ok(result.into())
    }
```

</details>



##### `stop`

```rust
fn stop (& self , services : Option < Vec < String > > , timeout : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.stop](../../../../angreal/python_bindings/integrations/compose.md#stop)

Stop services (docker-compose stop)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `exec`

```rust
fn exec (& self , service : & str , command : Vec < String > , detach : bool , tty : bool , user : Option < String > , workdir : Option < String > , env : Option < HashMap < String , String > > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.exec](../../../../angreal/python_bindings/integrations/compose.md#exec)

Execute a command in a service container (docker-compose exec)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `pull`

```rust
fn pull (& self , services : Option < Vec < String > >) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.pull](../../../../angreal/python_bindings/integrations/compose.md#pull)

Pull service images (docker-compose pull)

<details>
<summary>Source</summary>

```rust
    fn pull(&self, services: Option<Vec<String>>) -> PyResult<PyComposeOutput> {
        let services = services.unwrap_or_default();
        let result = self
            .inner
            .pull(&services)
            .map_err(|e| PyRuntimeError::new_err(format!("Docker Compose pull failed: {}", e)))?;

        Ok(result.into())
    }
```

</details>



##### `config`

```rust
fn config (& self , quiet : bool , services : bool , volumes : bool) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.config](../../../../angreal/python_bindings/integrations/compose.md#config)

Validate and view the compose configuration (docker-compose config)

<details>
<summary>Source</summary>

```rust
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
```

</details>



##### `__repr__`

```rust
fn __repr__ (& self) -> String
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.__repr__](../../../../angreal/python_bindings/integrations/compose.md#__repr__)

String representation

<details>
<summary>Source</summary>

```rust
    fn __repr__(&self) -> String {
        format!(
            "DockerCompose(file='{}', project_name={:?})",
            self.inner.compose_file().display(),
            self.inner.project_name()
        )
    }
```

</details>





### `class ComposeResult`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.integrations.compose.ComposeResult](../../../../angreal/python_bindings/integrations/compose.md#class-composeresult)

Python wrapper for compose command output

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `success` | `bool` |  |
| `exit_code` | `i32` |  |
| `stdout` | `String` |  |
| `stderr` | `String` |  |



## Functions

### `fn compose`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.python_bindings.integrations.compose.compose](../../../../angreal/python_bindings/integrations/compose.md#compose)

```rust
fn compose (file : & str , project_name : Option < & str >) -> PyResult < PyDockerCompose >
```

Convenience function to create a DockerCompose instance

<details>
<summary>Source</summary>

```rust
pub fn compose(file: &str, project_name: Option<&str>) -> PyResult<PyDockerCompose> {
    PyDockerCompose::new(file, project_name)
}
```

</details>



### `fn compose_integration`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn compose_integration (_py : Python , m : & Bound < '_ , PyModule >) -> PyResult < () >
```

Compose integration module

<details>
<summary>Source</summary>

```rust
pub fn compose_integration(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDockerCompose>()?;
    m.add_class::<PyComposeOutput>()?;
    m.add_function(wrap_pyfunction!(compose, m)?)?;
    Ok(())
}
```

</details>
