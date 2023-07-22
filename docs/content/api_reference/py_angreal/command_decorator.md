---
title: Command Decorator
---

##### command(**name**: str=*None*, **about**: str=*""*, **long_about**: str=*""*, ****attrs**) -> None:
> decorator that identifies a function as an angreal task
```python
import angreal

@angreal.command(name='test-command')
def noop_func():
    pass

# invoked with `angreal test-command`
```

### Args:
- name (str, optional): the name to be used to invoke a task. Defaults to the function name.
- about (str, optional): A short description of what the task does. Defaults to "".
- long_about (str, optional): A longer description of what the task does. Defaults to the docstring on the decorated function.
