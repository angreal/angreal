---
title: How to run task in a virtual environment
---


1. just use a wrapper with provided requirement file
```python

from angreal.integrations.venv import venv_required

@venv_required("path/to/venv",requirements=None)
@angreal.command()
def uses_numpy:
    import numpy
    pass
```


---

## Alternative

1. create a setup task to manage virtual environment creation

```python
@angreal.command(name="setup", about="setup the development environment")
def setup():
    venv_path = os.path.join(angreal.get_root(),'..','.venv')
    # Setup the virtual environment as .venv in the root folder
    venv = VirtualEnv(path=venv_path,now=True,
                      requirements=['numpy'])
    venv.install_requirements()
```

2. use `venv_required` decorator on a command

```python
venv_path = os.path.join(angreal.get_root(),'..','.venv')

@angreal.command(name="do-math", about="do some math")
@venv_required(venv_path)
def setup():
    import numpy as np
    pass
```
