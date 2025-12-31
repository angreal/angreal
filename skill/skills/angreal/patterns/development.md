# Development Patterns

Best practices for developing angreal tasks.

## Project Organization

### Standard Task File Layout

```
.angreal/
├── utils.py           # Shared utilities (optional)
├── task_dev.py        # Development utilities
├── task_test.py       # Testing commands
├── task_build.py      # Build commands
├── task_docs.py       # Documentation commands
└── task_deploy.py     # Deployment commands
```

For larger projects, you can use subdirectories:

```
.angreal/
├── utils/
│   ├── __init__.py
│   ├── config.py
│   └── shell.py
├── task_dev.py
└── ...
```

### Naming Conventions

| Convention | Example | Usage |
|------------|---------|-------|
| `task_<domain>.py` | `task_test.py` | Task file naming |
| `<domain>_<action>` | `test_all`, `test_unit` | Function naming |
| `<verb>-<noun>` | `check-deps`, `build-docs` | Command naming |
| `<domain>` | `test`, `dev`, `docs` | Group naming |

## Shared Utilities

Create shared modules in `.angreal/` and import them from task files.

### Simple Shared Module

```python
# .angreal/utils.py
import subprocess
import angreal

def run_command(cmd, check=True, capture=True):
    """Run a shell command in the project root."""
    project_root = angreal.get_root().parent  # .angreal dir -> project root
    result = subprocess.run(
        cmd,
        cwd=project_root,
        shell=isinstance(cmd, str),
        capture_output=capture,
        text=True
    )
    if check and result.returncode != 0:
        raise subprocess.CalledProcessError(result.returncode, cmd, result.stdout, result.stderr)
    return result

def get_config():
    """Load project configuration."""
    import json
    import os
    angreal_dir = angreal.get_root()
    config_path = os.path.join(angreal_dir, "config.json")
    if os.path.exists(config_path):
        with open(config_path) as f:
            return json.load(f)
    return {}
```

```python
# .angreal/task_build.py
import angreal
from utils import run_command, get_config

@angreal.command(name="build", about="Build the project")
def build():
    result = run_command(["cargo", "build", "--release"])
    print(result.stdout)
```

## Error Handling Patterns

### Graceful Failures

```python
import angreal
import os

@angreal.command(name="build", about="Build the project")
def build():
    project_root = angreal.get_root().parent  # .angreal dir -> project root

    # Check prerequisites
    cargo_toml = os.path.join(project_root, "Cargo.toml")
    if not os.path.exists(cargo_toml):
        print("Error: Cargo.toml not found")
        print("This command requires a Rust project")
        return 1

    # Attempt operation
    try:
        result = run_cargo_build()
        print("Build succeeded!")
        return 0
    except subprocess.CalledProcessError as e:
        print(f"Build failed with exit code {e.returncode}")
        print(e.stderr)
        return 1
    except Exception as e:
        print(f"Unexpected error: {e}")
        return 1
```

### Informative Error Messages

```python
def validate_environment(env):
    """Validate environment argument."""
    valid_envs = ["development", "staging", "production"]

    if env not in valid_envs:
        print(f"Error: Invalid environment '{env}'")
        print(f"Valid environments: {', '.join(valid_envs)}")
        return False

    return True
```

## Progress and Output

### Progress Indicators

```python
import angreal

@angreal.command(name="build", about="Build all components")
def build():
    components = ["frontend", "backend", "worker"]

    for i, component in enumerate(components, 1):
        print(f"[{i}/{len(components)}] Building {component}...")
        build_component(component)

    print("All components built successfully!")
```

### Verbose Mode Pattern

```python
import angreal

@angreal.command(name="test", about="Run tests")
@angreal.argument(name="verbose", long="verbose", short="v", is_flag=True, takes_value=False)
def test(verbose=False):
    if verbose:
        print("Discovering test files...")

    tests = discover_tests()

    if verbose:
        print(f"Found {len(tests)} test files")

    for test in tests:
        if verbose:
            print(f"Running {test}...")
        run_test(test)
```

### Quiet Mode Pattern

```python
import angreal

@angreal.command(name="check", about="Run checks")
@angreal.argument(name="quiet", long="quiet", short="q", is_flag=True, takes_value=False)
def check(quiet=False):
    issues = run_checks()

    if not issues:
        if not quiet:
            print("All checks passed!")
        return 0

    if quiet:
        # Just return exit code
        return 1

    # Print detailed issues
    print(f"Found {len(issues)} issues:")
    for issue in issues:
        print(f"  - {issue}")
    return 1
```

## Subprocess Patterns

### Running External Commands

```python
import subprocess
import angreal

def run_in_project(cmd, **kwargs):
    """Run command in project root with standard options."""
    project_root = angreal.get_root().parent  # .angreal dir -> project root
    defaults = {
        "cwd": project_root,
        "capture_output": True,
        "text": True
    }
    defaults.update(kwargs)
    return subprocess.run(cmd, **defaults)
```

