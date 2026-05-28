---
name: angreal-authoring
description: This skill should be used when the user asks to "create an angreal task", "write a task file", "add a command to angreal", "make a new task", "organize tasks with groups", "use @angreal.command", "use command_group", "use angreal.group decorator", "add long_about to a task", "require an angreal version", "same command name in different groups", or needs guidance on task file structure, the @command decorator, command groups (`command_group` and `@group(...)`), `required_version()`, naming conventions, or task organization within an existing angreal project.
version: 2.8.7
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

### `long_about` for detailed help

`about` is the one-liner shown in `angreal tree`. `long_about` is the multi-paragraph help shown by `angreal <command> --help`:

```python
@angreal.command(
    name="deploy",
    about="Deploy to an environment",
    long_about="""
Deploy the built artifacts to the target environment.

Supports: development, staging, production.
Reads credentials from $AWS_PROFILE; aborts if unset.
""",
)
def deploy():
    pass
```

`long_about` is for human CLI users. Pair it with a `tool=angreal.ToolDescription(...)` for AI agents — see the `angreal-tool-descriptions` skill.

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

### Same Command Name in Different Groups

Since 2.8.5 the command registry is keyed by full path, so the same leaf name can safely appear under different groups without collision:

```python
test = angreal.command_group(name="test", about="Testing")
docs = angreal.command_group(name="docs", about="Docs")

@test()
@angreal.command(name="all", about="Run all tests")
def test_all(): ...

@docs()
@angreal.command(name="all", about="Build all docs")  # no collision
def docs_all(): ...
```

Both `angreal test all` and `angreal docs all` work independently.

### Alternative: `@angreal.group(name=...)` single-shot

Instead of creating a reusable decorator with `command_group`, you can use the `@group(...)` decorator inline:

```python
@angreal.group(name="test", about="Testing commands")
@angreal.command(name="all", about="Run all tests")
def test_all():
    pass
```

Use `command_group` when you have many commands sharing the group; use `@group` for one-offs.

## Requiring a Minimum Angreal Version

If a task file uses APIs added in a specific release, declare it at the top of the file so users get a clear error on older Angreal:

```python
import angreal

angreal.required_version(">=2.8.7")

# ...rest of the file
```

The specifier follows PEP 440 (`>=`, `==`, `<`, etc.). On mismatch, Angreal exits with a clear message before attempting to register the file's commands.

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

| Return value | Exit code | Notes |
|--------------|-----------|-------|
| `None` / `True` / `0` | `0` | Success |
| `False` / `1` | `1` | General failure |
| Any non-zero int `N` | `N` | Custom — e.g. `return 2` exits with code 2 |
| `SystemExit(N)` raised | `N` | Intercepted and propagated |
| Any other exception | `56` | Angreal-specific exit code for unhandled task exceptions; traceback is printed |

Propagate subprocess exit codes via `return result.returncode` — don't translate them into 0/1 yourself unless you have a reason.

### Organization

- **One domain per file**: `task_test.py`, `task_build.py`
- **Group related commands**: Use `command_group` for related tasks
- **Limit nesting**: Two group levels maximum
- **Single responsibility**: Each task does one thing well
