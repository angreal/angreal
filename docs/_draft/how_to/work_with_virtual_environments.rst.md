---
title: Interact With `virtualenv`
---

Integration with `virtualenv` centers around two activities :

-   ensuring that a specific environment is activated

``` {.sourceCode .python}
from angreal.integrations.virtual_env import venv_required

@venv_required('virtual_environment')
def only_run_if_active():
    print('Hello World')
```

-   set up a virtual environment

``` {.sourceCode .python}
from angreal.integrations.virtual_env import VirtualEnv

#create a venv using python3 and a requirements file
venv = VirtualEnv(name='test_env', python='python3',requirements='requirements.txt')
```

-   update a current virtual environment

``` {.sourceCode .python}
from angreal.integrations.virtual_env import VirtualEnv

#create a venv using python3 and a requirements file
venv = VirtualEnv(name='test_env')
venv.install_requirements('requirements.txt')
```