### Streaming Output

```python
import subprocess
import angreal

@angreal.command(name="test", about="Run tests")
def test():
    project_root = angreal.get_root().parent  # .angreal dir -> project root

    # Stream output in real-time
    process = subprocess.Popen(
        ["pytest", "-v"],
        cwd=project_root,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True
    )

    for line in process.stdout:
        print(line, end="")

    return process.wait()
```

### Handling Failures

```python
import subprocess

def run_or_fail(cmd, error_msg):
    """Run command, print error message on failure."""
    result = subprocess.run(cmd, capture_output=True, text=True)

    if result.returncode != 0:
        print(f"Error: {error_msg}")
        if result.stderr:
            print(result.stderr)
        raise SystemExit(1)

    return result
```

## Dry Run Pattern

```python
import angreal

@angreal.command(name="clean", about="Clean build artifacts")
@angreal.argument(name="dry_run", long="dry-run", short="n", is_flag=True, takes_value=False)
def clean(dry_run=False):
    project_root = angreal.get_root().parent  # .angreal dir -> project root
    targets = ["dist/", "build/", ".cache/"]

    for target in targets:
        path = os.path.join(project_root, target)
        if os.path.exists(path):
            if dry_run:
                print(f"Would remove: {path}")
            else:
                print(f"Removing: {path}")
                shutil.rmtree(path)

    if dry_run:
        print("\nDry run complete. No changes made.")
```

## Environment Variable Handling

```python
import os
import angreal

@angreal.command(name="deploy", about="Deploy application")
@angreal.argument(name="env", long="env", required=True)
def deploy(env):
    # Check required environment variables
    required_vars = ["AWS_ACCESS_KEY_ID", "AWS_SECRET_ACCESS_KEY"]
    missing = [v for v in required_vars if not os.environ.get(v)]

    if missing:
        print("Error: Missing required environment variables:")
        for var in missing:
            print(f"  - {var}")
        print("\nSet these variables before deploying.")
        return 1

    # Proceed with deployment
    do_deploy(env)
```

## Confirmation Pattern

```python
import angreal

@angreal.command(name="delete", about="Delete resources")
@angreal.argument(name="force", long="force", short="f", is_flag=True, takes_value=False)
def delete(force=False):
    if not force:
        print("This will permanently delete resources.")
        response = input("Continue? [y/N] ")
        if response.lower() != "y":
            print("Aborted.")
            return 0

    do_delete()
    print("Deleted successfully.")
```

## Composing Tasks

### Calling Other Task Functions

```python
import angreal

# Import other task modules
from task_test import test_all
from task_lint import lint_check

@angreal.command(name="ci", about="Run CI pipeline")
def ci():
    print("Running linter...")
    if lint_check() != 0:
        print("Lint failed!")
        return 1

    print("\nRunning tests...")
    if test_all() != 0:
        print("Tests failed!")
        return 1

    print("\nCI passed!")
    return 0
```

### Conditional Task Execution

```python
import angreal
import os

@angreal.command(name="build", about="Build if needed")
@angreal.argument(name="force", long="force", short="f", is_flag=True, takes_value=False)
def build(force=False):
    project_root = angreal.get_root().parent  # .angreal dir -> project root
    output = os.path.join(project_root, "dist", "app")

    if not force and os.path.exists(output):
        src_mtime = get_latest_src_mtime()
        out_mtime = os.path.getmtime(output)

        if out_mtime > src_mtime:
            print("Build is up to date. Use --force to rebuild.")
            return 0

    print("Building...")
    do_build()
```

## Anti-Patterns to Avoid

### Don't Hardcode Paths

```python
# Bad
config_path = "/Users/me/project/.angreal/config.json"

# Good - for files in .angreal/
angreal_dir = angreal.get_root()  # Returns .angreal/ path
config_path = os.path.join(angreal_dir, "config.json")

# Good - for files in project root
project_root = angreal.get_root().parent
src_path = os.path.join(project_root, "src")
```

### Don't Ignore Return Codes

```python
# Bad
subprocess.run(["npm", "install"])
subprocess.run(["npm", "test"])  # Runs even if install failed

# Good
result = subprocess.run(["npm", "install"])
if result.returncode != 0:
    return 1
subprocess.run(["npm", "test"])
```

### Don't Swallow Exceptions

```python
# Bad
try:
    do_something()
except:
    pass  # Silently fails

# Good
try:
    do_something()
except SpecificError as e:
    print(f"Error: {e}")
    return 1
```

### Don't Mix Concerns

```python
# Bad - one task does too much
@angreal.command(name="do-everything")
def do_everything():
    build()
    test()
    lint()
    deploy()
    notify()

# Good - separate tasks, compose as needed
@angreal.command(name="release")
def release():
    """Orchestrate release process."""
    steps = [build, test, lint, deploy]
    for step in steps:
        if step() != 0:
            return 1
    notify()
    return 0
```
