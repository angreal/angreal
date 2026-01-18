---
title: "Explanation"
weight: 5
geekdocCollapseSection: true
---

# Understanding Angreal

Conceptual guides that explain the "why" behind Angreal's design and implementation.

## High Level Philosophy

### The Rust + Python Approach

Angreal uses a unique architecture:

1. **Rust Core**: Provides the CLI, performance-critical operations, and Python runtime management
2. **Python Tasks**: Offers flexibility and ease of use for task definitions
3. **Best of Both Worlds**: Performance where it matters, flexibility where you need it

### Performance-First Integrations

Angreal integrates with high-performance tools for speed-critical operations:

- **UV Integration**: 10-50x faster virtual environment and package management
- **Binary-First Approach**: Leverages fast external tools rather than pure Python implementations
- **Subprocess Safety**: Secure, reliable integration with external binaries

### Template Philosophy

Angreal's templating system is designed around:

- **Portability**: Templates can be shared via Git repositories
- **Flexibility**: Jinja2 templating for dynamic content
- **Consistency**: Standardized structure across all templates

### Task Discovery

Angreal automatically discovers tasks by:

1. Looking for `.angreal/` directories
2. Loading Python files matching `task_*.py`
3. Registering decorated functions as commands

## Key Principles

1. **Convention over Configuration**: Sensible defaults with override options
2. **Progressive Disclosure**: Simple tasks stay simple, complex tasks are possible
3. **Project-Local**: Tasks and configuration travel with the project
4. **Cross-Platform**: Core functionality works consistently across Windows, macOS, and Linux, with task authors able to handle OS-specific needs

## Further Reading

- [Contributing](/angreal/contributing) - How to contribute to Angreal
- [API Reference](/angreal/reference) - Technical details
- [Tutorials](/angreal/tutorials) - Learn by example
