---
title: Create a Command
weight: 10
---

# Create a Command

1. Within the `.angreal` folder of a project create a file that starts with `task_` and ends with `.py`.

2. Define a function.

3. Apply `command` decorator.

## Example

```python
import angreal

@angreal.command(name='command-name', about='text-to-display')
def command_function():
    return
```
