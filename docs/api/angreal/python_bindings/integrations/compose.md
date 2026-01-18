# compose


PyO3 bindings for Docker Compose integration

## Classes

### `class DockerCompose`

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose](../../../rust/angreal/python_bindings/integrations/compose.md#class-dockercompose)

Python wrapper for Docker Compose operations

#### Methods

##### `new`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">new</span>(file:  str, project_name: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Self &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::new](../../../rust/angreal/python_bindings/integrations/compose.md#new)

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



##### `is_available`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">is_available</span>() -> <span style="color: var(--md-default-fg-color--light);">bool</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::is_available](../../../rust/angreal/python_bindings/integrations/compose.md#is_available)

Check if Docker Compose is available

<details>
<summary>Source</summary>

```python
    fn is_available() -> bool {
        RustDockerCompose::is_available()
    }
```

</details>



##### `compose_file`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">compose_file</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::compose_file](../../../rust/angreal/python_bindings/integrations/compose.md#compose_file)

Get the compose file path

<details>
<summary>Source</summary>

```python
    fn compose_file(&self) -> String {
        self.inner.compose_file().display().to_string()
    }
```

</details>



##### `working_dir`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">working_dir</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::working_dir](../../../rust/angreal/python_bindings/integrations/compose.md#working_dir)

Get the working directory

<details>
<summary>Source</summary>

```python
    fn working_dir(&self) -> String {
        self.inner.working_dir().display().to_string()
    }
```

</details>



##### `project_name`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">project_name</span>() -> <span style="color: var(--md-default-fg-color--light);">Option &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::project_name](../../../rust/angreal/python_bindings/integrations/compose.md#project_name)

Get the project name

<details>
<summary>Source</summary>

```python
    fn project_name(&self) -> Option<String> {
        self.inner.project_name().map(|s| s.to_string())
    }
```

</details>



##### `up`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">up</span>(detach: bool, build: bool, remove_orphans: bool, force_recreate: bool, no_recreate: bool, services: Option &lt; Vec &lt; String &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::up](../../../rust/angreal/python_bindings/integrations/compose.md#up)

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



##### `down`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">down</span>(volumes: bool, remove_orphans: bool, timeout: Option &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::down](../../../rust/angreal/python_bindings/integrations/compose.md#down)

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



##### `restart`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">restart</span>(services: Option &lt; Vec &lt; String &gt; &gt;, timeout: Option &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::restart](../../../rust/angreal/python_bindings/integrations/compose.md#restart)

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



##### `logs`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">logs</span>(services: Option &lt; Vec &lt; String &gt; &gt;, follow: bool, timestamps: bool, tail: Option &lt; String &gt;, since: Option &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::logs](../../../rust/angreal/python_bindings/integrations/compose.md#logs)

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



##### `ps`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">ps</span>(all: bool, quiet: bool, services: bool, filter_services: Option &lt; Vec &lt; String &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::ps](../../../rust/angreal/python_bindings/integrations/compose.md#ps)

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



##### `build`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">build</span>(services: Option &lt; Vec &lt; String &gt; &gt;, no_cache: bool, pull: bool, parallel: bool) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::build](../../../rust/angreal/python_bindings/integrations/compose.md#build)

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



##### `start`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">start</span>(services: Option &lt; Vec &lt; String &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::start](../../../rust/angreal/python_bindings/integrations/compose.md#start)

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



##### `stop`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">stop</span>(services: Option &lt; Vec &lt; String &gt; &gt;, timeout: Option &lt; String &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::stop](../../../rust/angreal/python_bindings/integrations/compose.md#stop)

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



##### `exec`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">exec</span>(service:  str, command: Vec &lt; String &gt;, detach: bool, tty: bool, user: Option &lt; String &gt;, workdir: Option &lt; String &gt;, env: Option &lt; HashMap &lt; String , String &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::exec](../../../rust/angreal/python_bindings/integrations/compose.md#exec)

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



##### `pull`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">pull</span>(services: Option &lt; Vec &lt; String &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::pull](../../../rust/angreal/python_bindings/integrations/compose.md#pull)

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



##### `config`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">config</span>(quiet: bool, services: bool, volumes: bool) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyComposeOutput &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::config](../../../rust/angreal/python_bindings/integrations/compose.md#config)

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



##### `__repr__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__repr__</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyDockerCompose::__repr__](../../../rust/angreal/python_bindings/integrations/compose.md#__repr__)

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





### `class ComposeResult`

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::PyComposeOutput](../../../rust/angreal/python_bindings/integrations/compose.md#class-composeresult)

Python wrapper for compose command output



## Functions

### `compose`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">compose</span>(file:  str, project_name: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; PyDockerCompose &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::integrations::compose::compose](../../../rust/angreal/python_bindings/integrations/compose.md#fn-compose)

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
