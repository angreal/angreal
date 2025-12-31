---
title: Command Decorator
weight: 10
---

# Command Decorator

A decorator that identifies a function as an Angreal command.

## Signature

```python
command(name=None, about="", long_about="", tool=None, **attrs) -> None
```

## Example

```python
import angreal

@angreal.command(
    name='test-command',
    about='Run the test suite',
    tool=angreal.ToolDescription("""
Run the complete test suite for the project.

## When to use
- After code changes
- Before committing
- During CI/CD

## When NOT to use
- In production environments
- When tests are known to be broken

## Examples
```
angreal test-command
angreal test-command --verbose
```
""", risk_level="safe")
)
def command_function():
    pass

# invoked with `angreal test-command`
```

## Parameters

- **name** (str, optional): The name to be used to invoke a command. Defaults to the function name.
- **about** (str, optional): A short description of what the command does. Defaults to "".
- **long_about** (str, optional): A longer description of what the command does. Defaults to the docstring on the decorated function.
- **tool** (ToolDescription, optional): Rich description for MCP/AI agent integration. Includes prose guidance and risk level annotation. See [ToolDescription](#tooldescription) below.

## ToolDescription

The `ToolDescription` class provides rich metadata for AI agent integration via MCP:

```python
angreal.ToolDescription(description, risk_level="safe")
```

**Parameters:**
- **description** (str): Prose description with markdown formatting. Include "When to use", "When NOT to use", and "Examples" sections.
- **risk_level** (str): One of "safe", "read_only", or "destructive". Maps to MCP tool annotations.

**Risk Levels:**
| Level | Meaning | Use For |
|-------|---------|---------|
| `safe` | No destructive effects | Build, test, lint tasks |
| `read_only` | Only reads/reports | Status checks, info gathering |
| `destructive` | May modify or delete | Deploy, clean, database migrations |
