---
title: "Utils Module"
weight: 3
---

# Utils Module

The utils module provides utility functions for working with Angreal projects.

## Available Functions

- [`get_root()`](/reference/python-api/utils/get_root) - Get the project root directory
- [`get_context()`](/reference/python-api/utils/get_context) - Get template context
- [`required_version()`](/reference/python-api/utils/required_version) - Enforce Angreal version requirements

## Quick Examples

### Get Project Root
```python
import angreal
import os

# Get the project root directory
root = angreal.get_root()
print(f"Project root: {root}")

# Build paths relative to root
config_file = os.path.join(root, "config.yaml")
```

### Check Angreal Version
```python
import angreal

# Require exact version
angreal.required_version("2.0.0")

# Require minimum version
angreal.required_version(">=2.0.0")

# Require version range
angreal.required_version(">=2.0.0,<3.0.0")
```

### Get Template Context
```python
import angreal

# Get context from previous template run
context = angreal.get_context()
print(f"Project name: {context.get('project_name')}")
```

## See Also

- [API Reference](/reference/python-api) - Python API overview
- [How-to Guides](/how-to-guides) - Practical examples
