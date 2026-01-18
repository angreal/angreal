---
title: Create a Command Group
weight: 15
---

# Create a Command Group

Command groups allow you to organize related commands under a common namespace.

## Preferred Method

1. Create a command group.
2. Apply the resulting decorator to a command decorator.

```python
import angreal
test = angreal.group(name="test")

@test
@angreal.command(name="command", about="a test command")
def command_function():
    pass
```

## Alternative Method

You can directly invoke a group decorator on a command decorator:

```python
import angreal

@angreal.group(name="test_", about="commands for testing")
@angreal.command(name="command", about="a test command")
def command_function():
    pass
```
