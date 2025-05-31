---
title: Work with Virtual Environments (UV-Powered)
weight: 25
---

# Work with Virtual Environments

This guide shows how to work with Angreal's UV-powered virtual environment integration for ultra-fast Python environment management.

## Overview

Angreal uses UV (ultrafast Python package installer) to provide dramatically faster virtual environment operations. UV offers 10-50x performance improvements over traditional tools while maintaining full compatibility.

**Key Benefits:**
- ⚡ 10x faster environment creation
- 🚀 50x faster package installation
- 🔧 Automatic UV installation
- 🔄 Drop-in replacement for existing workflows

## Quick Start

### Basic Environment Creation

```python
from angreal.integrations.venv import VirtualEnv

# Create a virtual environment (fast!)
venv = VirtualEnv("myproject-env")

# Check if it exists
if venv.exists:
    print(f"Environment ready at: {venv.path}")
    print(f"Python executable: {venv.python_executable}")
```

### With Specific Python Version

```python
from angreal.integrations.venv import VirtualEnv

# Create environment with Python 3.11
venv = VirtualEnv("myproject-env", python="3.11")
```

### With Requirements

```python
from angreal.integrations.venv import VirtualEnv

# Install packages from a list
venv = VirtualEnv(
    "myproject-env",
    requirements=["django", "requests", "pandas"]
)

# Or from a requirements file
venv = VirtualEnv(
    "myproject-env",
    requirements="requirements.txt"
)
```

## Using the @venv_required Decorator

### Basic Decorator Usage

The `@venv_required` decorator automatically creates and activates a virtual environment for your function:

```python
from angreal.integrations.venv import venv_required
import angreal

@angreal.command(name="analyze", about="Run data analysis")
@venv_required(".venv", requirements=["pandas", "numpy", "matplotlib"])
def analyze_data():
    """This function runs in an isolated environment."""
    import pandas as pd
    import numpy as np
    import matplotlib.pyplot as plt

    # Your code runs with guaranteed package availability
    data = pd.DataFrame(np.random.rand(100, 4))
    data.plot()
    plt.show()
```

### With Requirements File

```python
@angreal.command(name="test", about="Run project tests")
@venv_required("test-env", requirements="requirements-test.txt")
def run_tests():
    """Run tests in isolated environment."""
    import pytest
    pytest.main(["-v", "tests/"])
```

### Multiple Environments

```python
@angreal.command(name="docs", about="Build documentation")
@venv_required("docs-env", requirements=["sphinx", "furo", "myst-parser"])
def build_docs():
    """Build documentation with specific tools."""
    import subprocess
    subprocess.run(["sphinx-build", "docs", "docs/_build"])

@angreal.command(name="lint", about="Run code linting")
@venv_required("lint-env", requirements=["black", "isort", "flake8"])
def lint_code():
    """Lint code with formatting tools."""
    import subprocess
    subprocess.run(["black", "src/"])
    subprocess.run(["isort", "src/"])
    subprocess.run(["flake8", "src/"])
```

## Advanced Environment Management

### Managing Python Versions

```python
from angreal.integrations.venv import VirtualEnv

# Discover available Python versions
pythons = VirtualEnv.discover_available_pythons()
print("Available Python installations:")
for version, path in pythons:
    print(f"  {version}: {path}")

# Ensure specific Python version is available
python_path = VirtualEnv.ensure_python("3.12")
print(f"Python 3.12 available at: {python_path}")

# Create environment with ensured Python version
venv = VirtualEnv("py312-env", python="3.12")
```

### Manual Package Installation

```python
from angreal.integrations.venv import VirtualEnv

# Create environment without requirements
venv = VirtualEnv("manual-env", now=True)

# Install packages as needed
venv.install("requests")
venv.install(["pandas", "numpy"])
venv.install("requirements.txt")

# Mix and match installation methods
venv.install("pytest")  # Single package
venv.install(["black", "isort"])  # Multiple packages
venv.install("dev-requirements.txt")  # Requirements file
```

### Conditional Environment Creation

