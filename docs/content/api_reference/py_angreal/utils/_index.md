---
title: Utility Functions
weight: 40
---


Angreal provides utility functions for common operations when working with projects and commands.

## Overview

These utilities help with:

- **Project Structure** - Locating the project root directory
- **Version Checking** - Ensuring the right Angreal version is used

## Key Functions

| Function | Description | Documentation |
|----------|-------------|---------------|
| `get_root` | Get the root directory of the Angreal project | [API Reference](get_root) |
| `required_version` | Check for minimum required Angreal version | [API Reference](required_version) |

## Example

```python
import angreal
import os

# Check for required version
angreal.required_version("2.0.0")

# Get the project root
root_dir = angreal.get_root()
templates_dir = os.path.join(root_dir, "templates")

print(f"Project root: {root_dir}")
print(f"Templates directory: {templates_dir}")
```

## Related Documentation

<!-- Geekdoc automatically generates child page navigation -->
