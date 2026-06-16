---
title: Create a Command Group
weight: 15
---

# Create a Command Group

Command groups allow you to organize related commands under a common namespace.

## Preferred Method

1. Create a reusable command group with `command_group`.
2. Apply the resulting decorator (with parentheses) above a command decorator.

```python
import angreal

test = angreal.command_group(name="test", about="commands for testing")

@test()
@angreal.command(name="command", about="a test command")
def command_function():
    pass

# invoked with `angreal test command`
```

The group decorator must sit **above** `@angreal.command` (the command decorator stays innermost). Applying the bare form `@test` instead of `@test()` also works identically.

## Alternative Method

You can apply a group decorator directly without first binding it to a variable. `angreal.command_group(...)` and `angreal.group(...)` return the same kind of group decorator, so either is fine here:

```python
import angreal

@angreal.group(name="test", about="commands for testing")
@angreal.command(name="command", about="a test command")
def command_function():
    pass
```
