# Best Practices

Conventions and patterns for well-designed angreal tasks.

## Naming Conventions

### Task Files

```
task_<domain>.py
```

Examples:
- `task_dev.py` - Development utilities
- `task_test.py` - Testing commands
- `task_build.py` - Build commands
- `task_deploy.py` - Deployment commands

### Command Names

Use lowercase with hyphens:

```python
# Good
@angreal.command(name="check-deps")
@angreal.command(name="run-tests")
@angreal.command(name="build-docs")

# Avoid
@angreal.command(name="checkDeps")   # camelCase
@angreal.command(name="check_deps")  # underscores (convention is hyphens)
@angreal.command(name="CheckDeps")   # PascalCase
```

### Function Names

Use snake_case (Python convention):

```python
@angreal.command(name="check-deps")
def check_deps():  # Function name matches but uses underscores
    pass
```

### Group Names

Short, descriptive nouns or verbs:

```python
# Good
dev = angreal.command_group(name="dev")
test = angreal.command_group(name="test")
db = angreal.command_group(name="db")

# Avoid
development_utilities = angreal.command_group(name="development-utilities")  # Too long
```

## Task Design

### Single Responsibility

Each task should do one thing well:

```python
# Good: Focused tasks
@angreal.command(name="test", about="Run unit tests")
def test():
    run_pytest()

@angreal.command(name="lint", about="Check code style")
def lint():
    run_linter()

# Bad: Task does too much
@angreal.command(name="check-all", about="Run tests and lint and format")
def check_all():
    run_pytest()
    run_linter()
    run_formatter()
    check_types()
    audit_deps()
```

If you need combined operations, compose tasks:

```python
@angreal.command(name="ci", about="Run CI checks")
def ci():
    """Run all CI checks in sequence."""
    if test() != 0:
        return 1
    if lint() != 0:
        return 1
    return 0
```

### Explicit Over Implicit

Make task behavior clear:

```python
# Good: Explicit flags
@angreal.command(name="build", about="Build project")
@angreal.argument(name="release", long="release", is_flag=True, takes_value=False, help="Build release version")
@angreal.argument(name="target", long="target", default_value="default", help="Build target")
def build(release=False, target="default"):
    pass

# Bad: Hidden behavior based on environment
@angreal.command(name="build")
def build():
    if os.environ.get("CI"):
        # Silently different behavior
        pass
```

### Fail Fast

Check prerequisites early:

```python
@angreal.command(name="deploy")
def deploy():
    # Check all prerequisites first
    if not check_credentials():
        print("Error: Missing deployment credentials")
        return 1

    if not check_build_exists():
        print("Error: No build artifacts found. Run 'angreal build' first.")
        return 1

    if not check_tests_passed():
        print("Error: Tests must pass before deployment")
        return 1

    # Only proceed if everything is ready
    do_deploy()
```

## Error Handling

### Return Codes

Use consistent return codes:

| Code | Meaning |
|------|---------|
| `0` or `None` | Success |
| `1` | General failure |
| `2` | Invalid arguments |
| `3+` | Task-specific errors |

```python
@angreal.command(name="validate")
def validate():
    if not os.path.exists("config.yaml"):
        print("Error: config.yaml not found")
        return 2  # Bad input

    try:
        config = load_config()
    except yaml.YAMLError as e:
        print(f"Error: Invalid YAML: {e}")
        return 2  # Bad input

    if not validate_config(config):
        print("Error: Configuration validation failed")
        return 1  # General failure

    print("Configuration is valid")
    return 0  # Success
```

### Informative Messages

Tell users what went wrong AND how to fix it:

```python
# Bad
print("Error")

# Better
print("Error: Build failed")

# Best
print("Error: Build failed - missing dependency 'cmake'")
print("Install with: brew install cmake (macOS) or apt install cmake (Linux)")
```

### Never Swallow Exceptions

```python
# Bad
try:
    do_something()
except Exception:
    pass

# Good
try:
    do_something()
except SpecificError as e:
    print(f"Error: {e}")
    return 1
```

## Documentation

### Always Write about

Every command needs a short description:

```python
# Good
@angreal.command(name="build", about="Build the project for distribution")

# Bad
@angreal.command(name="build")  # No description
```

### Write ToolDescriptions for AI Agents

If your task will be used by AI agents, add rich guidance:

```python
@angreal.command(
    name="deploy",
    about="Deploy to environment",
    tool=angreal.ToolDescription("""
        Deploy the application to the specified environment.

        ## When to use
        - After tests pass and release is approved

        ## Examples
        ```
        angreal deploy --env staging
        angreal deploy --env production --version v1.2.3
        ```
        """,
        risk_level="destructive"
    )
)
```

### Document Arguments

Every argument should have help text:

```python
@angreal.argument(
    name="env",
    long="env",
    required=True,
    help="Target environment: development, staging, or production"
)
```

## Organization

### Group Related Commands

```python
# Good: Grouped
test = angreal.command_group(name="test", about="Testing commands")

@test()
@angreal.command(name="unit", about="Run unit tests")
def test_unit():
    pass

@test()
@angreal.command(name="integration", about="Run integration tests")
def test_integration():
    pass

# Result: angreal test unit, angreal test integration
```

### Limit Nesting Depth

Two levels is usually sufficient:

```python
# Good: Two levels
# angreal docker build
# angreal docker compose up

# Questionable: Three levels
# angreal docker compose service restart
```

### Separate Files by Domain

Don't put everything in one file:

```python
# Good
.angreal/
├── task_dev.py      # Development commands
├── task_test.py     # Testing commands
├── task_build.py    # Build commands
└── task_deploy.py   # Deployment commands

# Bad
.angreal/
└── task_all.py      # 1000 lines with everything
```

## Output

### Use Print for Feedback

```python
@angreal.command(name="build")
def build():
    print("Starting build...")
    do_build()
    print("Build completed successfully!")
```

### Progress for Long Operations

```python
@angreal.command(name="test")
def test():
    tests = discover_tests()
    for i, test in enumerate(tests, 1):
        print(f"[{i}/{len(tests)}] Running {test}...")
        run_test(test)
```

### Verbose Mode for Debugging

```python
@angreal.argument(name="verbose", long="verbose", short="v", is_flag=True, takes_value=False)
def cmd(verbose=False):
    if verbose:
        print("Debug: Loading configuration...")
    # ...
```

## Anti-Patterns

### Don't Hardcode Paths

```python
# Bad
config = open("/Users/me/project/config.yaml")

# Good - get project root (parent of .angreal/)
project_root = angreal.get_root().parent
config = open(os.path.join(project_root, "config.yaml"))
```

### Don't Assume Environment

```python
# Bad: Assumes macOS
subprocess.run(["open", url])

# Good: Cross-platform
import webbrowser
webbrowser.open(url)
```

### Don't Require Interactive Input

Tasks should work non-interactively for CI and automation:

```python
# Bad
name = input("Enter name: ")

# Good
@angreal.argument(name="name", long="name", required=True)
def cmd(name):
    pass
```

### Don't Pollute Global State

```python
# Bad
import os
os.chdir("/some/path")  # Affects other tasks

# Good
subprocess.run(cmd, cwd="/some/path")  # Localized
```

### Don't Ignore Subprocess Failures

```python
# Bad
subprocess.run(["npm", "install"])
subprocess.run(["npm", "test"])  # Runs even if install failed

# Good
result = subprocess.run(["npm", "install"])
if result.returncode != 0:
    print("Install failed!")
    return 1
subprocess.run(["npm", "test"])
```
