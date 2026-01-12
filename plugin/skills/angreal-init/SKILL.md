---
name: angreal-init
description: This skill should be used when the user asks to "create an angreal project", "initialize angreal", "set up angreal", "add angreal to project", "start new angreal project", "create .angreal directory", or needs guidance on setting up angreal in a new or existing project, project templates, or initial task file structure.
version: 2.8.0
---

# Initializing Angreal Projects

Set up angreal task automation in a new or existing project.

## What is an Angreal Project?

An angreal project is any directory containing a `.angreal/` subdirectory with task files. Angreal provides development task automation - think of it as your project's `make` or `npm run`.

## Quick Setup

### 1. Create the .angreal Directory

```bash
mkdir .angreal
```

### 2. Create Your First Task File

```python
# .angreal/task_dev.py
import angreal

@angreal.command(name="hello", about="Test angreal setup")
def hello():
    print("Angreal is working!")
    return 0
```

### 3. Verify Setup

```bash
angreal tree    # Should show: hello - Test angreal setup
angreal hello   # Should print: Angreal is working!
```

## Recommended Project Structure

### Minimal Setup

```
my-project/
├── .angreal/
│   └── task_dev.py      # Start with one task file
├── src/
└── ...
```

### Standard Setup

```
my-project/
├── .angreal/
│   ├── task_dev.py      # Development utilities
│   ├── task_test.py     # Testing commands
│   ├── task_build.py    # Build commands
│   └── task_docs.py     # Documentation commands
├── src/
└── ...
```

### With Shared Utilities

```
my-project/
├── .angreal/
│   ├── utils.py         # Shared helper functions
│   ├── task_dev.py
│   ├── task_test.py
│   └── task_build.py
├── src/
└── ...
```

## Starter Templates

### Development Utilities

```python
# .angreal/task_dev.py
import angreal
import subprocess
import os

@angreal.command(name="check-deps", about="Verify development tools")
def check_deps():
    """Check that required tools are installed."""
    tools = ["python", "git"]
    missing = []

    for tool in tools:
        result = subprocess.run(
            ["which", tool],
            capture_output=True
        )
        if result.returncode != 0:
            missing.append(tool)

    if missing:
        print(f"Missing tools: {', '.join(missing)}")
        return 1

    print("All tools available!")
    return 0

@angreal.command(name="setup", about="Set up development environment")
def setup():
    """Initialize development environment."""
    project_root = angreal.get_root().parent

    # Example: Create virtual environment
    venv_path = project_root / ".venv"
    if not venv_path.exists():
        print("Creating virtual environment...")
        subprocess.run(["python", "-m", "venv", str(venv_path)])

    print("Development environment ready!")
    return 0
```

### Testing Commands

```python
# .angreal/task_test.py
import angreal
import subprocess

test = angreal.command_group(name="test", about="Testing commands")

@test()
@angreal.command(name="all", about="Run all tests")
def test_all():
    project_root = angreal.get_root().parent
    result = subprocess.run(
        ["pytest", "-v"],
        cwd=project_root
    )
    return result.returncode

@test()
@angreal.command(name="unit", about="Run unit tests only")
def test_unit():
    project_root = angreal.get_root().parent
    result = subprocess.run(
        ["pytest", "tests/unit", "-v"],
        cwd=project_root
    )
    return result.returncode
```

### Build Commands

```python
# .angreal/task_build.py
import angreal
import subprocess

@angreal.command(
    name="build",
    about="Build the project",
    tool=angreal.ToolDescription("""
        Build project artifacts.

        ## When to use
        - Before releasing
        - Testing production builds

        ## Examples
        ```
        angreal build
        angreal build --release
        ```
        """, risk_level="safe")
)
@angreal.argument(
    name="release",
    long="release",
    is_flag=True,
    takes_value=False,
    help="Build in release mode"
)
def build(release=False):
    project_root = angreal.get_root().parent
    cmd = ["python", "-m", "build"]

    print(f"Building {'release' if release else 'debug'}...")
    result = subprocess.run(cmd, cwd=project_root)
    return result.returncode
```

## Shared Utilities Module

```python
# .angreal/utils.py
import angreal
import subprocess
from pathlib import Path

def get_project_root() -> Path:
    """Return the project root directory."""
    return angreal.get_root().parent

def run_command(cmd, check=True, capture=False):
    """Run a command in the project root."""
    result = subprocess.run(
        cmd,
        cwd=get_project_root(),
        capture_output=capture,
        text=True
    )
    if check and result.returncode != 0:
        raise subprocess.CalledProcessError(
            result.returncode, cmd
        )
    return result

def file_exists(relative_path: str) -> bool:
    """Check if a file exists relative to project root."""
    return (get_project_root() / relative_path).exists()
```

## Adding Angreal to Existing Projects

1. Create `.angreal/` directory at project root
2. Add task files for existing workflows
3. Migrate shell scripts or Makefile targets to angreal tasks
4. Add `.angreal/` to version control

### Migrating from Makefile

**Before (Makefile):**
```makefile
test:
    pytest tests/

build:
    python -m build
```

**After (.angreal/task_build.py):**
```python
import angreal
import subprocess

@angreal.command(name="test", about="Run tests")
def test():
    return subprocess.run(["pytest", "tests/"]).returncode

@angreal.command(name="build", about="Build package")
def build():
    return subprocess.run(["python", "-m", "build"]).returncode
```

## Best Practices

### File Naming

- Use `task_<domain>.py` pattern
- One domain per file: `task_test.py`, `task_build.py`
- Shared code in `utils.py` or `helpers.py`

### Initial Tasks to Create

1. **check-deps** - Verify development tools are installed
2. **setup** - Initialize development environment
3. **test** - Run the test suite
4. **build** - Build artifacts

### Version Control

Add to `.gitignore`:
```
# Don't ignore .angreal/ - it should be versioned
# But ignore any generated files within it
.angreal/__pycache__/
.angreal/*.pyc
```

## Verification Checklist

After setup, verify:

- [ ] `angreal tree` shows your commands
- [ ] `angreal <command>` executes correctly
- [ ] Tasks find project files using `angreal.get_root().parent`
- [ ] Task files are in version control
