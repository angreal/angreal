---
title: "Quick Start"
weight: 1
geekdocCollapseSection: true
---

# Quick Start Guide

Get up and running with Angreal in just a few minutes.

## Installation

Angreal can be installed via pip:

```bash
pip install 'angreal>=2'
```

Or install the latest development version:

```bash
pip install git+https://github.com/angreal/angreal.git
```

## Verify Installation

After installation, verify that Angreal is working:

```bash
angreal --version
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
@angreal.argument(name="name", help="Name to greet", default="World")
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

Angreal can also create projects from templates:

```bash
# Initialize from a GitHub template
angreal init https://github.com/angreal/python.git

# Answer the prompts to customize your project
```

## Next Steps

- Learn more about [creating tasks](/angreal/how-to-guides/create-a-task)
- Explore [project templates](/angreal/tutorials)
- Read the [API reference](/angreal/reference)

{{< button relref="/tutorials/your_first_angreal" >}}Continue to Tutorials{{< /button >}}
