---
title: Rust-Python Interface
weight: 20
---

# Rust-Python Interface

This page explains how the Rust core and Python interface interact in Angreal.

## Interface Overview

Angreal is primarily implemented in Rust, with Python providing a convenient interface through PyO3 bindings. The main components include:

1. **Command Registration** - Python decorators register tasks with the Rust command system
2. **Argument Parsing** - Rust code processes command-line arguments and passes them to Python
3. **Template Rendering** - Rust's Tera templating engine handles all template operations

## Example: Command Registration

When you create a command in Python:

```python
@angreal.command(name='hello', about='Say hello')
@angreal.argument(name='name', long='name', takes_value=True, help='Name to greet')
def hello(name='World'):
    print(f"Hello, {name}!")
```

The following happens under the hood:

1. The `@command` decorator calls a Rust function that registers the command metadata
2. The `@argument` decorator registers argument metadata with the command
3. When executed, Rust parses the command-line arguments and calls the Python function

## Key Interface Points

### Python to Rust

1. **Command Registration**
   - Python decorators call Rust functions through PyO3
   - Command and argument metadata is stored in Rust data structures

2. **Function Calls**
   - Python functions are bound to Rust functions
   - Python values are converted and passed to Rust

3. **Logging**
   - Python logging messages are sent to a Rust subscriber for unified logging

### Rust to Python

1. **Command Execution**
   - Rust loads Python task files and executes decorated functions
   - Arguments are converted from Rust types to Python

## Interface Implementation Details

The Rust-Python interface is implemented in several key files:

- `src/task.rs` - Command registration and execution
- `src/builder/command_tree.rs` - Command-line argument processing
- `src/lib.rs` - PyO3 module registration and core functionality
- `src/utils.rs` - Template rendering and file operations
- `src/git.rs` - Git operations
- `src/init.rs` - Template initialization and project setup

Each Python function decorated with `@command` is stored in a global registry in Rust, which is used to build the command-line interface and execute commands. The Python interface primarily serves as a convenient way to interact with Angreal's Rust core functionality.
