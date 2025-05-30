---
title: "Python API Reference"
weight: 2
---

# Python API Reference

The Python API is used to define tasks and work with templates in Angreal.

## Core Modules

### [Commands](/reference/python-api/commands)
Task definition decorators and command groups.

### [Templates](/reference/python-api/templates)
Template rendering and project generation functions.

### [Utils](/reference/python-api/utils)
Utility functions for working with Angreal projects.

## Quick Reference

### Basic Command
```python
import angreal

@angreal.command(name="hello", about="Say hello")
def hello_command():
    print("Hello, World!")
```

### Command with Arguments
```python
@angreal.command(name="greet", about="Greet someone")
@angreal.argument(name="name", help="Name to greet")
def greet_command(name):
    print(f"Hello, {name}!")
```

### Command Groups
```python
group = angreal.command_group(name="tasks", about="Task commands")

@group()
@angreal.command(name="list", about="List tasks")
def list_tasks():
    print("Listing tasks...")
```

## See Also

- [CLI Reference](/reference/cli) - Command-line interface documentation
- [How-to Guides](/how-to-guides) - Practical examples
- [Tutorials](/tutorials) - Step-by-step learning guides
