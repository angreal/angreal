---
title: Create a Task
weight: 10

---

1. Within the `.angreal` folder of a project create a file that starts with `task_` and ends with `.py`.

1. Define a function.

1. Apply `command` decorator


```python

import angreal

@angreal.command(name='task-name', about='text-to-display')
def task_code():
    return
```
