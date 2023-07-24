---
title : Command Group
---

### Command Group Generator
---
{{% notice warning %}}
The command group decorator currently is causing issues passing arguments to the wrapped function.

Do not use when the task requires arguments to function.
{{% /notice %}}

##### command_group(**name**: str, **about**:str=*""*)
> decorator that creates a group decorator that can be re-used

```python
import angreal

test = angreal.command_group(name="test",about="commands for testing")

@test()
@angreal.command(name="command", about="a test command")
def noop_function():
    pass

# invoked with `angreal test command`
```

### Args:
- name (str): the name to be used for the group.
- about (str, optional): A short description of what the command group is for. Defaults to "".

### Group Decorator
---

##### group(**name**: str, **about**:str=*""*)
> decorator that assigns an angreal command to a command group. Can be chained to an arbitrary set of depths.
```python
import angreal

@angreal.group(name="test",about="commands for testing")
@angreal.command(name="command", about="a test command")
def noop_function():
    pass

# invoked with `angreal test command`
```

### Args:
- name (str): the name to be used for the group.
- about (str, optional): A short description of what the command group is for. Defaults to "".
