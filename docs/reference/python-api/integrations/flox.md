---
title: "Flox Environment Integration"
weight: 30
---

# angreal.integrations.flox

Cross-language development environment and services management powered by Flox.

## Overview

Angreal's Flox integration provides environment activation and services management using Flox, a Nix-based development environment manager. Unlike VirtualEnv (Python-only) or Docker (containerized), Flox provides lightweight, declarative environments that work across multiple languages with optional background services.

**Key Benefits:**
- Cross-language support (Python, Node.js, Rust, Go, etc.)
- Declarative environments via `manifest.toml`
- Built-in services management (databases, caches, etc.)
- Lower overhead than containers
- Reproducible across machines

## Prerequisites

Flox CLI must be installed before using this integration.

**Installation:**
```bash
# macOS
curl -fsSL https://flox.dev/install | bash

# Linux
curl -fsSL https://flox.dev/install | bash

# Verify installation
flox --version
```

**Creating a Flox Environment:**
```bash
cd your-project
flox init
flox install python@3.11 nodejs@20
```

## When to Use Flox

| Use Case | Best Integration |
|----------|------------------|
| Python-only project | VirtualEnv (simplest) |
| Multi-language project | **Flox** (cross-language) |
| Need containerized services | Docker Compose |
| Need lightweight services | **Flox** (process-based) |
| CI/CD reproducibility | **Flox** (declarative) |
| Team-wide consistency | **Flox** (manifest.toml) |

## Functions

### flox_required

Decorator that wraps a function to run in a Flox environment with optional services.

```python
flox_required(path=None, services=None)
```

**Parameters:**
- `path` (str | Path, optional): Path to Flox environment. Defaults to current directory.
- `services` (List[str], optional): Services to start before execution.

**Example:**
```python
from angreal.integrations.flox import flox_required
import angreal

@angreal.command(name="test")
@flox_required(".", services=["postgres"])
def run_tests():
    """Run tests with Flox environment and Postgres service."""
    import subprocess
    subprocess.run(["pytest", "-v"])

# Without services
@angreal.command(name="build")
@flox_required(".")
def build():
    """Build in Flox environment."""
    import subprocess
    subprocess.run(["npm", "run", "build"])
```

The decorator:
1. Activates the Flox environment
2. Starts specified services (if any)
3. Executes the wrapped function
4. Stops services and deactivates environment (even on exception)

## Classes

### Flox

Main class for managing Flox environments.

```python
from angreal.integrations.flox import Flox

flox = Flox(path=".")
```

#### Constructor

```python
Flox(path=None)
```

**Parameters:**
- `path` (str | Path, optional): Path to Flox environment. Defaults to current directory.

#### Properties

##### exists

Check if the Flox environment exists (`.flox/` directory present).

```python
@property
def exists(self) -> bool
```

**Returns:**
- `bool`: True if `.flox/` directory exists

##### has_manifest

Check if the environment has a manifest file.

```python
@property
def has_manifest(self) -> bool
```

**Returns:**
- `bool`: True if `.flox/env/manifest.toml` exists

##### path

Get the environment path.

```python
@property
def path(self) -> Path
```

**Returns:**
- `Path`: Absolute path to the Flox environment

##### services

Get a FloxServices manager for this environment.

```python
@property
def services(self) -> FloxServices
```

**Returns:**
- `FloxServices`: Services manager instance

#### Instance Methods

##### activate

Activate the Flox environment in the current Python process.

```python
def activate(self) -> None
```

Modifies `os.environ` to include Flox environment variables. Changes persist until `deactivate()` is called.

**Raises:**
- `RuntimeError`: If the environment does not exist

**Example:**
```python
flox = Flox(".")
flox.activate()
# Environment variables now include Flox paths
# FLOX_ENV, PATH, etc. are modified
flox.deactivate()
```

##### deactivate

Restore the original environment.

```python
def deactivate(self) -> None
```

Restores all environment variables to their state before activation. Safe to call multiple times.

##### run

Execute a command within the Flox environment.

```python
def run(command: str, args: List[str] = None) -> tuple[int, str, str]
```

**Parameters:**
- `command` (str): Command to execute
- `args` (List[str], optional): Command arguments

**Returns:**
- `tuple[int, str, str]`: (exit_code, stdout, stderr)

**Example:**
```python
flox = Flox(".")
exit_code, stdout, stderr = flox.run("python", ["--version"])
print(f"Python version: {stdout}")
```

#### Context Manager Support

Flox supports the context manager protocol for automatic activation/deactivation:

```python
with Flox(".") as flox:
    # Environment is activated
    exit_code, stdout, _ = flox.run("node", ["--version"])
    print(f"Node version: {stdout}")
# Environment is automatically deactivated
```

