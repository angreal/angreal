---
title: Python Virtual Environments
---

Angreal provides some basic utilities to define and activate python virtual environments for your tasks.

This may be useful in instances where you have a command that requires specific libraries to be available to it to function properly.


#### venv_required(**path**:str, **requirements**: str,list(str)=*None*) -> None
> wrap a function in a virtual environment before execution. When called, if the virtual environment does not exist - an attempt will be made to create and activate it first.

```python
from angreal.integrations.venv import venv_required

@venv_required("~/.venv", requirements=["pandas","numpy"])
@angreal.command(...)
def command_to_run:
    import pandas as pd
    pd.read_csv(...)
    ...
    return
```
### Args:
- path (str): the path for the virtual environment. (either to load or to be created)
- requirements (str, optional): a string containing a single package, a list containing multiple packages, or a string containing a file path. Defaults to None

---

### VirtualEnv Class


#### VirtualEnv(**path**:str, **requirements**: str,list(str)=*None*, **now**:bool=*True*)
> Interactions with virtual environments from within a currently running script.

```python

from angreal.integrations.venv import VirtualEnv

this_venv = "__test_venv"
requirements = ["numpy","pandas"]
my_venv = VirtualEnv(path=this_venv, requirements=requirements,
                now=True)
```
### Args:
- path (str): the path for the virtual environment. (either to load or to be created)
- requirements (str, optional): a string containing a single package, a list containing multiple packages, or a string containing a file path. Defaults to None
- now: whether or not to create and activate the environment immediately

#### exists -> bool
> a property to test whether or not the environment exists in the filesystem
```python
from angreal.integrations.venv import VirtualEnv

my_venv = VirtualEnv(path=this_venv, requirements=requirements,
                now=True)

if my_venv.exists :
    print("the path {} exists and appears to be a virtual environment.", my_venv.path)
```

#### install_requirements()
> Install the requirements that are set within the VirtualEnv object

```python
from angreal.integrations.venv import VirtualEnv

my_venv = VirtualEnv(path=this_venv, requirements=requirements,
                now=True)

if my_venv.exists :
    my_venv.install_requirements()
```

#### _create()
> Create the described environment

```python
from angreal.integrations.venv import VirtualEnv

my_venv = VirtualEnv(path=this_venv, requirements=requirements,
                now=True)

if not my_venv.exists :
    my_venv._create()

my_venv.install_requirements()

```


#### _activate()
> Activate the described environment

```python
from angreal.integrations.venv import VirtualEnv

my_venv = VirtualEnv(path=this_venv, requirements=requirements,
                now=True)

if not my_venv.exists :
    my_venv._create()
    my_venv.install_requirements()
my_venv._activate()
```
