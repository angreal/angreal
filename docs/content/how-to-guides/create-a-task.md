---
title: Create a Command
weight: 10
---

# Create a Command

1. Within the `.angreal` folder of a project create a file that starts with `task_` and ends with `.py`.

2. Define a function.

3. Apply `command` decorator.

## Example

```python
import angreal

@angreal.command(
    name='command-name',
    about='Short description for --help',
    tool=angreal.ToolDescription("""
Detailed description of what the command does.

## When to use
- During development
- For testing features

## When NOT to use
- In production deployments
- When debugging

## Examples
```
angreal command-name
angreal command-name --flag
```
""", risk_level="safe")
)
def command_function():
    return
```

## MCP Integration with ToolDescription

The `tool` parameter accepts a `ToolDescription` object that provides rich guidance for AI agents via MCP:

```python
angreal.ToolDescription(description, risk_level="safe")
```

- **description**: Prose description with markdown. Include "When to use", "When NOT to use", and "Examples" sections.
- **risk_level**: One of `"safe"`, `"read_only"`, or `"destructive"`. Maps to MCP tool annotations.

This enables AI agents to understand when and how to use your commands appropriately.
