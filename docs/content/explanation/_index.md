---
title: "Explanation"
weight: 5
geekdocCollapseSection: true
---

# Understanding Angreal

Conceptual guides that explain the "why" behind Angreal's design and implementation.

## Core Concepts

{{< columns >}}

### [Why Angreal?](/angreal/explanation/why-angreal)
The motivation and philosophy behind the project.

<--->

### [Architecture](/angreal/explanation/architecture)
How Angreal combines Rust and Python for optimal performance and flexibility.

{{< /columns >}}

### [Init Behaviour](/angreal/explanation/init-behaviour)
Deep dive into how `angreal init` works and template resolution.

## Design Decisions

### The Rust + Python Approach

Angreal uses a unique architecture:

1. **Rust Core**: Provides the CLI, performance-critical operations, and Python runtime management
2. **Python Tasks**: Offers flexibility and ease of use for task definitions
3. **Best of Both Worlds**: Performance where it matters, flexibility where you need it

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
4. **Cross-Platform**: Works the same on Windows, macOS, and Linux

## Further Reading

- [Contributing](/angreal/contributing) - How to contribute to Angreal
- [API Reference](/angreal/reference) - Technical details
- [Tutorials](/angreal/tutorials) - Learn by example
