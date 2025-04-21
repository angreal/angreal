---
title: Command Decorator
weight: 10
---

# Command Decorator

A decorator that identifies a function as an Angreal command.

## Signature

```python
command(name=None, about="", long_about="", **attrs) -> None
```

## Example

```python
import angreal

@angreal.command(name='test-command')
def command_function():
    pass

# invoked with `angreal test-command`
```

## Parameters

- **name** (str, optional): The name to be used to invoke a command. Defaults to the function name.
- **about** (str, optional): A short description of what the command does. Defaults to "".
- **long_about** (str, optional): A longer description of what the command does. Defaults to the docstring on the decorated function.
