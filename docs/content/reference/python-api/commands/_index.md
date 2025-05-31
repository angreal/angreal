---
title: Commands
weight: 10
---

# Command System

The command system allows you to create CLI commands for your Angreal projects. Commands are defined using Python decorators and can be grouped into logical categories.

## Overview

Angreal's command system consists of:

- **Commands** - Individual operations that can be executed from the command line
- **Command Groups** - Collections of related commands
- **Arguments** - Parameters that commands can accept

## Key Components

| Component | Description | Documentation |
|-----------|-------------|---------------|
| `@command` | Decorator to define a command | [API Reference](command_decorator) |
| `@command_group` | Decorator to create a group of commands | [API Reference](command_group) |
| `@argument` | Decorator to add arguments to commands | [API Reference](argument_decorator) |

## Comprehensive Guide

For a complete walkthrough of creating commands and arguments, see the [Command System Guide](commands_guide).

## Examples

```python
import angreal

# Define a command group
dev = angreal.command_group(name="dev", about="Development commands")

# Create a command in the group
@dev()
@angreal.command(name="build", about="Build the project")
@angreal.argument(name="target", long="target", takes_value=True,
                 help="Build target", default_value="debug")
def build_command(target):
    """Build the project for the specified target."""
    print(f"Building project for target: {target}")
```

## Related Documentation

<!-- Geekdoc automatically generates child page navigation -->
