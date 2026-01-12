---
name: angreal-authoring
description: This skill should be used when the user asks to "create an angreal task", "write a task file", "add a command to angreal", "make a new task", "organize tasks with groups", "use @angreal.command", "use command_group", or needs guidance on task file structure, the @command decorator, command groups, naming conventions, or task organization within an existing angreal project.
version: 2.8.0
---

# Authoring Angreal Tasks

Create task files within an existing angreal project. For initializing new projects, see the angreal-init skill.

## Task File Location

Task files live in `.angreal/` at your project root:

```
my-project/
├── .angreal/
│   ├── task_dev.py       # Development tasks
│   ├── task_test.py      # Testing tasks
│   ├── task_docs.py      # Documentation tasks
│   └── utils.py          # Shared utilities (optional)
├── src/
└── ...
```

**Naming convention**: Files must be named `task_*.py` to be discovered.

## The @command Decorator

Every task needs the `@command` decorator:

```python
import angreal

@angreal.command(
    name="build",              # Command name (kebab-case)
    about="Build the project"  # Short description for --help
)
def build():
    print("Building...")
    return 0  # Success
```

### Name Inference

If you omit `name`, it derives from the function:

```python
@angreal.command(about="Check dependencies")
def check_deps():  # Creates command "check-deps"
    pass
```

## Command Groups

Organize related commands with groups:

```python
import angreal

# Create reusable group decorator
test = angreal.command_group(name="test", about="Testing commands")

@test()  # Group decorator FIRST
@angreal.command(name="all", about="Run all tests")
def test_all():
    pass

@test()
@angreal.command(name="unit", about="Run unit tests")
def test_unit():
    pass
```

Creates: `angreal test all`, `angreal test unit`

### Nested Groups

```python
docker = angreal.command_group(name="docker", about="Docker commands")
compose = angreal.command_group(name="compose", about="Compose commands")

@docker()
@compose()
@angreal.command(name="up", about="Start services")
def docker_compose_up():
    pass
```

Creates: `angreal docker compose up`

## Getting Project Root

**Important**: `get_root()` returns `.angreal/` directory, not project root:

```python
import angreal

@angreal.command(name="build", about="Build project")
def build():
    angreal_dir = angreal.get_root()        # .angreal/ directory
    project_root = angreal_dir.parent       # Actual project root
    # Use project_root for file operations
```

## Shared Modules

Create shared utilities in `.angreal/`:

```python
# .angreal/utils.py
import angreal

def get_project_root():
    return angreal.get_root().parent

def run_in_project(cmd):
    import subprocess
    return subprocess.run(cmd, cwd=get_project_root())
```

```python
# .angreal/task_build.py
import angreal
from utils import run_in_project

@angreal.command(name="build", about="Build project")
def build():
    result = run_in_project(["cargo", "build"])
    return result.returncode
```

## Best Practices

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Task files | `task_<domain>.py` | `task_test.py` |
| Commands | kebab-case | `check-deps` |
| Functions | snake_case | `check_deps` |
| Groups | short nouns/verbs | `test`, `dev`, `docs` |

### Error Handling

```python
@angreal.command(name="build", about="Build project")
def build():
    project_root = angreal.get_root().parent

    # Check prerequisites first
    if not (project_root / "Cargo.toml").exists():
        print("Error: Cargo.toml not found")
        return 1

    # Attempt operation
    try:
        do_build()
        print("Build succeeded!")
        return 0
    except Exception as e:
        print(f"Build failed: {e}")
        return 1
```

### Return Codes

| Code | Meaning |
|------|---------|
| `0` or `None` | Success |
| `1` | General failure |
| `2` | Invalid arguments |

### Organization

- **One domain per file**: `task_test.py`, `task_build.py`
- **Group related commands**: Use `command_group` for related tasks
- **Limit nesting**: Two group levels maximum
- **Single responsibility**: Each task does one thing well
