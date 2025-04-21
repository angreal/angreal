---
title: Command System Guide
weight: 10
---

# Command System Guide

Angreal's command system allows you to create custom CLI commands for your projects. This guide explains how to use the command decorators to create your own commands.

## Overview

The command system consists of three main components:

1. **Commands** - Basic command units created with `@angreal.command`
2. **Command Groups** - Collections of related commands created with `@angreal.command_group`
3. **Arguments** - Command parameters created with `@angreal.argument`

## Creating a Simple Command

Here's a basic example of creating a command:

```python
import angreal

@angreal.command(name="hello", about="Say hello to someone")
@angreal.argument(name="name", long="name", takes_value=True, help="Name to greet")
def hello_command(name="World"):
    """
    Greet someone by name.

    This command simply prints a greeting message to the console.
    """
    print(f"Hello, {name}!")
```

This creates a command that can be run with:

```bash
angreal hello --name=John
```

## Creating Command Groups

Command groups help organize related commands:

```python
import angreal

# Create a command group
dev = angreal.command_group(name="dev", about="Development commands")

# Add a command to the group
@dev()
@angreal.command(name="build", about="Build the project")
def build_command():
    """
    Build the project for development.
    """
    print("Building the project...")

# Add another command to the same group
@dev()
@angreal.command(name="test", about="Run tests")
def test_command():
    """
    Run the project tests.
    """
    print("Running tests...")
```

These commands can be run with:

```bash
angreal dev build
angreal dev test
```

## Command Arguments

Arguments can be added to commands with various options:

| Option | Description | Example |
|--------|-------------|---------|
| `name` | Argument name | `name="output"` |
| `long` | Long flag name | `long="output"` |
| `short` | Short flag name | `short="o"` |
| `takes_value` | Whether the argument takes a value | `takes_value=True` |
| `help` | Help text | `help="Output file path"` |
| `default_value` | Default value | `default_value="output.txt"` |
| `is_flag` | Boolean flag | `is_flag=True` |

### Example with Multiple Arguments

```python
import angreal

@angreal.command(name="generate", about="Generate a file")
@angreal.argument(name="output", long="output", short="o",
                 takes_value=True, help="Output file path")
@angreal.argument(name="force", long="force", short="f",
                 is_flag=True, help="Overwrite existing file")
@angreal.argument(name="type", long="type",
                 takes_value=True, help="File type")
def generate_command(output="output.txt", force=False, type="txt"):
    """
    Generate a file with the specified options.
    """
    if os.path.exists(output) and not force:
        print(f"File {output} already exists. Use --force to overwrite.")
        return

    print(f"Generating {type} file at {output}...")
    # File generation code goes here
```

## Best Practices

1. **Descriptive Names** - Use clear, descriptive names for commands and arguments
2. **Helpful Documentation** - Add detailed docstrings and help text
3. **Logical Grouping** - Group related commands together
4. **Consistent Style** - Maintain a consistent naming style across commands
5. **Default Values** - Provide sensible defaults for arguments
6. **Validation** - Validate argument values before using them

## Related Documentation

- [command_decorator](command_decorator) - Full API reference for `@angreal.command`
- [command_group](command_group) - Full API reference for `@angreal.command_group`
- [argument_decorator](argument_decorator) - Full API reference for `@angreal.argument`
