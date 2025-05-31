---
title : Get Root
---


##### get_root() -> str:
> get the path to the root of the current angreal project. Note that this returns the path to the `.angreal` folder, so you will often need to get the parent directory to obtain the actual project root.
```python
import angreal

@angreal.command(name='test-command')
@angreal.argument(name='noop_arg')
def noop_func(noop_arg):
    print(angreal.get_root())
    pass

# invoked with `angreal test-command`
```
