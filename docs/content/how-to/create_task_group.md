---
title: Create a Task Group
weight: 10

---

{{% notice warning %}}
The command group decorator currently is causing issues passing arguments to the wrapped function.
**Do not use when the task requires arguments to function.**
{{% /notice %}}


### Preferred Method
1. Create a command group.
1. Apply the resulting decorator to a command decorator

```python
import angreal
test = angreal.group(name="test")

@test
@angreal.command(name="command", about="a test command")
def noop_function():
    pass

```

### Alternative

1. Directly invoke a group decorator on a command decorator

```python

import angreal

@angreal.group(name="test_",about="commands for testing")
@angreal.command(name="command", about="a test command")
def noop_function():
    pass
```
