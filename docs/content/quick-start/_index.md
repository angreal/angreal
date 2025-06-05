---
title: "Quick Start"
weight: 1
geekdocCollapseSection: true
---

# Quick Start Guide

Get up and running with Angreal in just a few minutes.

## Prerequisites

- Python 3.8 or higher
- pip (Python package installer)
- Git (for template installation)

## Installation

### From PyPI (Recommended)

Install the latest stable version:

```bash
pip install angreal
```

### From Source

Install the latest development version:

```bash
pip install git+https://github.com/angreal/angreal.git
```

### Development Installation

For contributing to Angreal:

```bash
# Clone the repository
git clone https://github.com/angreal/angreal.git
cd angreal

# Angreal uses itself for development setup!
pip install angreal  # Install angreal first
angreal dev install  # Set up development environment

# This will:
# - Create a .venv virtual environment
# - Install maturin, pre-commit, and pytest
# - Set up pre-commit hooks
# - Check for required system dependencies (Hugo, Cargo)
```

## Verify Installation

After installation, verify that Angreal is working:

```bash
angreal --version
# Expected output: angreal 2.2.0 (or current version)
```

You can also check available commands:

```bash
angreal --help
```

## Your First Task

1. Create a new directory for your project:

```bash
mkdir my-project
cd my-project
```

2. Create an `.angreal` directory:

```bash
mkdir .angreal
```

3. Create your first task file `.angreal/task_hello.py`:

```python
import angreal

@angreal.command(name="hello", about="Say hello")
@angreal.argument(name="name", long="name", help="Name to greet", required=False)
def hello_command(name="World"):
    """A simple hello world task."""
    print(f"Hello, {name}!")
```

4. Run your task:

```bash
angreal hello
# Output: Hello, World!

angreal hello --name Alice
# Output: Hello, Alice!
```

## Using Templates

Angreal can create projects from templates:

```bash
# Initialize from a GitHub template
angreal init https://github.com/angreal/python.git my-new-project

# Or use a local template
angreal init /path/to/template my-new-project

# Answer the prompts to customize your project
```

Popular templates:
- `https://github.com/angreal/python.git` - Python project template

## Troubleshooting

### Command not found
If `angreal` is not found after installation, ensure your Python Scripts directory is in your PATH:
- **Linux/macOS**: `~/.local/bin`
- **Windows**: `%APPDATA%\Python\Scripts`

### ImportError
If you get import errors, ensure you're using Python 3.8+ and have installed all dependencies:
```bash
python --version  # Should be 3.8 or higher
pip install --upgrade angreal
```

## Next Steps

- Learn more about [creating tasks](/angreal/how-to-guides/create-a-task)
- Explore [project templates](/angreal/tutorials)
- Read the [API reference](/angreal/reference)
