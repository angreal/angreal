---
title: Command Group
weight: 20
---

# Command Group

Angreal provides two ways to create command groups for organizing related commands.

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