#### Class Methods

##### is_available

Check if Flox CLI is installed.

```python
@classmethod
def is_available() -> bool
```

**Returns:**
- `bool`: True if `flox` command is available in PATH

**Example:**
```python
if not Flox.is_available():
    print("Please install Flox: curl -fsSL https://flox.dev/install | bash")
```

##### version

Get Flox version string.

```python
@classmethod
def version() -> str
```

**Returns:**
- `str`: Flox version (e.g., "1.2.0")

**Raises:**
- `RuntimeError`: If Flox is not installed

### FloxServices

Manages Flox services for an environment.

```python
from angreal.integrations.flox import FloxServices

services = FloxServices(".")
# Or via Flox instance
services = Flox(".").services
```

#### Methods

##### start

Start services and return a handle for later cleanup.

```python
def start(*services: str) -> FloxServiceHandle
```

**Parameters:**
- `*services`: Service names to start. If empty, starts all services.

**Returns:**
- `FloxServiceHandle`: Handle for managing started services

**Example:**
```python
services = FloxServices(".")
handle = services.start("postgres", "redis")
# Services are running...
handle.stop()
```

##### stop

Stop all services in the environment.

```python
def stop() -> None
```

##### status

Get status of all services.

```python
def status() -> List[ServiceInfo]
```

**Returns:**
- `List[ServiceInfo]`: List of service status objects

**Example:**
```python
for svc in services.status():
    print(f"{svc.name}: {svc.status} (PID: {svc.pid})")
```

##### logs

Get logs for a service.

```python
def logs(service: str, follow: bool = False, tail: int = None) -> str
```

**Parameters:**
- `service` (str): Service name
- `follow` (bool): Follow log output (default: False)
- `tail` (int, optional): Number of lines from end

**Returns:**
- `str`: Log output

##### restart

Restart services.

```python
def restart(*services: str) -> None
```

**Parameters:**
- `*services`: Service names to restart. If empty, restarts all.

### FloxServiceHandle

Handle for managing started services across sessions.

#### Properties

- `flox_env_path` (Path): Path to the Flox environment
- `services` (List[ServiceInfo]): List of service info objects
- `started_at` (str): ISO timestamp when services were started

#### Methods

##### stop

Stop the services tracked by this handle.

```python
def stop() -> None
```

##### save

Save handle to JSON file for later restoration.

```python
def save(path: str = None) -> None
```

**Parameters:**
- `path` (str, optional): File path. Defaults to `.flox-services.json`

**Example:**
```python
handle = services.start("postgres")
handle.save()  # Saves to .flox-services.json
# Later, in another session...
handle = FloxServiceHandle.load()
handle.stop()
```

##### load (classmethod)

Load handle from JSON file.

```python
@classmethod
def load(path: str = None) -> FloxServiceHandle
```

**Parameters:**
- `path` (str, optional): File path. Defaults to `.flox-services.json`

**Returns:**
- `FloxServiceHandle`: Loaded handle

### ServiceInfo

Information about a service.

#### Attributes

- `name` (str): Service name
- `status` (str): Status (e.g., "Running", "Stopped")
- `pid` (int | None): Process ID if running

#### Methods

##### as_tuple

Convert to tuple format.

```python
def as_tuple() -> tuple[str, str, int | None]
```

**Returns:**
- `tuple`: (name, status, pid)

## Usage Patterns

### Pattern 1: Decorator (Recommended for Tasks)

Best for task-scoped environments with automatic cleanup.

```python
from angreal.integrations.flox import flox_required
import angreal

@angreal.command(name="test")
@flox_required(".", services=["postgres"])
def run_tests():
    """Run tests with database."""
    import subprocess
    subprocess.run(["pytest", "tests/"])
```

### Pattern 2: Context Manager

Best for scoped operations within a function.

```python
from angreal.integrations.flox import Flox

def deploy():
    with Flox(".") as flox:
        flox.run("npm", ["run", "build"])
        flox.run("npm", ["run", "deploy"])
```

### Pattern 3: Explicit Control

Best for long-running sessions or complex workflows.

```python
from angreal.integrations.flox import Flox

flox = Flox(".")
flox.activate()

try:
    # Start services
    handle = flox.services.start("postgres", "redis")
    handle.save()  # Save for later cleanup

    # Do work...
    flox.run("python", ["app.py"])

finally:
    flox.services.stop()
    flox.deactivate()
```

### Pattern 4: Cross-Session Services

For persistent services across multiple task invocations.