```python
from angreal.integrations.venv import VirtualEnv
import angreal

@angreal.command(name="setup", about="Set up project environment")
def setup_project():
    """Smart environment setup."""
    venv = VirtualEnv(".venv", now=False)

    if not venv.exists:
        print("Creating new virtual environment...")
        venv = VirtualEnv(".venv", python="3.11")
        print("✓ Environment created")
    else:
        print("✓ Environment already exists")

    # Always ensure requirements are up to date
    print("Installing/updating requirements...")
    venv.install("requirements.txt")
    print("✓ Requirements installed")

    print(f"Environment ready at: {venv.python_executable}")
```

## Real-World Examples

### Django Project Setup

```python
import angreal
from angreal.integrations.venv import VirtualEnv

@angreal.command(name="django-setup", about="Set up Django project")
def setup_django():
    """Set up a Django project with proper environment."""
    # Create environment with Django and common packages
    venv = VirtualEnv(
        "django-env",
        python="3.11",
        requirements=[
            "django>=4.2",
            "psycopg2-binary",  # PostgreSQL support
            "pillow",           # Image handling
            "django-environ",   # Environment variables
            "pytest-django",    # Testing
        ]
    )

    print(f"Django environment ready at: {venv.path}")
    print("Run: source django-env/bin/activate")
    print("Then: django-admin startproject myproject")
```

### Data Science Workflow

```python
import angreal
from angreal.integrations.venv import venv_required

@angreal.command(name="analyze", about="Analyze dataset")
@angreal.argument("dataset", help="Path to dataset file")
@venv_required("data-env", requirements=[
    "pandas",
    "numpy",
    "matplotlib",
    "seaborn",
    "jupyter",
    "scikit-learn"
])
def analyze_dataset(dataset):
    """Perform data analysis in isolated environment."""
    import pandas as pd
    import matplotlib.pyplot as plt
    import seaborn as sns

    # Load and analyze data
    df = pd.read_csv(dataset)
    print("Dataset shape:", df.shape)
    print("\nDataset info:")
    print(df.info())

    # Create visualization
    plt.figure(figsize=(10, 6))
    sns.heatmap(df.corr(), annot=True)
    plt.title("Dataset Correlation Matrix")
    plt.tight_layout()
    plt.savefig("correlation_matrix.png")
    print("✓ Correlation matrix saved to correlation_matrix.png")
```

### Testing Environment

```python
import angreal
from angreal.integrations.venv import VirtualEnv

@angreal.command(name="test-setup", about="Set up testing environment")
def setup_testing():
    """Set up comprehensive testing environment."""
    # Create testing environment
    test_env = VirtualEnv(
        "test-env",
        requirements="requirements.txt"  # Base requirements
    )

    # Add testing-specific packages
    test_env.install([
        "pytest",
        "pytest-cov",      # Coverage reporting
        "pytest-xdist",    # Parallel testing
        "pytest-mock",     # Mocking utilities
        "black",           # Code formatting
        "isort",           # Import sorting
        "flake8",          # Linting
    ])

    print("✓ Testing environment ready")
    print(f"Activate with: source {test_env.path}/bin/activate")

@angreal.command(name="test", about="Run tests")
@venv_required("test-env")
def run_tests():
    """Run tests in isolated environment."""
    import subprocess

    # Run tests with coverage
    result = subprocess.run([
        "pytest",
        "--cov=src",
        "--cov-report=html",
        "--cov-report=term",
        "-v"
    ])

    if result.returncode == 0:
        print("✓ All tests passed!")
    else:
        print("✗ Some tests failed")
        exit(result.returncode)
```

## Performance Comparison

### Before (Traditional Tools)

```python
# Traditional approach - slow
import subprocess
import sys

# Create environment (5-10 seconds)
subprocess.run([sys.executable, "-m", "venv", "myenv"])

# Install packages (30-60 seconds for multiple packages)
subprocess.run(["myenv/bin/pip", "install", "pandas", "numpy", "matplotlib"])
```

### After (UV-Powered)

```python
# UV approach - fast!
from angreal.integrations.venv import VirtualEnv

# Create environment + install packages (1-2 seconds total!)
venv = VirtualEnv("myenv", requirements=["pandas", "numpy", "matplotlib"])
```

## Troubleshooting

### UV Installation Issues

If UV installation fails:

