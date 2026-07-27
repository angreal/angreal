---
title: "Use OCI Template Targets"
weight: 45
---

# Use OCI Template Targets

This guide shows how to **consume** a template stored in an OCI registry and how
to **publish** one, so your templates can be versioned and distributed the same
way container images and Helm charts are.

Angreal's OCI support is self-contained: it needs no `docker`, `oras`, or
`skopeo` binary and no running daemon.

## Consume a template

Point `angreal init` at an `oci://` reference:

```bash
angreal init oci://ghcr.io/acme/py-template:latest
```

Angreal pulls the artifact, caches it under
`~/.angrealrc/oci/<registry>/<repository>/<tag>/`, then renders it exactly like
any other template (prompting for `angreal.toml` values and running `init.py`).
As with git templates, the artifact is re-fetched on each run so a moved tag is
always honored.

The `--in-place`, `--force`, and values-file flags work with `oci://` targets
just as they do for other templates:

```bash
angreal init oci://ghcr.io/acme/py-template:1.2.0 --in-place
```

### Private registries

For a private repository, authenticate first. Either `docker login` (angreal
reads `~/.docker/config.json`):

```bash
docker login ghcr.io
angreal init oci://ghcr.io/acme/private-template:latest
```

…or, from a task, register credentials on an `Oci` client (see below).

## Publish a template

Publishing packages your template directory (the tree containing `angreal.toml`)
into an OCI artifact and pushes it. Use the `Oci` client from a task or a script:

```python
import os
from angreal.integrations.oci import Oci

client = Oci()
client.login("ghcr.io", os.environ["GHCR_USER"], os.environ["GHCR_TOKEN"])
client.push("oci://ghcr.io/acme/py-template:1.2.0", "./template")
```

Anyone can now consume it with `angreal init oci://ghcr.io/acme/py-template:1.2.0`.

### Wrap publishing in a task

```python
import os
import angreal
from angreal.integrations.oci import Oci

@angreal.command(name="publish", about="Publish this template to a registry")
@angreal.argument(name="version", long="version", takes_value=True, required=True)
def publish(version: str):
    """Package ./template and push it as an OCI artifact."""
    client = Oci()
    client.login("ghcr.io", os.environ["GHCR_USER"], os.environ["GHCR_TOKEN"])
    client.push(f"oci://ghcr.io/acme/py-template:{version}", "./template")
    print(f"published py-template:{version}")
```

## Inspect available versions

List the tags published for a repository:

```python
from angreal.integrations.oci import Oci

for tag in Oci().tags("oci://ghcr.io/acme/py-template"):
    print(tag)
```

## Try it locally

You can exercise the whole loop against a throwaway registry:

```bash
# Start a local registry
docker run -d --rm -p 5000:5000 registry:2
```

```python
from angreal.integrations.oci import Oci

client = Oci()
# Push a local template directory (must contain angreal.toml)
client.push("oci://localhost:5000/demo/template:v1", "./template")
```

```bash
# Consume it
angreal init oci://localhost:5000/demo/template:v1
```

## How it is stored

An angreal template artifact is a standard OCI image manifest: a small JSON
config blob plus a single gzipped-tar layer of the template directory. See the
[OCI integration reference](/angreal/reference/python-api/integrations/oci) for
the exact media types — they make artifacts angreal pushes interoperable with
`oras` and `skopeo`.

## See Also

- [OCI Registry Integration reference](/angreal/reference/python-api/integrations/oci)
- [Create Templates](/angreal/how-to-guides/create-templates)
- [angreal init Behavior](/angreal/explanation/angreal_init_behaviour)
