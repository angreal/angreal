---
title: "OCI Registry Integration"
weight: 50
---

# angreal.integrations.oci

Pull and push angreal templates as OCI artifacts, list tags, and authenticate
against registries — daemonless and self-contained.

## Overview

Angreal's OCI integration talks directly to [OCI](https://opencontainers.org/)
registries (GHCR, Docker Hub, ECR, Harbor, Zot, a local `registry:2`, …) using a
pure-Rust client. It is the plugin-layer counterpart to consuming OCI templates
from the CLI with `angreal init oci://…` — both share the same core, so a
template you push with `Oci.push()` is one you can render with
`angreal init oci://…`.

Unlike the [Docker Compose integration](/angreal/reference/python-api/integrations/docker-compose),
this integration does **not** require a running Docker daemon. It performs
registry/artifact operations (pull, push, tag listing) over HTTPS.

## Prerequisites

None beyond angreal itself — the OCI client is bundled. To push to (or pull
from) a private repository you need credentials, supplied either via
`login()` or an existing Docker `~/.docker/config.json` (e.g. from
`docker login`).

## Template artifact convention (v1)

An angreal template is stored as a standard OCI image manifest with:

| Component | Media type |
|-----------|------------|
| Config blob | `application/vnd.angreal.template.config.v1+json` |
| Layer | `application/vnd.angreal.template.layer.v1.tar+gzip` |

The layer is a gzipped tar of the template directory — the same tree
`angreal init` renders. Because it is a spec-compliant manifest, tools like
`oras` and `skopeo` interoperate with artifacts angreal pushes.

## Classes

### Oci

Client for angreal template artifacts.

```python
from angreal.integrations.oci import Oci

client = Oci()
```

#### Constructor

```python
Oci()
```

Takes no arguments. Credentials registered with `login()` are held for the
lifetime of the instance.

#### Instance Methods

##### login

Store Basic credentials for a registry. Credentials set here take precedence
over Docker `config.json` for that registry, and are used by subsequent
`pull`/`push`/`tags` calls. No network request is made by `login()` itself.

```python
def login(registry: str, username: str, password: str) -> None
```

**Parameters:**
- `registry` (str): Registry host, e.g. `ghcr.io` or `localhost:5000`.
- `username` (str): Username.
- `password` (str): Password or token.

**Example:**
```python
import os

client = Oci()
client.login("ghcr.io", "my-user", os.environ["GHCR_TOKEN"])
```

##### pull

Pull a template artifact and unpack its layer.

```python
def pull(reference: str, dest: str | Path = None) -> str
```

**Parameters:**
- `reference` (str): Artifact reference, with or without the `oci://` scheme
  (e.g. `oci://ghcr.io/acme/py-template:latest`).
- `dest` (str | Path, optional): Destination directory. Defaults to a directory
  named after the repository (its last path segment) in the current directory.

**Returns:**
- `str`: The destination path the template was unpacked into.

**Example:**
```python
client = Oci()
path = client.pull("oci://ghcr.io/acme/py-template:latest", "./scaffold")
print(f"template unpacked to {path}")
```

##### push

Package a directory as a template artifact and push it.

```python
def push(reference: str, path: str | Path) -> None
```

**Parameters:**
- `reference` (str): Target reference, with or without the `oci://` scheme.
- `path` (str | Path): Template directory to package (must contain an
  `angreal.toml`).

**Example:**
```python
client = Oci()
client.push("oci://ghcr.io/acme/py-template:1.2.0", "./my-template")
```

##### tags

List the tags available for a repository.

```python
def tags(repository: str) -> list[str]
```

**Parameters:**
- `repository` (str): Repository reference (a tag, if present, is ignored),
  with or without the `oci://` scheme.

**Returns:**
- `list[str]`: Available tags.

**Example:**
```python
client = Oci()
for tag in client.tags("oci://ghcr.io/acme/py-template"):
    print(tag)
```

## Authentication resolution

For each operation, auth is resolved in this order:

1. Credentials registered via `login()` for the reference's registry.
2. Docker `~/.docker/config.json` credentials for that registry (e.g. after
   `docker login`).
3. Anonymous (public repositories).

## Errors

Registry, network, reference-parsing, and packaging failures surface as Python
`RuntimeError` with a descriptive message.

## Examples

### Publish a template from a task

```python
import angreal
from angreal.integrations.oci import Oci

@angreal.command(name="publish", about="Publish this template to GHCR")
@angreal.argument(name="version", long="version", takes_value=True, required=True)
def publish(version: str):
    """Package the template directory and push it as an OCI artifact."""
    import os

    client = Oci()
    client.login("ghcr.io", os.environ["GHCR_USER"], os.environ["GHCR_TOKEN"])
    client.push(f"oci://ghcr.io/acme/py-template:{version}", "./template")
    print(f"published py-template:{version}")
```

### Mirror a template between registries

```python
from angreal.integrations.oci import Oci

client = Oci()
tmp = client.pull("oci://ghcr.io/acme/py-template:latest", "/tmp/py-template")
client.push("oci://registry.internal:5000/acme/py-template:latest", tmp)
```

## See Also

- [Use OCI Template Targets](/angreal/how-to-guides/use-oci-templates) — consume
  and publish templates end-to-end.
- [angreal init Behavior](/angreal/explanation/angreal_init_behaviour) — how
  `angreal init` resolves an `oci://` reference.
- [Create Templates](/angreal/how-to-guides/create-templates) — authoring the
  template directory you publish.
