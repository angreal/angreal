---
title: Argument Decorator
weight: 10
---

# Argument Decorator

A decorator that adds an argument to an Angreal command.

## Signature

```python
argument(name, python_type="str", takes_value=True, default_value=None, require_equals=None,
         multiple_values=None, number_of_values=None, max_values=None, min_values=None,
         short=None, long=None, long_help=None, help=None, required=None, **kwargs) -> None
```

## Example

```python
import angreal

@angreal.command(name='test-command')
@angreal.argument(name='noop_arg')
def noop_func(noop_arg):
    pass
```

## Parameters

- **name** (str): The argument name, must match a corresponding function argument
- **python_type** (str, optional): The Python type to pass the value as. Must be one of ("str","int","float"). Defaults to "str".
- **takes_value** (bool, optional): Does the argument consume a trailing value. Defaults to True.
- **default_value** (str, optional): The default value to apply if none is provided. Defaults to None.
- **is_flag** (bool, optional): Is the argument a flag. Defaults to False.
- **require_equals** (bool, optional): The consumed value requires an equal sign (i.e.`--arg=value`). Defaults to None.
- **multiple_values** (bool, optional): The argument takes multiple values. Defaults to None.
- **number_of_values** (int, optional): The argument takes a specific number of values. Defaults to None.
- **max_values** (int, optional): The argument takes at most X values. Defaults to None.
- **min_values** (int, optional): The argument takes at least X values. Defaults to None.
- **short** (str, optional): The short (single character) flag for the argument (i.e. `-i` in the CLI would be `i`). Defaults to None.
- **long** (str, optional): The long (single word) flag for the argument (i.e. `--information` in the CLI would be `information`). Defaults to None.
- **long_help** (str, optional): The help message to display with "long help" is requested with `--help`. Defaults to None.
- **help** (str, optional): The help message to display when help is requested via `-h`. Defaults to None.
- **required** (bool, optional): Whether the argument is required or not. Defaults to None.
