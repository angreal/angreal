---
title: "angreal::python_bindings::integrations::compose"
description: "PyO3 bindings for Docker Compose integration"
sidebar:
  order: 36
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


PyO3 bindings for Docker Compose integration

## Structs

### <span id="struct.PyDockerCompose"></span>`class DockerCompose`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose](../../../../../angreal/python_bindings/integrations/compose/#class.DockerCompose)

Python wrapper for Docker Compose operations

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `inner` | `RustDockerCompose` |  |

#### Methods

##### <span id="PyDockerCompose.new"></span>`new`

```rust
fn new (file : & str , project_name : Option < & str >) -> PyResult < Self >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.new](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.new)

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



##### <span id="PyDockerCompose.is_available"></span>`is_available`

```rust
fn is_available () -> bool
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.is_available](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.is_available)

Check if Docker Compose is available

<details>
<summary>Source</summary>

```rust
    fn is_available() -> bool {
        RustDockerCompose::is_available()
    }
```

</details>



##### <span id="PyDockerCompose.compose_file"></span>`compose_file`

```rust
fn compose_file (& self) -> String
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.compose_file](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.compose_file)

Get the compose file path

<details>
<summary>Source</summary>

```rust
    fn compose_file(&self) -> String {
        self.inner.compose_file().display().to_string()
    }
```

</details>



##### <span id="PyDockerCompose.working_dir"></span>`working_dir`

```rust
fn working_dir (& self) -> String
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.working_dir](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.working_dir)

Get the working directory

<details>
<summary>Source</summary>

```rust
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>



##### <span id="PyDockerCompose.project_name"></span>`project_name`

```rust
fn project_name (& self) -> Option < String >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.project_name](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.project_name)

Get the project name

<details>
<summary>Source</summary>

```rust
    fn project_name(&self) -> Option<String> {
        self.inner.project_name().map(|s| s.to_string())
    }
```

</details>



##### <span id="PyDockerCompose.up"></span>`up`

```rust
fn up (& self , detach : bool , build : bool , remove_orphans : bool , force_recreate : bool , no_recreate : bool , services : Option < Vec < String > > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.up](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.up)

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



##### <span id="PyDockerCompose.down"></span>`down`

```rust
fn down (& self , volumes : bool , remove_orphans : bool , timeout : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.down](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.down)

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



##### <span id="PyDockerCompose.restart"></span>`restart`

```rust
fn restart (& self , services : Option < Vec < String > > , timeout : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.restart](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.restart)

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



##### <span id="PyDockerCompose.logs"></span>`logs`

```rust
fn logs (& self , services : Option < Vec < String > > , follow : bool , timestamps : bool , tail : Option < String > , since : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.logs](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.logs)

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



##### <span id="PyDockerCompose.ps"></span>`ps`

```rust
fn ps (& self , all : bool , quiet : bool , services : bool , filter_services : Option < Vec < String > > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.ps](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.ps)

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



##### <span id="PyDockerCompose.build"></span>`build`

```rust
fn build (& self , services : Option < Vec < String > > , no_cache : bool , pull : bool , parallel : bool ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.build](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.build)

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



##### <span id="PyDockerCompose.start"></span>`start`

```rust
fn start (& self , services : Option < Vec < String > >) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.start](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.start)

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



##### <span id="PyDockerCompose.stop"></span>`stop`

```rust
fn stop (& self , services : Option < Vec < String > > , timeout : Option < String > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.stop](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.stop)

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



##### <span id="PyDockerCompose.exec"></span>`exec`

```rust
fn exec (& self , service : & str , command : Vec < String > , detach : bool , tty : bool , user : Option < String > , workdir : Option < String > , env : Option < HashMap < String , String > > ,) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.exec](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.exec)

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



##### <span id="PyDockerCompose.pull"></span>`pull`

```rust
fn pull (& self , services : Option < Vec < String > >) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.pull](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.pull)

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



##### <span id="PyDockerCompose.config"></span>`config`

```rust
fn config (& self , quiet : bool , services : bool , volumes : bool) -> PyResult < PyComposeOutput >
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.config](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.config)

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



##### <span id="PyDockerCompose.__repr__"></span>`__repr__`

```rust
fn __repr__ (& self) -> String
```

> **Python API**: [angreal.python_bindings.integrations.compose.DockerCompose.__repr__](../../../../../angreal/python_bindings/integrations/compose/#DockerCompose.__repr__)

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





### <span id="struct.PyComposeOutput"></span>`class ComposeResult`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


> **Python API**: [angreal.python_bindings.integrations.compose.ComposeResult](../../../../../angreal/python_bindings/integrations/compose/#class.ComposeResult)

Python wrapper for compose command output

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `success` | `bool` |  |
| `exit_code` | `i32` |  |
| `stdout` | `String` |  |
| `stderr` | `String` |  |



## Functions

### <span id="fn.compose"></span>`fn compose`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


> **Python API**: [angreal.python_bindings.integrations.compose.compose](../../../../../angreal/python_bindings/integrations/compose/#fn.compose)

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



### <span id="fn.compose_integration"></span>`fn compose_integration`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


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
