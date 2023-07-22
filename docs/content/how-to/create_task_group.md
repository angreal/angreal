---
title: Create a Task
weight: 10

---

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
