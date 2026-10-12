---
title: "angreal.python_bindings.integrations.oci"
description: "OCI registry integration bindings."
sidebar:
  order: 8
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


OCI registry integration bindings.

Exposes a minimal `angreal.integrations.oci.Oci` surface — pull, push, tags,
login — over the shared OCI core (`crate::integrations::oci`). All heavy
lifting (registry I/O, auth, the template artifact convention) lives in the
core so this binding stays a thin adapter and matches the `init oci://…` path.

## Classes

### <span id="class.Oci"></span>`class Oci`

> **Rust Implementation**: [angreal::python_bindings::integrations::oci::PyOci](../../../../rust/angreal/python_bindings/integrations/oci/#struct.PyOci)

OCI registry client for angreal template artifacts.

Pull and push templates, list tags, and authenticate against registries.
Credentials set via `login()` are held for the lifetime of the instance and
take precedence over Docker `config.json` credentials for that registry.

#### Methods

##### <span id="Oci.new"></span>`new`

```python
def new() -> Self
```

> **Rust Implementation**: [angreal::python_bindings::integrations::oci::PyOci::new](../../../../rust/angreal/python_bindings/integrations/oci/#PyOci.new)

<details>
<summary>Source</summary>

```python
    fn new() -> Self {
        Self {
            credentials: Mutex::new(HashMap::new()),
        }
    }
```

</details>



##### <span id="Oci.login"></span>`login`

```python
def login(registry:  str, username: str, password: str) -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::oci::PyOci::login](../../../../rust/angreal/python_bindings/integrations/oci/#PyOci.login)

Store Basic credentials for `registry` (e.g. `ghcr.io`), used by subsequent pull/push/tags calls against that registry.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `registry` | ` str` |  |
| `username` | `str` |  |
| `password` | `str` |  |


<details>
<summary>Source</summary>

```python
    fn login(&self, registry: &str, username: String, password: String) -> PyResult<()> {
        self.credentials
            .lock()
            .expect("oci credentials mutex poisoned")
            .insert(registry.to_string(), OciAuth::Basic { username, password });
        Ok(())
    }
```

</details>



##### <span id="Oci.pull"></span>`pull`

```python
def pull(
    reference:  str,
    dest: Option < PathBuf >,
    insecure: bool,
) -> PyResult < String >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::oci::PyOci::pull](../../../../rust/angreal/python_bindings/integrations/oci/#PyOci.pull)

Pull a template artifact to `dest` (default: a directory named after the repository, under the current directory). Set `insecure=True` to pull from a plain-HTTP registry. Returns the destination path.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `reference` | ` str` |  |
| `dest` | `Option < PathBuf >` |  |
| `insecure` | `bool` |  |


<details>
<summary>Source</summary>

```python
    fn pull(&self, reference: &str, dest: Option<PathBuf>, insecure: bool) -> PyResult<String> {
        let dest = match dest {
            Some(d) => d,
            None => {
                let name = core::reference_basename(reference).map_err(to_pyerr)?;
                std::env::current_dir()?.join(name)
            }
        };
        let auth = self.auth_for(reference);
        let out = Oci::pull_artifact(reference, &dest, &auth, insecure).map_err(to_pyerr)?;
        Ok(out.display().to_string())
    }
```

</details>



##### <span id="Oci.push"></span>`push`

```python
def push(reference:  str, path: PathBuf, insecure: bool) -> PyResult < () >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::oci::PyOci::push](../../../../rust/angreal/python_bindings/integrations/oci/#PyOci.push)

Package the directory at `path` as a template artifact and push it to `reference`. Set `insecure=True` to push to a plain-HTTP registry.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `reference` | ` str` |  |
| `path` | `PathBuf` |  |
| `insecure` | `bool` |  |


<details>
<summary>Source</summary>

```python
    fn push(&self, reference: &str, path: PathBuf, insecure: bool) -> PyResult<()> {
        let auth = self.auth_for(reference);
        Oci::push_artifact(reference, &path, &auth, insecure).map_err(to_pyerr)
    }
```

</details>



##### <span id="Oci.tags"></span>`tags`

```python
def tags(repository:  str, insecure: bool) -> PyResult < Vec < String > >
```

> **Rust Implementation**: [angreal::python_bindings::integrations::oci::PyOci::tags](../../../../rust/angreal/python_bindings/integrations/oci/#PyOci.tags)

List the tags available for `repository`. Set `insecure=True` for a plain-HTTP registry.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `repository` | ` str` |  |
| `insecure` | `bool` |  |


<details>
<summary>Source</summary>

```python
    fn tags(&self, repository: &str, insecure: bool) -> PyResult<Vec<String>> {
        let auth = self.auth_for(repository);
        Oci::list_tags(repository, &auth, insecure).map_err(to_pyerr)
    }
```

</details>
