---
title: Work with Virtual Environments
weight: 15
---

# Work with Virtual Environments


Angreal provides virtual environment management using UV, a fast Python package installer and resolver.

## Overview

The virtual environment integration provides:

- Create and manage virtual environments
- Install packages and requirements
- Ensure specific Python versions
- Automatic environment activation for tasks
- Programmatic activation within Python processes

## Basic Usage


```python
from angreal.integrations.venv import VirtualEnv


# Create a new environment
venv = VirtualEnv("myproject-env")

# Install packages
venv.install(["django", "requests"])

# Install from requirements file
venv.install("requirements.txt")
```

## Activating Virtual Environments

### Manual Activation

```python
from angreal.integrations.venv import VirtualEnv

# Create and activate a virtual environment
venv = VirtualEnv("data-env", now=True)
venv.install(["numpy", "pandas"])

# Activate the environment
venv.activate()

# Now you can import the installed packages
import numpy as np
import pandas as pd

# Remember to deactivate when done
venv.deactivate()
```

### Using Context Manager

```python
from angreal.integrations.venv import VirtualEnv

# Automatic activation/deactivation with context manager
with VirtualEnv("analysis-env", now=True) as venv:
    venv.install("scikit-learn")

    # Environment is activated here
    from sklearn import datasets
    iris = datasets.load_iris()

# Environment is automatically deactivated here
```

## Using with Tasks


```python
from angreal.integrations.venv import venv_required
import angreal

@angreal.command(name="analyze", about="Run data analysis")
@venv_required(".venv", requirements=["pandas", "numpy"])
def analyze_data():
    """This function runs in an isolated environment with packages available."""
    import pandas as pd
    import numpy as np
    # Your code here - the virtual environment is activated
```

## Python Version Management

```python
from angreal.integrations.venv import VirtualEnv


# Create environment with specific Python version
venv = VirtualEnv("py312-env", python="3.12")

# List available Python versions
pythons = VirtualEnv.discover_available_pythons()
for version, path in pythons:
    print(f"{version}: {path}")
```

## Related Documentation

- [Virtual Environment API Reference](/angreal/reference/python-api/integrations/venv) - Complete API documentation
- [Create Tasks](/angreal/how-to-guides/create-a-task) - How to integrate environments with tasks


## Next Steps

- Read the [Virtual Environment API Reference](/angreal/reference/python-api/integrations/venv) for complete API documentation
- Learn about [UV Integration Architecture](/angreal/explanation/uv_integration_architecture) for implementation details
- Explore [Creating Tasks](/angreal/how-to-guides/create-a-task) to integrate environments with your automation workflows
