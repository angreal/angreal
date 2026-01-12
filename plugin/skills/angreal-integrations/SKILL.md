---
name: angreal-integrations
description: This skill should be used when the user asks to "use Git in a task", "manage virtual environments", "use Docker Compose", "clone a repository", "create a venv", "run docker-compose", "use angreal.integrations", "render a template", "scaffold files", "generate files from template", "use render_template", "use render_directory", or needs guidance on the built-in Git, VirtualEnv, Docker, or Tera templating integrations available in angreal tasks.
version: 2.8.0
---

# Angreal Integrations

Angreal provides built-in integrations for common development tools: Git, virtual environments, Docker Compose, and Tera templating.

## Tera Templating

Generate files and directories using the Tera template engine. Useful for scaffolding code, configs, or any structured files within tasks.

### Import

```python
import angreal
```

### render_template()

Render a template string with variable substitution:

```python
import angreal

# Simple variable substitution
template = "Hello {{ name }}!"
result = angreal.render_template(template, {"name": "World"})
# result == "Hello World!"

# With conditionals and loops
template = """
# {{ project_name }}

{% if use_docker %}
## Docker Setup
Run `docker-compose up` to start.
{% endif %}

## Dependencies
{% for dep in dependencies %}
- {{ dep }}
{% endfor %}
"""

result = angreal.render_template(template, {
    "project_name": "My App",
    "use_docker": True,
    "dependencies": ["requests", "click"]
})
```

**Signature**: `angreal.render_template(template: str, context: dict) -> str`

### render_directory()

Render an entire directory tree, processing both file contents and file/directory names:

```python
import angreal

# Source directory structure:
# templates/
#   {{ module_name }}/
#     __init__.py
#     {{ module_name }}.py

rendered_files = angreal.render_directory(
    "templates",           # src: source directory
    "output",              # dst: destination directory
    False,                 # force: overwrite existing files?
    {"module_name": "mymodule"}  # context: template variables
)

# Creates:
# output/
#   mymodule/
#     __init__.py
#     mymodule.py

print(f"Created {len(rendered_files)} files")

# Context is optional - copy without templating
angreal.render_directory("static_files", "output", False, None)
```

**Signature**: `angreal.render_directory(src: str, dst: str, force: bool, context: dict | None) -> list[str]`

Returns a list of created file paths.

### generate_context()

Load variables from a TOML file, optionally prompting the user:

```python
import angreal

# Load from TOML without prompting
context = angreal.generate_context("config.toml", take_input=False)

# Load and prompt user for values
context = angreal.generate_context("config.toml", take_input=True)
```

**Signature**: `angreal.generate_context(path: str, take_input: bool) -> dict`

### get_context()

Get the context from the current project's `.angreal/angreal.toml`:

```python
import angreal

# Returns dict from .angreal/angreal.toml or empty dict if not found
context = angreal.get_context()
project_name = context.get("project_name", "unknown")
```

**Signature**: `angreal.get_context() -> dict`

### Tera Syntax Quick Reference

```jinja2
{# Comments #}

{{ variable }}                    {# Variable substitution #}
{{ name | upper }}                {# Filters: upper, lower, title, trim #}
{{ items | length }}              {# Get length #}
{{ value | default("fallback") }} {# Default value #}

{% if condition %}                {# Conditionals #}
{% elif other %}
{% else %}
{% endif %}

{% for item in items %}           {# Loops #}
  {{ loop.index }}: {{ item }}
{% endfor %}

{% raw %}{{ not processed }}{% endraw %}  {# Escape template syntax #}
```

### Complete Scaffolding Example

```python
import angreal
import os

@angreal.command(name="scaffold", about="Generate a new module")
@angreal.argument(name="name", long="name", required=True, help="Module name")
@angreal.argument(name="with_tests", long="with-tests", is_flag=True, takes_value=False)
def scaffold(name, with_tests=False):
    project_root = angreal.get_root().parent

    # Template for module file
    module_template = '''"""{{ name }} module."""

class {{ name | title }}:
    """{{ description }}"""

    def __init__(self):
        pass
'''

    # Template for test file
    test_template = '''"""Tests for {{ name }}."""
import pytest
from {{ name }} import {{ name | title }}

def test_{{ name }}_init():
    instance = {{ name | title }}()
    assert instance is not None
'''

    context = {
        "name": name,
        "description": f"The {name} module"
    }

    # Create module
    module_content = angreal.render_template(module_template, context)
    module_path = os.path.join(project_root, "src", f"{name}.py")
    os.makedirs(os.path.dirname(module_path), exist_ok=True)
    with open(module_path, "w") as f:
        f.write(module_content)
    print(f"Created {module_path}")

    # Create test if requested
    if with_tests:
        test_content = angreal.render_template(test_template, context)
        test_path = os.path.join(project_root, "tests", f"test_{name}.py")
        os.makedirs(os.path.dirname(test_path), exist_ok=True)
        with open(test_path, "w") as f:
            f.write(test_content)
        print(f"Created {test_path}")

    return 0
```

