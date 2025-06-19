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
    about='text-to-display',
    when_to_use=['During development', 'For testing features'],
    when_not_to_use=['In production deployments', 'When debugging']
)
def command_function():
    return
```

## MCP Integration Fields

The `when_to_use` and `when_not_to_use` fields provide guidance for AI agents and tools about appropriate usage contexts:

- **when_to_use**: List scenarios where this command is appropriate
- **when_not_to_use**: List scenarios where this command should be avoided

These fields enhance the machine-readable command tree output and improve AI agent decision-making.
