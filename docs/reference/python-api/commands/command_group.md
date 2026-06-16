---
title: Command Group
weight: 20
---

# Command Group

Angreal provides two ways to create command groups for organizing related commands.

## `command_group` vs `group`

`angreal.command_group(name, about=None)` and `angreal.group(name="default", about=None)` return the **same** type of object: a group decorator. There is no functional difference between them; `command_group` is just the conventional spelling when you want to name a reusable group and bind it to a variable. The two parameters (`name`, `about`) are identical.

The returned group decorator can be applied to a command in either form:

- **With parentheses** — `@mygroup()`
- **Without parentheses** — `@mygroup`

Both work identically. In every case the group decorator must be placed **above** `@angreal.command` (the command decorator must be the innermost decorator); the group decorator looks for the command created by `@angreal.command` and raises a `SyntaxError` if it is applied before one exists.

## Command Group Generator

A function that creates a reusable group decorator.

### Signature

```python
command_group(name, about="")
```

### Example

```python
import angreal

test = angreal.command_group(name="test", about="commands for testing")

@test()
@angreal.command(name="command", about="a test command")
def command_function():
    pass

# invoked with `angreal test command`
```

### Parameters

- **name** (str): The name to be used for the group.
- **about** (str, optional): A short description of what the command group is for. Defaults to "".

## Group Decorator

A decorator that assigns an Angreal command to a command group. Can be chained to an arbitrary set of depths.

### Signature

```python
group(name, about="")
```

### Example

```python
import angreal

@angreal.group(name="test", about="commands for testing")
@angreal.command(name="command", about="a test command")
def command_function():
    pass

# invoked with `angreal test command`
```

### Parameters

- **name** (str): The name to be used for the group.
- **about** (str, optional): A short description of what the command group is for. Defaults to "".