## Git Integration

Full-featured Git wrapper for repository operations.

### Import

```python
from angreal.integrations.git import Git, clone
```

### Basic Usage

```python
from angreal.integrations.git import Git

# Initialize with working directory (default: current directory)
git = Git()
git = Git("/path/to/repo")

# All methods return (exit_code, stderr, stdout) tuples
exit_code, stderr, stdout = git.status()
```

### Git Class Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `init` | `init(bare=False)` | Initialize a new repository |
| `add` | `add(*paths)` | Stage files for commit |
| `commit` | `commit(message, all=False)` | Create a commit |
| `push` | `push(remote=None, branch=None)` | Push to remote |
| `pull` | `pull(remote=None, branch=None)` | Pull from remote |
| `status` | `status(short=False)` | Show working tree status |
| `branch` | `branch(name=None, delete=False)` | List or manage branches |
| `checkout` | `checkout(branch, create=False)` | Switch branches |
| `tag` | `tag(name, message=None)` | Create a tag |

### Callable Interface

The Git object is callable for arbitrary git commands:

```python
git = Git()

# Call any git command
exit_code, stderr, stdout = git("log", "--oneline", "-5")
exit_code, stderr, stdout = git("diff", "HEAD~1")
exit_code, stderr, stdout = git("stash", "pop")
```

### Clone Function

```python
from angreal.integrations.git import clone

# Clone a repository
path = clone("https://github.com/user/repo.git")
path = clone("https://github.com/user/repo.git", "/custom/destination")
```

### Complete Example

```python
import angreal
from angreal.integrations.git import Git

@angreal.command(name="release", about="Create a release")
@angreal.argument(name="version", long="version", required=True, help="Version tag")
def release(version):
    git = Git()

    # Check for clean working tree
    exit_code, stderr, stdout = git.status(short=True)
    if stdout.strip():
        print("Error: Working tree not clean")
        return 1

    # Create and push tag
    git.tag(version, message=f"Release {version}")
    git.push("origin", version)

    print(f"Released {version}")
    return 0
```

## VirtualEnv Integration

Manage Python virtual environments with automatic activation.

### Import

```python
from angreal.integrations.venv import VirtualEnv, venv_required
```

### Creating Virtual Environments

```python
from angreal.integrations.venv import VirtualEnv

# Create venv (default path: .venv, created immediately)
venv = VirtualEnv()

# Custom path
venv = VirtualEnv("/path/to/venv")

# Defer creation
venv = VirtualEnv(".venv", now=False)
venv.create()  # Create manually later

# With requirements
venv = VirtualEnv(".venv", requirements="requirements.txt")
venv = VirtualEnv(".venv", requirements=["requests", "click"])
```

### VirtualEnv Constructor

```python
VirtualEnv(
    path=".venv",           # Path to virtual environment
    python=None,            # Python version (e.g., "3.11")
    requirements=None,      # Requirements file or package list
    now=True,               # Create immediately if True
)
```

### VirtualEnv Methods

| Method | Description |
|--------|-------------|
| `create()` | Create the virtual environment |
| `activate()` | Activate the venv in current process |
| `deactivate()` | Deactivate the venv |
| `install(packages)` | Install packages (string, list, or requirements file) |
| `install_requirements()` | Install requirements passed to constructor |
| `remove()` | Delete the virtual environment |

### VirtualEnv Properties

| Property | Type | Description |
|----------|------|-------------|
| `path` | Path | Absolute path to venv directory |
| `name` | str | Name of the venv |
| `python_executable` | Path | Path to Python interpreter |
| `exists` | bool | Whether the venv exists |

### Context Manager Usage

```python
from angreal.integrations.venv import VirtualEnv

# Automatically activate and deactivate
with VirtualEnv(".venv") as venv:
    # venv is activated here
    import subprocess
    subprocess.run(["python", "-c", "import sys; print(sys.prefix)"])
# venv is deactivated here
```

### The @venv_required Decorator

Automatically manage venv lifecycle for a task:

```python
import angreal
from angreal.integrations.venv import venv_required

@angreal.command(name="test", about="Run tests in venv")
@venv_required(".venv", requirements=["pytest"])
def test():
    import subprocess
    # Runs inside activated venv
    subprocess.run(["pytest", "tests/"])
```

### Complete Example

```python
import angreal
from angreal.integrations.venv import VirtualEnv

@angreal.command(name="setup", about="Setup development environment")
def setup():
    venv = VirtualEnv(".venv", requirements="requirements-dev.txt")
    venv.install_requirements()

    print(f"Virtual environment created at {venv.path}")
    print(f"Activate with: source {venv.path}/bin/activate")
    return 0
```

