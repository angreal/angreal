---
title: Command Decorator
weight: 10
---

# Command Decorator

A decorator that identifies a function as an Angreal command.

## Signature

```python
command(name=None, about="", long_about="", when_to_use=None, when_not_to_use=None, **attrs) -> None
```

## Example

```python
import angreal

@angreal.command(
    name='test-command',
    about='Run the test suite',
    when_to_use=['After code changes', 'Before committing', 'During CI/CD'],
    when_not_to_use=['In production environments', 'When tests are known to be broken']
)
def command_function():
    pass

# invoked with `angreal test-command`
```

## Parameters

- **name** (str, optional): The name to be used to invoke a command. Defaults to the function name.
- **about** (str, optional): A short description of what the command does. Defaults to "".
- **long_about** (str, optional): A longer description of what the command does. Defaults to the docstring on the decorated function.
- **when_to_use** (List[str], optional): List of scenarios when this command should be used. Used for AI agent guidance and documentation.
- **when_not_to_use** (List[str], optional): List of scenarios when this command should not be used. Used for AI agent guidance and documentation.
