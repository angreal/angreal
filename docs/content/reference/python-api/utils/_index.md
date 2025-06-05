---
title: Utility Functions
weight: 40
---


Angreal provides utility functions for common operations when working with projects and commands.

## Overview

These utilities help with:

- **Project Structure** - Locating the project root directory
- **Context Access** - Retrieving template variables from project initialization
- **Version Checking** - Ensuring the right Angreal version is used
- **Command Aliasing** - Creating custom command names for white-labeling

## Key Functions

| Function | Description | Documentation |
|----------|-------------|---------------|
| `get_root` | Get the root directory of the Angreal project | [API Reference](get_root) |
| `get_context` | Get the context from angreal.toml used to render the template | [API Reference](get_context) |
| `required_version` | Check for minimum required Angreal version | [API Reference](required_version) |
| `register_entrypoint` | Create a command alias for angreal (white-labeling) | [API Reference](register_entrypoint) |
| `list_entrypoints` | List all registered command aliases | [API Reference](list_entrypoints) |
| `unregister_entrypoint` | Remove a command alias | [API Reference](unregister_entrypoint) |
| `cleanup_entrypoints` | Remove all registered command aliases | [API Reference](cleanup_entrypoints) |

## Examples

### Basic Project Operations

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

### Context Access

```python
import angreal

# Get the context from angreal.toml
context = angreal.get_context()

if context:
    print("Project name:", context.get("project_name", "Not set"))
    print("Version:", context.get("version", "Not set"))
else:
    print("No context found (not in an angreal project)")
```

### Command Aliasing

```python
import angreal

# Create a command alias for white-labeling
angreal.register_entrypoint("mycompany-tool")

# List all registered aliases
aliases = angreal.list_entrypoints()
print("Registered aliases:", aliases)

# Remove an alias
angreal.unregister_entrypoint("mycompany-tool")

# Clean up all aliases
angreal.cleanup_entrypoints()
```

## Related Documentation

<!-- Geekdoc automatically generates child page navigation -->