```python
from angreal.integrations.venv import VirtualEnv

try:
    venv = VirtualEnv("test-env")
except RuntimeError as e:
    print(f"UV installation failed: {e}")
    print("Try manual installation:")
    print("  macOS/Linux: curl -LsSf https://astral.sh/uv/install.sh | sh")
    print("  Windows: irm https://astral.sh/uv/install.ps1 | iex")
```

### Python Version Issues

```python
from angreal.integrations.venv import VirtualEnv

# Check available Python versions
pythons = VirtualEnv.discover_available_pythons()
if not pythons:
    print("No Python installations found!")
    print("Install Python from https://python.org")
else:
    print("Available Python versions:")
    for version, path in pythons:
        print(f"  {version}: {path}")

    # Use first available version
    version = pythons[0][0]
    if "cpython-" in version:
        version = version.replace("cpython-", "").split(".")[0:2]
        version = ".".join(version)

    venv = VirtualEnv("auto-env", python=version)
```

### Network Issues

```python
from angreal.integrations.venv import VirtualEnv
import time

def create_env_with_retry(path, requirements, max_retries=3):
    """Create environment with retry logic for network issues."""
    for attempt in range(max_retries):
        try:
            venv = VirtualEnv(path, requirements=requirements)
            return venv
        except RuntimeError as e:
            if "network" in str(e).lower() and attempt < max_retries - 1:
                print(f"Network error (attempt {attempt + 1}), retrying in 5 seconds...")
                time.sleep(5)
            else:
                raise

# Usage
venv = create_env_with_retry("myproject", ["requests", "pandas"])
```

## Best Practices

### 1. Use Project-Specific Environments

```python
# Good: Project-specific environment
venv = VirtualEnv("myproject-env")

# Better: Environment in project directory
venv = VirtualEnv(".venv")  # Standard location
```

### 2. Pin Requirements for Reproducibility

```python
# Good: Specific versions in requirements.txt
"""
pandas==2.0.3
numpy==1.24.3
matplotlib==3.7.1
"""

# Usage
venv = VirtualEnv(".venv", requirements="requirements.txt")
```

### 3. Separate Development and Production Requirements

```python
@angreal.command(name="dev-setup", about="Set up development environment")
def setup_dev():
    """Development environment with extra tools."""
    venv = VirtualEnv("dev-env")
    venv.install("requirements.txt")        # Production requirements
    venv.install("requirements-dev.txt")    # Development tools

@angreal.command(name="prod-setup", about="Set up production environment")
def setup_prod():
    """Production environment - minimal packages."""
    venv = VirtualEnv("prod-env", requirements="requirements.txt")
```

### 4. Check Environment Status

```python
from angreal.integrations.venv import VirtualEnv

def check_environment(path):
    """Check environment status and health."""
    venv = VirtualEnv(path, now=False)

    if not venv.exists:
        print(f"❌ Environment {path} does not exist")
        return False

    print(f"✅ Environment {path} exists")
    print(f"   Python: {venv.python_executable}")
    print(f"   UV version: {VirtualEnv.version()}")
    return True

# Usage
if check_environment(".venv"):
    print("Environment is ready!")
```

## Migration Guide

### From Traditional venv

Replace this:
```bash
python -m venv myenv
source myenv/bin/activate
pip install -r requirements.txt
```

With this:
```python
from angreal.integrations.venv import VirtualEnv
venv = VirtualEnv("myenv", requirements="requirements.txt")
```

### From virtualenv

Replace this:
```python
import virtualenv
virtualenv.create_environment("myenv")
```

With this:
```python
from angreal.integrations.venv import VirtualEnv
venv = VirtualEnv("myenv")
```

### From conda

Replace this:
```bash
conda create -n myenv python=3.11
conda activate myenv
conda install pandas numpy
```

With this:
```python
from angreal.integrations.venv import VirtualEnv
venv = VirtualEnv("myenv", python="3.11", requirements=["pandas", "numpy"])
```

## Next Steps

- Read the [Virtual Environment API Reference](/reference/python-api/integrations/venv) for complete API documentation
- Learn about [UV Integration Architecture](/explanation/uv_integration_architecture) for implementation details
- Explore [Creating Tasks](/how-to-guides/create-a-task) to integrate environments with your automation workflows
