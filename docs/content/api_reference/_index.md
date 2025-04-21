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

## Rust-Python Interface

The integration between Rust and Python is handled through PyO3. Key integration points include:

1. **Command Registration**: Python functions decorated with `@angreal.command` are registered with the Rust command system
2. **Argument Handling**: Arguments defined in Python are translated to Rust command arguments
3. **Template Rendering**: Rust handles template rendering with values supplied from Python
4. **Integrations**: Docker, Git, and venv functionality implemented in Rust with Python-friendly wrappers
