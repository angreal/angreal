---
title: "Integrations Module"
weight: 4
---

# Integrations Module

The integrations module provides interfaces to external tools and systems.

## Available Integrations

### Python Integrations

- [`git`](/reference/python-api/integrations/git) - Git version control operations
- [`venv`](/reference/python-api/integrations/venv) - Virtual environment management

### Docker Integration (Rust-based)

The Docker integration is implemented in Rust and exposed to Python through the `angreal._integrations.docker` module. It provides:

- **Container** - Container management operations
- **Containers** - Container collection operations
- **Image** - Docker image management
- **Network** - Docker network management
- **Volume** - Docker volume management

{{< hint type=info >}}
**Note**: The Docker integration is implemented in Rust for performance and type safety. The Python API provides a thin wrapper around the Rust implementation.
{{< /hint >}}

## Usage Examples

### Git Integration

```python
from angreal.integrations.git import Git

git = Git()
git.add(".")
git.commit(m="Update documentation")
git.push("origin", "main")
```

### Virtual Environment Integration

```python
from angreal.integrations.venv import VirtualEnv

venv = VirtualEnv("/path/to/env", requirements=["requests"])
venv.install_requirements()
```

### Docker Integration

```python
from angreal.integrations.docker import Container, Image

# Work with containers
container = Container("my-container")
container.start()

# Work with images
image = Image("python:3.9")
image.pull()
```

## See Also

- [How-to Guides](/how-to-guides) - Practical integration examples
- [Docker Integration Guide](/how-to-guides/use-docker-integration) - Docker-specific guide
- [Virtual Environment Guide](/how-to-guides/work-with-virtual-environments) - Venv guide
