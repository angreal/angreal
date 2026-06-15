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

Setting up Angreal for local development is a different workflow — see the
[contributing docs](../contributing/).

## Verify Installation

After installation, verify that Angreal is working. It prints the installed
version number:

```bash
angreal --version
# Expected output: angreal X.Y.Z
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

2. Create an `.angreal` directory — the directory Angreal looks in for your task
files:

```bash
mkdir .angreal
```

3. Create your first task file `.angreal/task_hello.py`. Task files must be named
`task_*.py`:

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

Angreal can create projects from templates. Pass `angreal init` a single
argument — a template URL or a local path — and answer the prompts. The name you
provide becomes your new project's directory:

```bash
# Initialize from a GitHub template
angreal init https://github.com/angreal/python.git

# Or use a local template
angreal init /path/to/template
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
