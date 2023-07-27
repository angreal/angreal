---
title: Git
---

Angreal provides a very thin layer to an available git binary on the operating system. It is not 
an interface to libgit or libgit2 but an interface to a command line interface. 

#### Git(**git_path**: str=*None*,**working_dir**: str=*None*)
> a hyper light weight wrapper for a git binary

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
> Install the requirements that are set within the VirtaulEnv object

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
