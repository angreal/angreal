---
title: "Virtual Environment Integration (UV-Powered)"
weight: 1
---

# angreal.integrations.venv

Ultra-fast virtual environment and package management powered by UV (ultrafast Python package installer).

## Overview

Angreal's virtual environment integration uses UV to provide 10-50x performance improvements over traditional Python virtual environment tools. UV is automatically installed when first used, requiring no additional setup.

**Performance Benefits:**
- Virtual environment creation: ~10x faster than venv
- Package installation: ~50x faster than pip
- Overall workflow: 3x faster execution times

## Installation Requirements

UV is automatically installed when first used. No manual installation required.

**Supported Platforms:**
- macOS (via curl)
- Linux (via curl)
- Windows (via PowerShell)

## Functions

### venv_required

Decorator that wraps a function in a virtual environment before execution. The virtual environment is activated before the function runs and deactivated afterward, ensuring packages installed in the venv are available for import.

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
    # These imports work because the venv is activated
    import requests
    import pandas as pd

    # Your code here - runs in the activated virtual environment
    df = pd.DataFrame({'data': [1, 2, 3]})
    response = requests.get('https://api.example.com')

# After the function completes, the original environment is restored
```

## Classes

### VirtualEnv

Manage virtual environments using UV's ultra-fast operations.

```python
from angreal.integrations.venv import VirtualEnv

venv = VirtualEnv(path="/path/to/venv")
```

#### Constructor

```python
VirtualEnv(path, python=None, requirements=None, now=True)
```

**Parameters:**
- `path` (str | Path): The path to the virtual environment. Relative paths are resolved to absolute paths based on the current working directory.
- `python` (str, optional): Python version to use (e.g., "3.11", "3.12")
- `requirements` (str | List[str], optional): Requirements to install
- `now` (bool, optional): Create environment immediately. Defaults to True.

#### Properties

##### exists

Check if the virtual environment exists.

```python
@property
def exists(self) -> bool
```

**Returns:**
- `bool`: True if the virtual environment exists, False otherwise

##### python_executable

Get the Python executable path for this virtual environment.

```python
@property
def python_executable(self) -> Path
```

**Returns:**
- `Path`: Path to the Python executable

#### Instance Methods

##### install_requirements

Install the requirements set during initialization.

```python
def install_requirements(self) -> None
```

**Raises:**
- `TypeError`: If requirements is not a file path, list, or string
- `RuntimeError`: If UV fails to install the requirements

##### install

Install packages or requirements file.

```python
def install(self, packages: Union[str, List[str], Path]) -> None
```

**Parameters:**
- `packages`: Package names, requirements file path, or list of packages

**Example:**
```python
# Install single package
venv.install("requests")

# Install multiple packages
venv.install(["pandas", "numpy", "matplotlib"])

# Install from requirements file
venv.install("requirements.txt")
```

##### activate

Activate the virtual environment in the current Python process.

```python
def activate(self) -> None
```

This method modifies `sys.prefix` and `sys.path` to make the virtual environment's packages available for import. Unlike shell activation, this affects only the current Python process.

**Example:**
```python
venv = VirtualEnv("myenv", now=True)
venv.install("requests")

# Before activation, requests might not be importable
venv.activate()

# Now requests can be imported
import requests
response = requests.get("https://api.github.com")

# Remember to deactivate when done
venv.deactivate()
```

**Raises:**
- `RuntimeError`: If the virtual environment does not exist

##### deactivate

Restore the original Python environment.

```python
def deactivate(self) -> None
```

This method restores `sys.prefix` and `sys.path` to their original values before activation. Safe to call multiple times.

**Example:**
```python
venv = VirtualEnv("myenv", now=True)
venv.activate()
# Use the virtual environment
venv.deactivate()  # Restore original environment
```

#### Context Manager Support

VirtualEnv supports the context manager protocol for automatic activation/deactivation:

```python
with VirtualEnv("myenv", now=True) as venv:
    venv.install("numpy")

    # Virtual environment is activated here
    import numpy as np
    array = np.array([1, 2, 3])

