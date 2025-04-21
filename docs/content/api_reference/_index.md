---
title: API Reference
weight: 50
---

# API Reference

Angreal is built with a hybrid architecture - a Rust core with Python bindings that provide a convenient API for users. This section provides documentation for both the Rust and Python components of Angreal.

## Architecture Overview

Angreal uses [PyO3](https://pyo3.rs/) to create Python bindings for the Rust core functionality. This architecture provides:

1. **Performance**: Critical operations are implemented in Rust for speed and efficiency
2. **Flexibility**: Python API for user-friendly interaction and customization
3. **Stability**: Compiled Rust binary reduces dependency conflicts

### Core Architecture

At its core, Angreal consists of a Rust binary that provides the main functionality, with Python bindings that make it easy to create and execute tasks.

{{< mermaid >}}
graph TD
    A[Python Interface] -->|PyO3 Bindings| B[Rust Core]
    B --> C[Project Templating]
    B --> D[Command Management]
    B --> E[Integrations]
    C --> F[Git Operations]
    C --> G[Template Rendering]
    D --> H[Command Building]
    D --> I[Command Execution]
    E --> J[Docker Integration]
    E --> K[Virtual Environment]
{{< /mermaid >}}

### Component Responsibilities

#### Rust Core
The Rust core handles:
1. **Command-line parsing and execution**: Using clap to define and process commands
2. **Template rendering**: Using Tera (a Rust implementation similar to Jinja2)
3. **File system operations**: Creating project structure, copying files
4. **Git operations**: Cloning templates, managing repositories
5. **Integration with Docker and other tools**

#### Python Bindings
The Python layer provides:
1. **User-friendly decorators**: For defining commands and arguments
2. **Task discovery and registration**: Finding and loading task files
3. **Custom Python implementations**: For tasks that are easier to write in Python
4. **Integration with Python tools and libraries**

### Data Flow

When a user runs an Angreal command, the following process occurs:

1. The Python entry point is called
2. Python loads any available task files and registers commands with the Rust core
3. The Rust core parses command-line arguments and determines which command to run
4. If the command is implemented in Rust, it executes directly
5. If the command is implemented in Python, it calls back to the Python function

{{< mermaid >}}
sequenceDiagram;
    participant User
    participant Python as Python Entry Point
    participant RustCore as Rust Core
    participant TaskRegistry as Task Registry
    participant ArgParser as Argument Parser
    participant PythonTask as Python Task

    User->>Python: Execute CLI command
    Python->>RustCore: Call main() function
    RustCore->>TaskRegistry: Load available tasks
    TaskRegistry->>RustCore: Return registered tasks
    RustCore->>ArgParser: Parse command line args
    ArgParser->>RustCore: Return matched command

    alt Is Built-in Command
        RustCore->>RustCore: Execute directly
    else Is Python Command
        RustCore->>PythonTask: Call with parsed args
        PythonTask->>RustCore: Return result
    end

    RustCore->>Python: Return control
    Python->>User: Display results
{{< /mermaid >}}

### Memory Management and Type Conversions

PyO3 handles memory management between Rust and Python:

1. Rust objects exposed to Python are wrapped in Python objects
2. Python objects passed to Rust are converted to appropriate Rust types
3. Reference counting and garbage collection are handled automatically

Data is converted between Rust and Python types as follows:

| Python Type | Rust Type |
|-------------|-----------|
| `str` | `String` or `&str` |
| `int` | `i32`, `i64`, etc. |
| `float` | `f32`, `f64` |
| `bool` | `bool` |
| `list` | `Vec<T>` |
| `dict` | `HashMap<K, V>` |
| `None` | `Option::None` |

## Templating Language Reference
Templating for this project is provided by the Rust crate [Tera](https://tera.netlify.app/).

Full [documentation](https://tera.netlify.app/docs/) is here, but if you've ever used the Python templating [Jinja2](https://jinja.palletsprojects.com/en/3.1.x/)
before, you should be pretty comfortable already.

## Rust API Reference

Angreal's core functionality is implemented in Rust. Browse the comprehensive Rust API documentation below:

[Rust API Documentation](https://docs.rs/angreal)

This documentation is generated directly from the Rust source code and is kept in sync with the codebase.

## Python API Reference

The Python API provides a user-friendly interface to Angreal's functionality:

[Python API Documentation](py_angreal)

This documentation is automatically generated from Python docstrings and is kept in sync with the codebase.

## Extending Angreal

There are two main ways to extend Angreal:

1. **Adding Python tasks**: Create `.py` files in the `.angreal` directory that use the `@angreal.command` decorator
2. **Extending the Rust core**: Add new functionality to the Rust codebase and expose it through PyO3

### Example: Registering a Python Task

```python
import angreal

@angreal.command(name='hello', about='Say hello')
@angreal.argument(name='name', long='name', takes_value=True, help='Name to greet')
def hello(name='World'):
    print(f"Hello, {name}!")
```

When registering a Python task:
1. The `@angreal.command` decorator registers the function with the Rust command system
2. The `@angreal.argument` decorator adds an argument to the command definition
3. When the command is executed, the Rust code parses arguments and calls the Python function
4. Arguments are converted from Rust types to Python types before the function call