## Docker Compose Integration

Manage Docker Compose services from tasks.

### Import

```python
from angreal.integrations.docker import DockerCompose, compose
```

### Basic Usage

```python
from angreal.integrations.docker import DockerCompose

# Create instance from compose file
dc = DockerCompose("docker-compose.yml")
dc = DockerCompose("docker-compose.yml", project_name="myproject")

# Or use the convenience function
from angreal.integrations.docker import compose
dc = compose("docker-compose.yml")
```

### DockerCompose Methods

All methods return a `ComposeResult` object with: `success`, `exit_code`, `stdout`, `stderr`.

#### Service Lifecycle

```python
# Start services
result = dc.up(detach=True)
result = dc.up(detach=True, build=True, services=["web", "db"])

# Stop and remove
result = dc.down(volumes=True, remove_orphans=True)

# Start/stop without removing
result = dc.start(services=["web"])
result = dc.stop(services=["web"], timeout="30s")

# Restart
result = dc.restart(services=["web"])
```

#### Build and Pull

```python
# Build images
result = dc.build(no_cache=True, parallel=True)
result = dc.build(services=["web"])

# Pull images
result = dc.pull(services=["db"])
```

#### Inspection

```python
# List services
result = dc.ps(all=True)

# View logs
result = dc.logs(services=["web"], follow=False, tail="100")

# Validate config
result = dc.config(quiet=True)
```

#### Execute Commands

```python
# Run command in service container
result = dc.exec(
    "web",
    ["python", "manage.py", "migrate"],
    user="app",
    workdir="/app"
)
```

### Method Signatures

#### up()
```python
dc.up(
    detach=True,           # Run in background
    build=False,           # Build images before starting
    remove_orphans=False,  # Remove containers for undefined services
    force_recreate=False,  # Recreate even if unchanged
    no_recreate=False,     # Don't recreate existing containers
    services=None,         # List of services to start
)
```

#### down()
```python
dc.down(
    volumes=False,         # Remove named volumes
    remove_orphans=False,  # Remove undefined service containers
    timeout=None,          # Shutdown timeout (e.g., "30s")
)
```

#### logs()
```python
dc.logs(
    services=None,         # Services to show logs for
    follow=False,          # Follow log output
    timestamps=False,      # Show timestamps
    tail=None,             # Number of lines (e.g., "100")
    since=None,            # Show logs since timestamp
)
```

#### exec()
```python
dc.exec(
    service,               # Service name
    command,               # Command as list of strings
    detach=False,          # Run in background
    tty=True,              # Allocate pseudo-TTY
    user=None,             # Run as user
    workdir=None,          # Working directory
    env=None,              # Environment variables dict
)
```

### Static Methods

```python
# Check if Docker Compose is available
if DockerCompose.is_available():
    dc = DockerCompose("docker-compose.yml")
```

### Properties

| Property | Type | Description |
|----------|------|-------------|
| `compose_file` | str | Path to compose file |
| `working_dir` | str | Working directory |
| `project_name` | str | Project name (if set) |

### Complete Example

```python
import angreal
from angreal.integrations.docker import DockerCompose

@angreal.command(name="dev", about="Start development environment")
@angreal.argument(name="build", long="build", is_flag=True, takes_value=False)
def dev(build=False):
    dc = DockerCompose("docker-compose.yml", project_name="myapp")

    if not DockerCompose.is_available():
        print("Error: Docker Compose not found")
        return 1

    result = dc.up(detach=True, build=build)
    if not result.success:
        print(f"Failed: {result.stderr}")
        return 1

    print("Services started:")
    dc.ps()
    return 0

@angreal.command(name="logs", about="View service logs")
@angreal.argument(name="service", long="service", help="Service name")
@angreal.argument(name="follow", short="f", long="follow", is_flag=True, takes_value=False)
def logs(service=None, follow=False):
    dc = DockerCompose("docker-compose.yml")
    services = [service] if service else None
    result = dc.logs(services=services, follow=follow, tail="100")
    print(result.stdout)
    return 0
```

## Combining Integrations

```python
import angreal
from angreal.integrations.git import Git
from angreal.integrations.docker import DockerCompose
from angreal.integrations.venv import VirtualEnv

@angreal.command(name="ci", about="Run CI pipeline locally")
def ci():
    git = Git()

    # Ensure clean state
    _, _, status = git.status(short=True)
    if status.strip():
        print("Commit or stash changes first")
        return 1

    # Start services
    dc = DockerCompose("docker-compose.test.yml")
    dc.up(detach=True, build=True)

    try:
        # Run tests in venv
        with VirtualEnv(".venv", requirements="requirements-test.txt") as venv:
            import subprocess
            result = subprocess.run(["pytest", "-v"])
            return result.returncode
    finally:
        dc.down(volumes=True)
```