# Virtual environment is automatically deactivated here
```

#### Static Methods

##### discover_available_pythons

Discover all Python installations on the system using UV.

```python
@staticmethod
def discover_available_pythons() -> List[tuple[str, str]]
```

**Returns:**
- `List[tuple[str, str]]`: List of (version, path) tuples for available Python installations

**Example:**
```python
pythons = VirtualEnv.discover_available_pythons()
for version, path in pythons:
    print(f"Python {version}: {path}")
```

##### ensure_python

Ensure a specific Python version is available, installing if needed.

```python
@staticmethod
def ensure_python(version: str) -> str
```

**Parameters:**
- `version` (str): Python version to ensure (e.g., "3.11", "3.12")

**Returns:**
- `str`: Path to the Python installation

**Example:**
```python
python_path = VirtualEnv.ensure_python("3.11")
print(f"Python 3.11 available at: {python_path}")
```

##### version

Get UV version information.

```python
@staticmethod
def version() -> str
```

**Returns:**
- `str`: UV version string

## Activation Behavior

### How Activation Works

Unlike shell-based virtual environment activation (which modifies environment variables), the VirtualEnv activation methods work within the Python process by:

1. Modifying `sys.prefix` to point to the virtual environment
2. Updating `sys.path` to prioritize the virtual environment's site-packages
3. Preserving the original state for restoration during deactivation

### Activation Notes

- **Process-Only**: Activation only affects the current Python process, not subprocesses or the shell
- **Import Availability**: After activation, packages installed in the venv become importable
- **Multiple Activations**: Only one virtual environment can be active at a time. Activating a second venv will override the first
- **Thread Safety**: Activation modifies global Python state (`sys.prefix`, `sys.path`) and is not thread-safe
- **Subprocess Behavior**: Subprocesses will not inherit the activation - use the venv's Python executable directly for subprocesses

### Limitations

- **No Nested Activation**: The current implementation does not support nested virtual environment activation. Deactivating always restores the original environment, not any previously activated environment
- **No Shell Integration**: This activation does not affect your shell environment or terminal prompt
- **Module Caching**: Python's module import cache may retain modules from before activation. Use `importlib.reload()` if needed

## Examples

### Basic Usage

```python
from angreal.integrations.venv import VirtualEnv

# Create virtual environment with Python 3.11
venv = VirtualEnv("/path/to/myenv", python="3.11")

# Check if it exists
if venv.exists:
    print("Virtual environment is ready")
    print(f"Python executable: {venv.python_executable}")
```

### With Requirements

```python
from angreal.integrations.venv import VirtualEnv

