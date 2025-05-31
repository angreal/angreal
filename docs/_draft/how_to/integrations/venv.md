---
title: "Virtual Environment Integration"
weight: 2
---

# angreal.integrations.venv

Integration with Python virtual environments.

## Functions

### venv_required

Decorator that wraps a function in a virtual environment before execution.

```python
@venv_required(path, requirements=None)
```

**Parameters:**
- `path` (str): The path to the virtual environment (or where it should be created if it doesn't exist)
- `requirements` (str | List[str], optional): A string containing a single module, a list of module names, or a string containing a requirements file path. Defaults to None.

**Example:**
```python
from angreal.integrations.venv import venv_required

@venv_required("/path/to/venv", requirements=["requests", "pandas"])
def process_data():
    import requests
    import pandas as pd
    # Your code here
```

## Classes

### VirtualEnv

Manage virtual environments from within a currently running script.

```python
from angreal.integrations.venv import VirtualEnv

venv = VirtualEnv(path="/path/to/venv")
```

#### Constructor

```python
VirtualEnv(path, requirements=None, now=True)
```

**Parameters:**
- `path` (str): The path to the virtual environment
- `requirements` (str | List[str], optional): A string containing a single module, a list of module names, or a string containing a requirements file path. Defaults to None.
- `now` (bool, optional): Should the environment be created/activated on initialization. Defaults to True.

#### Properties

##### exists

Check if the virtual environment exists.

```python
@property
def exists(self) -> bool
```

**Returns:**
- `bool`: True if the virtual environment exists, False otherwise

#### Methods

##### install_requirements

Install the requirements set during initialization.

```python
def install_requirements(self)
```

**Raises:**
- `TypeError`: If requirements is not a file path, list, or string
- `EnvironmentError`: If pip fails to install the requirements

## Examples

### Basic Usage

```python
from angreal.integrations.venv import VirtualEnv

# Create and activate a virtual environment
venv = VirtualEnv("/path/to/myenv")

# Check if it exists
if venv.exists:
    print("Virtual environment is ready")
```

### With Requirements

```python
from angreal.integrations.venv import VirtualEnv

# Create venv with a list of packages
venv = VirtualEnv(
    "/path/to/myenv",
    requirements=["django", "pillow", "requests"]
)
venv.install_requirements()

# Or with a requirements file
venv = VirtualEnv(
    "/path/to/myenv",
    requirements="requirements.txt"
)
venv.install_requirements()
```

### Using the Decorator

```python
from angreal.integrations.venv import venv_required
import angreal

@angreal.command(name="analyze", about="Analyze data with pandas")
@venv_required(".venv", requirements=["pandas", "numpy"])
def analyze_command():
    """This command runs in an isolated virtual environment."""
    import pandas as pd
    import numpy as np

    # Your analysis code here
    data = pd.DataFrame(np.random.rand(100, 4))
    print(data.describe())
```

### Manual Environment Management

```python
from angreal.integrations.venv import VirtualEnv

# Create but don't activate immediately
venv = VirtualEnv("/path/to/env", now=False)

# Check and create if needed
if not venv.exists:
    venv._create()

# Activate when ready
venv._activate()

# Install specific requirements
venv.requirements = ["flask", "sqlalchemy"]
venv.install_requirements()
```

## Notes

- The virtual environment is activated by modifying `sys.path` and `sys.prefix`
- Windows and Unix-like systems are both supported
- The base path for virtual environments defaults to `~/.venv`
- When using the `venv_required` decorator, the original `sys.prefix` is restored after function execution

## See Also

- [Working with Virtual Environments](/how-to-guides/work-with-virtual-environments) - How-to guide
- [Python Utils](/reference/python-api/utils) - Other Python utilities
