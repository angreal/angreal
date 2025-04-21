---
title: venv
---

# venv

angreal.integrations.virtual_env
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Integration to virtualenv

## Functions

### venv_required

```python
venv_required(path, requirements)
```

wrap a function in a virtual environment before execution

Args:
    path (str): The path to the virtual environment (or where the environment
      should be created if it doesn't exist)
    requirements (_type_, optional): A string containing a single module, a
      list of module names, or a string containing a file path. Defaults to None.

## Classes

### VirtualEnv

Interacting with virtual environments from within a currently running script.

Args:
    path (str): the path to the virtual environment
    requirements ([str,List[str]]), optional): A string containing a single module,
      a list of module names, or a string containing a file path. Defaults to None.
    now (bool, optional): should the environment be created/activated
      on initialization. Defaults to True

#### Methods

##### exists

```python
exists(self)
```

Does the virtual environment exist

##### __init__

```python
__init__(self, path, requirements, now)
```

Initializes the object either creating or activating the named environment.

##### install_requirements

```python
install_requirements(self)
```

install requirements the requirements set during initialization.