# Create venv with a list of packages
venv = VirtualEnv(
    "/path/to/myenv",
    python="3.11",
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

### Python Version Management

```python
from angreal.integrations.venv import VirtualEnv

# Discover available Python versions
pythons = VirtualEnv.discover_available_pythons()
print("Available Python installations:")
for version, path in pythons:
    print(f"  {version}: {path}")

# Ensure Python 3.12 is available
python_path = VirtualEnv.ensure_python("3.12")
print(f"Python 3.12 ready at: {python_path}")

# Create environment with specific Python version
venv = VirtualEnv("myproject-venv", python="3.12")
```

### Manual Environment Management

```python
from angreal.integrations.venv import VirtualEnv

# Create but don't activate immediately
venv = VirtualEnv("/path/to/env", now=False)

# Check and create if needed manually
if not venv.exists:
    venv._create()

# Install specific packages
venv.install(["flask", "sqlalchemy", "pytest"])

# Install from requirements file
venv.install("requirements.txt")
```

### Programmatic Activation

```python
from angreal.integrations.venv import VirtualEnv
import sys

# Create and set up environment
venv = VirtualEnv("data-science-env", now=True)
venv.install(["numpy", "pandas", "matplotlib"])

print(f"Before activation - sys.prefix: {sys.prefix}")

# Activate the environment
venv.activate()
print(f"After activation - sys.prefix: {sys.prefix}")

# Now we can import the installed packages
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

# Do some work with the packages
data = pd.DataFrame(np.random.randn(100, 4), columns=['A', 'B', 'C', 'D'])
data.plot()

# Deactivate when done
venv.deactivate()
print(f"After deactivation - sys.prefix: {sys.prefix}")

# Using context manager for automatic cleanup
with VirtualEnv("analysis-env", now=True) as venv:
    venv.install("scikit-learn")

    # This import works because we're in the activated environment
    from sklearn.datasets import load_iris
    iris = load_iris()
    print(f"Loaded {len(iris.data)} samples")

# Environment is automatically deactivated here
```

### Performance Monitoring

```python
from angreal.integrations.venv import VirtualEnv
import time

# Check UV version
print(f"Using UV version: {VirtualEnv.version()}")

# Time environment creation (typically <1 second)
start = time.time()
venv = VirtualEnv("fast-env", requirements=["requests"])
end = time.time()
print(f"Environment created in {end - start:.2f} seconds")
```

## Performance Characteristics

### Benchmark Comparisons

| Operation | Traditional Tool | UV | Improvement |
|-----------|------------------|----|-----------  |
| Virtual Environment Creation | ~5-10 seconds | ~0.5-1 second | 10x faster |
| Package Installation (10 packages) | ~30-60 seconds | ~1-2 seconds | 50x faster |
| Requirements File (50 packages) | ~2-5 minutes | ~5-10 seconds | 20-30x faster |

### Memory Usage

UV operations run as separate processes and clean up automatically, resulting in minimal memory overhead compared to in-process package management.

## Error Handling

### Common Errors

**UV Installation Failed:**
```python
RuntimeError: UV installation failed
```
**Solution:** Check network connectivity or install UV manually

**Python Version Not Found:**
```python
RuntimeError: Python 3.11 installed but not found
```
**Solution:** Specify exact version or use `discover_available_pythons()` to see available versions

**Package Installation Failed:**
```python
RuntimeError: Failed to install packages: network timeout
```
**Solution:** Check network connectivity and package names

### Best Practices

1. **Version Specification**: Use specific Python versions (e.g., "3.11") rather than "3" for consistency
2. **Error Handling**: Wrap UV operations in try-catch blocks for production code
3. **Environment Isolation**: Use separate environments for different projects
4. **Requirements Files**: Prefer requirements.txt files for complex dependency specifications

## Migration from Traditional Tools

### From venv + pip

```python
# Traditional approach
import subprocess
import sys

subprocess.run([sys.executable, "-m", "venv", "myenv"])
subprocess.run(["myenv/bin/pip", "install", "requests"])

# Angreal UV approach (much faster)
from angreal.integrations.venv import VirtualEnv

venv = VirtualEnv("myenv", requirements=["requests"])  # Done!
```

### From virtualenv

```python
# Traditional virtualenv
import virtualenv

virtualenv.create_environment("myenv")

# Angreal UV approach
from angreal.integrations.venv import VirtualEnv

venv = VirtualEnv("myenv", python="3.11")
```

## UV Binary Management

### Automatic Installation

UV is automatically installed when:
1. Any VirtualEnv operation is performed
2. The venv module is imported
3. UV is not found in the system PATH

### Manual UV Operations

While not typically needed, you can check UV status:

```python
# This is handled automatically - shown for reference
from angreal import ensure_uv_installed, uv_version

# Ensure UV is available
ensure_uv_installed()

# Check version
version = uv_version()
print(f"UV version: {version}")
```

## Architecture Notes

- **Binary Integration**: Uses UV as external binary for maximum performance and stability
- **Subprocess Safety**: All UV operations use secure subprocess calls with proper argument handling
- **Cross-Platform**: Supports Windows, macOS, and Linux with platform-specific installation methods
- **Backward Compatibility**: 100% compatible with existing venv API - drop-in replacement

## See Also

- [UV Integration Architecture](/angreal/explanation/uv_integration_architecture) - Architectural decisions and implementation details
- [Working with Virtual Environments](/angreal/how-to-guides/work-with-virtual-environments) - How-to guide
- [Python Utils](/angreal/reference/python-api/utils) - Other Python utilities
- [UV Documentation](https://docs.astral.sh/uv/) - Official UV documentation
