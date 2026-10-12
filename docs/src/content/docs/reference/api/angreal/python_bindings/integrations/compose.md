---
title: "angreal.python_bindings.integrations.compose"
description: "PyO3 bindings for Docker Compose integration"
sidebar:
  order: 5
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


PyO3 bindings for Docker Compose integration

## Classes

### <span id="class.DockerCompose"></span>`class DockerCompose`

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose](../../../../rust/angreal/python_bindings/integrations/compose/#struct.PyDockerCompose)

Python wrapper for Docker Compose operations

#### Methods

##### <span id="DockerCompose.new"></span>`new`

```python
def new(file:  str, project_name: Option < & str >) -> PyResult < Self >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::new](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.new)

Create a new DockerCompose instance

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `file` | ` str` |  |
| `project_name` | `Option < & str >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.is_available"></span>`is_available`

```python
def is_available() -> bool
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::is_available](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.is_available)

Check if Docker Compose is available

<details>
<summary>Source</summary>

```python
    fn is_available() -> bool {
        RustDockerCompose::is_available()
    }
```

</details>



##### <span id="DockerCompose.compose_file"></span>`compose_file`

```python
def compose_file() -> str
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::compose_file](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.compose_file)

Get the compose file path

<details>
<summary>Source</summary>

```python
    fn compose_file(&self) -> String {
        self.inner.compose_file().display().to_string()
    }
```

</details>



##### <span id="DockerCompose.working_dir"></span>`working_dir`

```python
def working_dir() -> str
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::working_dir](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.working_dir)

Get the working directory

<details>
<summary>Source</summary>

```python
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>



##### <span id="DockerCompose.project_name"></span>`project_name`

```python
def project_name() -> Option < String >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::project_name](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.project_name)

Get the project name

<details>
<summary>Source</summary>

```python
    fn project_name(&self) -> Option<String> {
        self.inner.project_name().map(|s| s.to_string())
    }
```

</details>



##### <span id="DockerCompose.up"></span>`up`

```python
def up(
    detach: bool,
    build: bool,
    remove_orphans: bool,
    force_recreate: bool,
    no_recreate: bool,
    services: Option < Vec < String > >,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::up](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.up)

Start services (docker-compose up)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `detach` | `bool` |  |
| `build` | `bool` |  |
| `remove_orphans` | `bool` |  |
| `force_recreate` | `bool` |  |
| `no_recreate` | `bool` |  |
| `services` | `Option < Vec < String > >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.down"></span>`down`

```python
def down(
    volumes: bool,
    remove_orphans: bool,
    timeout: Option < String >,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::down](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.down)

Stop and remove services (docker-compose down)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `volumes` | `bool` |  |
| `remove_orphans` | `bool` |  |
| `timeout` | `Option < String >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.restart"></span>`restart`

```python
def restart(
    services: Option < Vec < String > >,
    timeout: Option < String >,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::restart](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.restart)

Restart services (docker-compose restart)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | `Option < Vec < String > >` |  |
| `timeout` | `Option < String >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.logs"></span>`logs`

```python
def logs(
    services: Option < Vec < String > >,
    follow: bool,
    timestamps: bool,
    tail: Option < String >,
    since: Option < String >,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::logs](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.logs)

View service logs (docker-compose logs)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | `Option < Vec < String > >` |  |
| `follow` | `bool` |  |
| `timestamps` | `bool` |  |
| `tail` | `Option < String >` |  |
| `since` | `Option < String >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.ps"></span>`ps`

```python
def ps(
    all: bool,
    quiet: bool,
    services: bool,
    filter_services: Option < Vec < String > >,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::ps](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.ps)

List running services (docker-compose ps)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `all` | `bool` |  |
| `quiet` | `bool` |  |
| `services` | `bool` |  |
| `filter_services` | `Option < Vec < String > >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.build"></span>`build`

```python
def build(
    services: Option < Vec < String > >,
    no_cache: bool,
    pull: bool,
    parallel: bool,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::build](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.build)

Build services (docker-compose build)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | `Option < Vec < String > >` |  |
| `no_cache` | `bool` |  |
| `pull` | `bool` |  |
| `parallel` | `bool` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.start"></span>`start`

```python
def start(services: Option < Vec < String > >) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::start](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.start)

Start services (docker-compose start)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | `Option < Vec < String > >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.stop"></span>`stop`

```python
def stop(
    services: Option < Vec < String > >,
    timeout: Option < String >,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::stop](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.stop)

Stop services (docker-compose stop)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | `Option < Vec < String > >` |  |
| `timeout` | `Option < String >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.exec"></span>`exec`

```python
def exec(
    service:  str,
    command: Vec < String >,
    detach: bool,
    tty: bool,
    user: Option < String >,
    workdir: Option < String >,
    env: Option < HashMap < String , String > >,
) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::exec](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.exec)

Execute a command in a service container (docker-compose exec)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `service` | ` str` |  |
| `command` | `Vec < String >` |  |
| `detach` | `bool` |  |
| `tty` | `bool` |  |
| `user` | `Option < String >` |  |
| `workdir` | `Option < String >` |  |
| `env` | `Option < HashMap < String , String > >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.pull"></span>`pull`

```python
def pull(services: Option < Vec < String > >) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::pull](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.pull)

Pull service images (docker-compose pull)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `services` | `Option < Vec < String > >` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.config"></span>`config`

```python
def config(quiet: bool, services: bool, volumes: bool) -> PyResult < PyComposeOutput >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::config](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.config)

Validate and view the compose configuration (docker-compose config)

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `quiet` | `bool` |  |
| `services` | `bool` |  |
| `volumes` | `bool` |  |


<details>
<summary>Source</summary>

```python
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



##### <span id="DockerCompose.__repr__"></span>`__repr__`

```python
def __repr__() -> str
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::__repr__](../../../../rust/angreal/python_bindings/integrations/compose/#PyDockerCompose.__repr__)

String representation

<details>
<summary>Source</summary>

```python
    fn __repr__(&self) -> String {
        format!(
            "DockerCompose(file='{}', project_name={:?})",
            self.inner.compose_file().display(),
            self.inner.project_name()
        )
    }
```

</details>





### <span id="class.ComposeResult"></span>`class ComposeResult`

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyComposeOutput](../../../../rust/angreal/python_bindings/integrations/compose/#struct.PyComposeOutput)

Python wrapper for compose command output



## Functions

### <span id="fn.compose"></span>`compose`

```python
def compose(file:  str, project_name: Option < & str >) -> PyResult < PyDockerCompose >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::compose](../../../../rust/angreal/python_bindings/integrations/compose/#fn.compose)

Convenience function to create a DockerCompose instance

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `file` | ` str` |  |
| `project_name` | `Option < & str >` |  |


<details>
<summary>Source</summary>

```python
pub fn compose(file: &str, project_name: Option<&str>) -> PyResult<PyDockerCompose> {
    PyDockerCompose::new(file, project_name)
}
```

</details>
