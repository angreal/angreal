---
title: "angreal::python_bindings::integrations::oci"
description: "OCI registry integration bindings."
editUrl: false
sidebar:
  order: 35
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

OCI registry integration bindings.

Exposes a minimal `angreal.integrations.oci.Oci` surface — pull, push, tags,
login — over the shared OCI core (`crate::integrations::oci`). All heavy
lifting (registry I/O, auth, the template artifact convention) lives in the
core so this binding stays a thin adapter and matches the `init oci://…` path.

## Structs

### <span id="struct.PyOci"></span><code style="font-size: 0.85em">class Oci</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

> **Python API**: [`angreal.integrations.oci.Oci`](../../../../../angreal/integrations/oci/#class.Oci)

OCI registry client for angreal template artifacts.

Pull and push templates, list tags, and authenticate against registries.
Credentials set via `login()` are held for the lifetime of the instance and
take precedence over Docker `config.json` credentials for that registry.

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `credentials` | `Mutex<HashMap<String, OciAuth>>` | Per-registry credentials registered via `login()`. |

#### Methods

##### <span id="PyOci.new"></span><code>new</code>

```rust
fn new() -> Self
```

> **Python API**: [`angreal.integrations.oci.Oci.new`](../../../../../angreal/integrations/oci/#Oci.new)

<details>
<summary>Source</summary>

```rust
    fn new() -> Self {
        Self {
            credentials: Mutex::new(HashMap::new()),
        }
    }
```

</details>

##### <span id="PyOci.login"></span><code>login</code>

```rust
fn login(
    &self) -> PyResult<(,
    registry: &str) -> PyResult<(,
    username: String) -> PyResult<(,
    password: String) -> PyResult<(,
) -> PyResult<()>
```

> **Python API**: [`angreal.integrations.oci.Oci.login`](../../../../../angreal/integrations/oci/#Oci.login)

Store Basic credentials for `registry` (e.g. `ghcr.io`), used by subsequent pull/push/tags calls against that registry.

<details>
<summary>Source</summary>

```rust
    fn login(&self, registry: &str, username: String, password: String) -> PyResult<()> {
        self.credentials
            .lock()
            .expect("oci credentials mutex poisoned")
            .insert(registry.to_string(), OciAuth::Basic { username, password });
        Ok(())
    }
```

</details>

##### <span id="PyOci.pull"></span><code>pull</code>

```rust
fn pull(
    &self,
    reference: &str,
    dest: Option<PathBuf>,
    insecure: bool,
) -> PyResult<String>
```

> **Python API**: [`angreal.integrations.oci.Oci.pull`](../../../../../angreal/integrations/oci/#Oci.pull)

Pull a template artifact to `dest` (default: a directory named after the repository, under the current directory). Set `insecure=True` to pull from a plain-HTTP registry. Returns the destination path.

<details>
<summary>Source</summary>

```rust
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

##### <span id="PyOci.push"></span><code>push</code>

```rust
fn push(
    &self) -> PyResult<(,
    reference: &str) -> PyResult<(,
    path: PathBuf) -> PyResult<(,
    insecure: bool) -> PyResult<(,
) -> PyResult<()>
```

> **Python API**: [`angreal.integrations.oci.Oci.push`](../../../../../angreal/integrations/oci/#Oci.push)

Package the directory at `path` as a template artifact and push it to `reference`. Set `insecure=True` to push to a plain-HTTP registry.

<details>
<summary>Source</summary>

```rust
    fn push(&self, reference: &str, path: PathBuf, insecure: bool) -> PyResult<()> {
        let auth = self.auth_for(reference);
        Oci::push_artifact(reference, &path, &auth, insecure).map_err(to_pyerr)
    }
```

</details>

##### <span id="PyOci.tags"></span><code>tags</code>

```rust
fn tags(
    &self,
    repository: &str,
    insecure: bool,
) -> PyResult<Vec<String>>
```

> **Python API**: [`angreal.integrations.oci.Oci.tags`](../../../../../angreal/integrations/oci/#Oci.tags)

List the tags available for `repository`. Set `insecure=True` for a plain-HTTP registry.

<details>
<summary>Source</summary>

```rust
    fn tags(&self, repository: &str, insecure: bool) -> PyResult<Vec<String>> {
        let auth = self.auth_for(repository);
        Oci::list_tags(repository, &auth, insecure).map_err(to_pyerr)
    }
```

</details>

## Functions

### <span id="fn.oci"></span><code style="font-size: 0.85em">fn oci</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn oci(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()>
```

OCI integration module, exposed as `angreal.integrations.oci` in Python.

<details>
<summary>Source</summary>

```rust
pub fn oci(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyOci>()?;
    Ok(())
}
```

</details>
