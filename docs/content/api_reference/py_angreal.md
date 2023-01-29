---
title : Python API Reference
---
Angreal's Python API

---
## Decorators:

##### command(**name**: str=*None*, **about**: str=*""*, **long_about**: str=*""*, ****attrs**) -> None:
> decorator that identifies a function as an angreal task
```python
import angreal

@angreal.command(name='test-command')
def noop_func():
    pass
```

### Args:
- name (str, optional): the name to be used to invoke a task. Defaults to the function name.  
- about (str, optional): A short description of what the task does. Defaults to "".
- long_about (str, optional): A longer description of what the task does. Defaults to the docstring on the decorated function.

    


##### argument(**name**, **python_type**: str=*"str"*, **takes_value**: bool=*True*, **default_value**: str=*None*, **require_equals**: bool=*None*, **multiple_values**: bool=*None*, **number_of_values**: int=*None*, **max_values**: int=*None*, **min_values**: int=*None*, **short**: str=*None*, **long**: str=*None*, **long_help**: str=*None*, **help**: str=*None*, **required**: bool=*None*, ****kwargs**) -> None:
> decorator that adds an argument to an angreal task

```python
import angreal

@angreal.command(name='test-command')
@angreal.argument(name='noop_arg')
def noop_func(noop_arg):
    pass

```
### Args:
- name (str): the argument name, must match a corresponding function argument
- python_type (str, optional): the python type to pass the value as. Must be one of ("str","int","float") . Defaults to "str".
- takes_value (bool, optional): doest the argument consume a trailing value. Defaults to True.
- default_value (str, optional): The default value to apply if none is provided. Defaults to None.
- require_equals (bool, optional): The consumed value requires an equal sign (i.e.`--arg=value`). Defaults to None.
- multiple_values (bool, optional): The argument takes multiple values. Defaults to None.
- number_of_values (int, optional): The argument takes a specific number of values. Defaults to None.
- max_values (int, optional): The argument takes at most X values. Defaults to None.
- min_values (int, optional): The argument takes at least X values. Defaults to None.
- short (str, optional): The short (single character) flag for the argument (i.e. `-i in the cli` would be `i`). Defaults to None.
- long (str, optional): The short (single word) flag for the argument (i.e. `--information` in the clie would be `information`). Defaults  to None.
- long_help (str, optional): The help message to display with "long help" is requested with `--help`. Defaults to None.
- help (str, optional): The help message to display when help is requested via `-h`. Defaults to None.
- required (bool, optional): Whether the argument is required or not. Defaults to None.
    


##### get_root() -> str:
> get the path to the root of the current angreal project.

```python
import angreal

@angreal.command(name='test-command')
@angreal.argument(name='noop_arg')
def noop_func(noop_arg):
    print(angreal.get_root())
    pass
```

