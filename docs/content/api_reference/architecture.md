---
title: Architecture
weight: 10
---

# Angreal Architecture

Angreal uses a hybrid architecture combining Rust and Python to provide both performance and usability. This page explains how the different components of the system interact.

## Core Architecture

At its core, Angreal consists of a Rust binary that provides the main functionality, with Python bindings that make it easy to create and execute tasks.

{{<mermaid align="center">}}
graph TD;
    A[User] -->|angreal command| B[Python Entry Point]
    B -->|FFI| C[Rust Core Library]
    C --> D[Command Processing]
    C --> E[Template Rendering]
    C --> F[Project Operations]
    F -->|Git Operations| G[Git Repository]
    E -->|File Generation| H[Project Files]
    I[Python Tasks] -->|Register| D
    D -->|Execute| I
{{< /mermaid >}}

## Detailed Component Architecture

This diagram shows the major components of Angreal and their relationships.

{{<mermaid align="center">}}
graph TD;
    A[User] --> B[CLI Entry Point]
    B --> C[Rust Core]
    C --> D[Task Discovery]
    C --> E[Argument Parser]
    C --> F[Command Router]

    F --> G[Built-in Commands]
    F --> H[Python Commands]

    G --> I[Template Engine]
    H --> J[Python Functions]
{{< /mermaid >}}

## Component Responsibilities

### Rust Core

The Rust core handles:

1. **Command-line parsing and execution**: Using clap to define and process commands
2. **Template rendering**: Using Tera (a Rust implementation similar to Jinja2)
3. **File system operations**: Creating project structure, copying files
4. **Git operations**: Cloning templates, managing repositories
5. **Integration with Docker and other tools**

### Python Bindings

The Python layer provides:

1. **User-friendly decorators**: For defining commands and arguments
2. **Task discovery and registration**: Finding and loading task files
3. **Custom Python implementations**: For tasks that are easier to write in Python
4. **Integration with Python tools and libraries**

## Data Flow

When a user runs an Angreal command, the following process occurs:

1. The Python entry point is called
2. Python loads any available task files and registers commands with the Rust core
3. The Rust core parses command-line arguments and determines which command to run
4. If the command is implemented in Rust, it executes directly
5. If the command is implemented in Python, it calls back to the Python function

### Command Execution Flow

This diagram shows the detailed flow of command execution from CLI to execution:

{{<mermaid align="center">}}
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

### Template Rendering Flow

This diagram illustrates the template rendering process during initialization:

{{<mermaid align="center">}}
sequenceDiagram;
    participant User
    participant Init as Init Command
    participant GitModule as Git Module
    participant TemplateRepo as Template Repository
    participant TemplateEngine as Template Engine
    participant FileSystem as File System
    participant PythonInit as Python init.py

    User->>Init: angreal init <template>

    Init->>GitModule: Check template source

    alt Local Template
        GitModule->>Init: Use local path
    else Remote Template
        GitModule->>TemplateRepo: Clone or pull
        TemplateRepo->>GitModule: Return template path
        GitModule->>Init: Return template path
    end

    Init->>TemplateEngine: Read template config

    alt Interactive Mode
        TemplateEngine->>User: Prompt for variables
        User->>TemplateEngine: Provide input values
    else Use Defaults
        TemplateEngine->>TemplateEngine: Load default values
    end

    TemplateEngine->>FileSystem: Render templates to project
    FileSystem->>Init: Return generated paths

    alt Has init.py
        Init->>PythonInit: Execute initialization script
        PythonInit->>Init: Complete initialization
    end

    Init->>User: Display success message
{{< /mermaid >}}

## Memory Management

PyO3 handles memory management between Rust and Python:

1. Rust objects exposed to Python are wrapped in Python objects
2. Python objects passed to Rust are converted to appropriate Rust types
3. Reference counting and garbage collection are handled automatically

## Type Conversions

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

### Concrete Examples of Rust-Python Interaction

#### Example 1: Python Calling Rust Function

Here's how a Python script calls the Rust `render_template` function:

```python
import angreal

# Initialize a new project from a template (calls Rust function)
angreal.main()  # Assumes ARGV contains ['init', 'template-name']
```

The Rust implementation in `init.rs`:

```rust
// Exposed to Python through PyO3
#[pyfunction]
fn main() -> PyResult<()> {
    // Process arguments and call init function
    match sub_command.subcommand() {
        Some(("init", _sub_matches)) => init(
            _sub_matches.value_of("template").unwrap(),
            _sub_matches.is_present("force"),
            _sub_matches.is_present("defaults").not(),
            if _sub_matches.is_present("values_file") {
                Some(_sub_matches.value_of("values_file").unwrap())
            } else {
                None
            },
        ),
        // ...
    }
    Ok(())
}
```

#### Example 2: Rust Calling Python Functions

The Rust code executes Python task functions like this:

```rust
// In task.rs, when executing a command
Python::with_gil(|py| {
    // Create keyword arguments from CLI parameters
    let mut kwargs: Vec<(&str, PyObject)> = Vec::new();

    for arg in args.into_iter() {
        // Convert CLI arguments to Python types
        match arg.python_type.unwrap().as_str() {
            "str" => kwargs.push((arg_name, value.to_object(py))),
            "int" => kwargs.push((arg_name, value.parse::<i32>().unwrap().to_object(py))),
            "float" => kwargs.push((arg_name, value.parse::<f32>().unwrap().to_object(py))),
            _ => kwargs.push((arg_name, value.to_object(py))),
        }
    }

    // Call the Python function with converted arguments
    let result = command.func.call(py, (), Some(kwargs.into_py_dict(py)));
    // Handle result...
})
```

## Integration Architecture

Angreal provides several integrations with external tools that help tasks interact with common development tools:

{{<mermaid align="center">}}
graph LR;
    A[Python Task] --> B[Docker Integration]
    A --> C[Git Integration]
    A --> D[Virtual Environment]

    B --> E[Docker API]
    C --> F[Git CLI]
    D --> G[Python venv]
{{< /mermaid >}}

The integration layer allows Angreal tasks to:

1. **Create and manage Docker containers**: For isolated execution environments
2. **Perform Git operations**: For source control management
3. **Create and manage Python virtual environments**: For isolated Python dependency management

## Extending Angreal

There are two main ways to extend Angreal:

1. **Adding Python tasks**: Create `.py` files in the `.angreal` directory that use the `@angreal.command` decorator
2. **Extending the Rust core**: Add new functionality to the Rust codebase and expose it through PyO3

### Task Registration Flow

{{<mermaid align="center">}}
sequenceDiagram;
    participant A as Python File
    participant B as Decorator
    participant C as Rust Core

    A->>B: Define function
    B->>C: Register command

    Note over A,C: Later when executed...

    C->>A: Call with arguments
{{< /mermaid >}}

When registering a Python task:

```python
import angreal

@angreal.command(name='hello', about='Say hello')
@angreal.argument(name='name', long='name', takes_value=True, help='Name to greet')
def hello(name='World'):
    print(f"Hello, {name}!")
```

The following occurs:

1. The `@angreal.command` decorator registers the function with the Rust command system
2. The `@angreal.argument` decorator adds an argument to the command definition
3. When the command is executed, the Rust code parses arguments and calls the Python function
4. Arguments are converted from Rust types to Python types before the function call

This architecture allows Angreal to have the performance benefits of Rust while maintaining the flexibility and ease of use of Python.