```python
# Start services (e.g., at beginning of dev session)
@angreal.command(name="dev-start")
def start_dev():
    flox = Flox(".")
    handle = flox.services.start("postgres", "redis")
    handle.save()
    print("Development services started")

# Stop services (e.g., at end of dev session)
@angreal.command(name="dev-stop")
def stop_dev():
    from angreal.integrations.flox import FloxServiceHandle
    handle = FloxServiceHandle.load()
    handle.stop()
    print("Development services stopped")
```

## Comparison: Flox vs VirtualEnv vs Docker

| Feature | VirtualEnv | Docker Compose | Flox |
|---------|------------|----------------|------|
| **Scope** | Python only | Any language | Any language |
| **Isolation** | Python packages | Full container | Environment vars |
| **Services** | No | Yes (containers) | Yes (processes) |
| **Overhead** | Minimal | High (container runtime) | Low |
| **Startup Time** | ~100ms | ~1-5s | ~200ms |
| **Reproducibility** | requirements.txt | Dockerfile | manifest.toml |
| **Cross-Platform** | Yes | Yes (with Docker) | Yes (with Nix) |
| **Networking** | N/A | Virtual network | Host network |
| **File System** | Shared | Volumes/mounts | Shared |

### When to Use Each

**VirtualEnv:**
- Python-only projects
- Simple dependency management
- Fastest startup time

**Docker Compose:**
- Need container isolation
- Complex service networking
- Production parity
- Already have Docker infrastructure

**Flox:**
- Multi-language projects
- Need services without container overhead
- Reproducible dev environments
- Cross-team consistency

## Troubleshooting

### "Flox environment does not exist"

**Cause:** The `.flox/` directory is missing.

**Solution:**
```bash
cd your-project
flox init
```

### "Failed to get Flox services status: does not have any services"

**Cause:** No services defined in manifest.toml.

**Solution:** Add services to your `.flox/env/manifest.toml`:
```toml
[services.postgres]
command = "postgres -D $PGDATA"

[services.redis]
command = "redis-server"
```

### "Flox CLI not installed"

**Solution:**
```bash
curl -fsSL https://flox.dev/install | bash
```

### Services not starting

**Check service status:**
```python
for svc in flox.services.status():
    print(f"{svc.name}: {svc.status}")
```

**View service logs:**
```python
logs = flox.services.logs("postgres", tail=50)
print(logs)
```

### Environment variables not applied

The `activate()` method modifies `os.environ` in the current process. For subprocesses, use `flox.run()` instead:

```python
# This works - runs in Flox environment
exit_code, stdout, stderr = flox.run("python", ["script.py"])

# This may not inherit Flox environment
import subprocess
subprocess.run(["python", "script.py"])  # May not have Flox vars
```

## Examples

### Full Development Workflow

```python
from angreal.integrations.flox import Flox, flox_required
import angreal

# Check Flox availability at module load
if not Flox.is_available():
    print("Warning: Flox not installed. Some commands may not work.")

@angreal.command(name="setup")
def setup():
    """Initialize Flox environment."""
    import subprocess
    subprocess.run(["flox", "init"])
    subprocess.run(["flox", "install", "python@3.11", "nodejs@20", "postgresql"])

@angreal.command(name="test")
@flox_required(".", services=["postgres"])
def test():
    """Run tests with database."""
    import subprocess
    result = subprocess.run(["pytest", "-v", "tests/"])
    return result.returncode

@angreal.command(name="dev")
@flox_required(".", services=["postgres", "redis"])
def dev():
    """Start development server."""
    import subprocess
    subprocess.run(["python", "manage.py", "runserver"])

@angreal.command(name="build")
@flox_required(".")
def build():
    """Build the project."""
    flox = Flox(".")
    exit_code, stdout, stderr = flox.run("npm", ["run", "build"])
    if exit_code != 0:
        print(f"Build failed: {stderr}")
        return 1
    print("Build successful")
    return 0
```

### Service Status Dashboard

```python
from angreal.integrations.flox import Flox
import angreal

@angreal.command(name="status")
def status():
    """Show Flox environment and service status."""
    flox = Flox(".")

    print(f"Flox version: {Flox.version()}")
    print(f"Environment exists: {flox.exists}")
    print(f"Has manifest: {flox.has_manifest}")
    print(f"Path: {flox.path}")
    print()

    print("Services:")
    for svc in flox.services.status():
        pid_info = f"PID {svc.pid}" if svc.pid else "no PID"
        print(f"  {svc.name}: {svc.status} ({pid_info})")
```

## See Also

- [Virtual Environment Integration](/angreal/reference/python-api/integrations/venv) - Python venv integration
- [Docker Compose Integration](/angreal/reference/python-api/integrations/docker-compose) - Docker integration
- [Flox Documentation](https://flox.dev/docs) - Official Flox documentation
