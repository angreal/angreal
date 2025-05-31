---
title: "Angreal Documentation"
geekdocAnchor: false
geekdocBreadcrumb: false
---

# Angreal

**Task automation and project templating that scales**

Angreal combines the flexibility of Python with the performance of Rust to help developers automate repetitive tasks and maintain consistent project structures.

## What is Angreal?

Angreal solves the problem of "doing things often enough that they need automation, but not so often that you remember all the steps." It lets you remember by forgetting—you only need to remember the command, not the implementation details.

### Core Features

- **Task Automation** - Define custom commands using Python decorators
- **Project Templates** - Create reusable project scaffolds with Jinja2 templating
- **Cross-Platform** - Works seamlessly on Windows, macOS, and Linux
- **Fast & Reliable** - Rust-powered core with Python flexibility
- **Project-Local** - Tasks and configuration travel with your project

## Why Angreal?

The original Angreal was built on Python modules that created dependency conflicts. Version 2.0 is a complete rewrite using Rust for the core operations, eliminating dependency hell while maintaining the ease of Python for task definitions.

## Quick Example

```bash
# Install Angreal
pip install 'angreal>=2'

# Initialize from a template
angreal init https://github.com/angreal/python.git

# Define a custom task in .angreal/tasks.py
@angreal.command(name="test", about="Run tests")
def run_tests():
    subprocess.run(["pytest", "tests/"])

# Run your task
angreal test
```

## Documentation Navigation

Use the sidebar to explore:

- **Quick Start** - Get up and running in minutes
- **Tutorials** - Learn by building real projects
- **How-to Guides** - Solve specific problems
- **Reference** - Complete API and CLI documentation
- **Explanation** - Understand the design and philosophy
