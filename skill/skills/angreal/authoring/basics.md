# Task Authoring Basics

How to create angreal task files.

## Task File Location

Task files live in the `.angreal/` directory at your project root:

```
my-project/
├── .angreal/
│   ├── task_dev.py       # Development tasks
│   ├── task_test.py      # Testing tasks
│   ├── task_docs.py      # Documentation tasks
│   └── task_deploy.py    # Deployment tasks
├── src/
└── ...
```

**Naming convention**: Files must be named `task_*.py` to be discovered.

## Minimal Task

The simplest possible task:

```python
import angreal

@angreal.command(name="hello", about="Say hello")
def hello():
    print("Hello, world!")
```

This creates a command `angreal hello` that prints a greeting.

## The @command Decorator

Every task needs the `@command` decorator:

```python
@angreal.command(
    name="build",              # Command name (required if different from function)
    about="Build the project", # Short description (shown in --help)
)
def build():
    pass
```

### Name Inference

If you omit `name`, it's derived from the function name:
- Underscores become hyphens
- Lowercased

```python
@angreal.command(about="Check dependencies")
def check_deps():  # Creates command "check-deps"
    pass
```

### About vs Long About

- `about`: One-line description for help output
- `long_about`: Extended description (shown with `--help`)

```python
@angreal.command(
    name="deploy",
    about="Deploy the application",
    long_about="""
    Deploy the application to the specified environment.

    This command handles the full deployment lifecycle including
    building, uploading, and verifying the deployment.
    """
)
def deploy():
    pass
```

## Return Values

Tasks can return values:

```python
@angreal.command(name="check", about="Check project status")
def check():
    if everything_ok():
        print("All checks passed!")
        return 0  # Success
    else:
        print("Checks failed!")
        return 1  # Failure
```

Return values appear in MCP response as `return_value`.

## Imports and Dependencies

Tasks can import any Python module:

```python
import angreal
import os
import subprocess
import json

@angreal.command(name="status", about="Show project status")
def status():
    result = subprocess.run(["git", "status"], capture_output=True, text=True)
    print(result.stdout)
```

## Getting Project Root

Use `angreal.get_root()` to find the project root:

```python
import angreal
import os

@angreal.command(name="list-files", about="List project files")
def list_files():
    root = angreal.get_root()
    for item in os.listdir(root):
        print(item)
```

**Important**: Always use `get_root()` instead of hardcoding paths or using relative paths.

## Error Handling

Handle errors gracefully:

```python
import angreal

@angreal.command(name="build", about="Build the project")
def build():
    try:
        # Build logic
        pass
    except FileNotFoundError as e:
        print(f"Error: Required file not found: {e}")
        return 1
    except Exception as e:
        print(f"Build failed: {e}")
        return 1

    print("Build succeeded!")
    return 0
```

## Output Best Practices

### Use Print for User Feedback

```python
print("Starting build...")
print("Compiling source files...")
print("Build complete!")
```

### Use Return Codes

- `0` or `None`: Success
- Non-zero: Failure

### Be Informative on Failure

```python
if not os.path.exists(config_file):
    print(f"Error: Config file not found: {config_file}")
    print("Run 'angreal init' to create default configuration")
    return 1
```

## Complete Example

```python
import angreal
import os
import subprocess

@angreal.command(
    name="build",
    about="Build the project",
    long_about="Compile the project and create distribution artifacts."
)
def build():
    """Build the project for distribution."""
    root = angreal.get_root()

    print("Starting build...")

    # Check prerequisites
    if not os.path.exists(os.path.join(root, "Cargo.toml")):
        print("Error: Cargo.toml not found. Is this a Rust project?")
        return 1

    # Run build
    result = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=root,
        capture_output=True,
        text=True
    )

    if result.returncode != 0:
        print("Build failed!")
        print(result.stderr)
        return 1

    print("Build succeeded!")
    print(result.stdout)
    return 0
```
