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

## Basic Usage


```python
from angreal.integrations.venv import VirtualEnv


# Create a new environment
venv = VirtualEnv("myproject-env")

# Install packages
venv.install_packages(["django", "requests"])

# Install from requirements file
venv.install_requirements("requirements.txt")
```

## Using with Tasks


```python
from angreal.integrations.venv import venv_required
import angreal

@angreal.command(name="analyze", about="Run data analysis")
@venv_required(".venv", requirements=["pandas", "numpy"])

def analyze_data():
    """This function runs in an isolated environment."""
    import pandas as pd
    import numpy as np
    # Your code here
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

- [Virtual Environment API Reference](/reference/python-api/integrations/venv) - Complete API documentation
- [Create Tasks](/how-to-guides/create-a-task) - How to integrate environments with tasks


## Next Steps

- Read the [Virtual Environment API Reference](/reference/python-api/integrations/venv) for complete API documentation
- Learn about [UV Integration Architecture](/explanation/uv_integration_architecture) for implementation details
- Explore [Creating Tasks](/how-to-guides/create-a-task) to integrate environments with your automation workflows

