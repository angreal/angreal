---
name: angreal-arguments
description: This skill should be used when the user asks to "add arguments to a task", "use @angreal.argument", "add flags to command", "make argument required", "add optional parameter", "use python_type", "handle multiple values", or needs guidance on the @argument decorator, argument types, flags, default values, or CLI argument handling in angreal tasks.
version: 2.8.0
---

# Angreal Arguments

Add command-line arguments to angreal tasks using the `@argument` decorator.

## Basic Usage

```python
import angreal

@angreal.command(name="greet", about="Greet someone")
@angreal.argument(name="name", long="name", help="Name to greet")
def greet(name="World"):
    print(f"Hello, {name}!")
```

CLI: `angreal greet --name Alice`

## Decorator Order

**Critical**: `@argument` must come AFTER `@command`:

```python
# Correct
@angreal.command(name="build", about="Build project")
@angreal.argument(name="release", long="release", is_flag=True, takes_value=False)
def build(release=False):
    pass

# Wrong - will fail
@angreal.argument(name="release", is_flag=True)  # Error!
@angreal.command(name="build", about="Build project")
def build(release=False):
    pass
```

## The @argument Decorator

```python
@angreal.argument(
    name,                   # Required: must match function parameter
    short=None,             # Short flag: "v" for -v
    long=None,              # Long flag: "verbose" for --verbose
    help=None,              # Help text for --help
    long_help=None,         # Extended help text (shown with --help)
    required=None,          # Is argument required?
    default_value=None,     # Default (must be string)
    takes_value=True,       # Takes a value? (default True)
    is_flag=False,          # Boolean flag? (default False)
    python_type="str",      # Type: "str", "int", "float"
    multiple_values=None,   # Can be repeated?
    require_equals=None,    # Require --arg=value syntax
    number_of_values=None,  # Exact number of values required
    min_values=None,        # Minimum values when multiple
    max_values=None,        # Maximum values when multiple
)
```

### Parameter Details

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | str | **Required**. Must match function parameter name |
| `short` | str | Single character for short flag (e.g., `"v"` for `-v`) |
| `long` | str | Long flag name (e.g., `"verbose"` for `--verbose`) |
| `help` | str | Brief help text shown in `--help` output |
| `long_help` | str | Extended help text for detailed documentation |
| `required` | bool | Whether the argument must be provided |
| `default_value` | str | Default value (always a string, converted by `python_type`) |
| `takes_value` | bool | Whether argument accepts a value (default: `True`) |
| `is_flag` | bool | Boolean flag mode (default: `False`) |
| `python_type` | str | Type conversion: `"str"`, `"int"`, `"float"` |
| `multiple_values` | bool | Allow repeating the argument |
| `require_equals` | bool | Require `--arg=value` instead of `--arg value` |
| `number_of_values` | int | Exact number of values required |
| `min_values` | int | Minimum number of values (with `multiple_values`) |
| `max_values` | int | Maximum number of values (with `multiple_values`) |

## Argument Types

### String Arguments

```python
@angreal.command(name="echo", about="Echo message")
@angreal.argument(name="message", long="message", help="Message to display")
def echo(message="default"):
    print(message)
```

### Integer Arguments

```python
@angreal.command(name="repeat", about="Repeat N times")
@angreal.argument(name="count", long="count", python_type="int", help="Iterations")
def repeat(count=1):
    for i in range(int(count)):
        print(i)
```

### Boolean Flags

Use `is_flag=True` with `takes_value=False`:

```python
@angreal.command(name="build", about="Build project")
@angreal.argument(
    name="verbose",
    short="v",
    long="verbose",
    is_flag=True,
    takes_value=False,
    help="Enable verbose output"
)
def build(verbose=False):
    if verbose:
        print("Verbose mode enabled")
```

CLI: `angreal build --verbose` or `angreal build -v`

### Required Arguments

```python
@angreal.command(name="deploy", about="Deploy application")
@angreal.argument(
    name="target",
    long="target",
    required=True,
    help="Deployment target (required)"
)
def deploy(target):
    print(f"Deploying to {target}")
```

### Multiple Values

```python
@angreal.command(name="compile", about="Compile files")
@angreal.argument(
    name="file",
    long="file",
    multiple_values=True,
    help="Files to process"
)
def compile(file=None):
    files = file or []
    for f in files:
        print(f"Processing {f}")
```

CLI: `angreal compile --file a.txt --file b.txt`

## Complete Example

```python
import angreal

@angreal.command(name="build", about="Build the project")
@angreal.argument(
    name="target",
    long="target",
    default_value="debug",
    help="Build target (debug or release)"
)
@angreal.argument(
    name="verbose",
    short="v",
    long="verbose",
    is_flag=True,
    takes_value=False,
    help="Verbose output"
)
@angreal.argument(
    name="jobs",
    short="j",
    long="jobs",
    python_type="int",
    default_value="4",
    help="Parallel jobs"
)
def build(target="debug", verbose=False, jobs=4):
    if verbose:
        print(f"Building {target} with {jobs} jobs")
```

CLI: `angreal build --target release -v -j 8`

## Common Patterns

### Environment Selection

```python
@angreal.argument(
    name="env",
    short="e",
    long="env",
    default_value="development",
    help="Environment: development, staging, production"
)
```

### Dry Run Mode

```python
@angreal.argument(
    name="dry_run",
    short="n",
    long="dry-run",
    is_flag=True,
    takes_value=False,
    help="Show what would be done without making changes"
)
```

### File I/O

```python
@angreal.argument(
    name="input",
    short="i",
    long="input",
    required=True,
    help="Input file path"
)
@angreal.argument(
    name="output",
    short="o",
    long="output",
    help="Output file path (default: stdout)"
)
```

## Function Signature Matching

Parameter names must match argument `name` values:

```python
@angreal.command(name="cmd", about="Example")
@angreal.argument(name="target", long="target", required=True)
@angreal.argument(name="verbose", long="verbose", is_flag=True, takes_value=False)
def cmd(target, verbose=False):  # Names match!
    pass
```

## Help Text Best Practices

```python
# Good - includes default and constraints
@angreal.argument(
    name="workers",
    long="workers",
    python_type="int",
    default_value="4",
    help="Number of worker processes (1-16, default: 4)"
)
```

## Advanced Parameters

### Long Help Text

Use `long_help` for extended documentation:

```python
@angreal.argument(
    name="format",
    long="format",
    help="Output format",
    long_help="""Output format for the generated report.

Supported formats:
  - json: Machine-readable JSON output
  - csv: Comma-separated values
  - table: Human-readable table (default)

Example: --format=json"""
)
```

### Require Equals Syntax

Force `--arg=value` instead of `--arg value`:

```python
@angreal.argument(
    name="config",
    long="config",
    require_equals=True,
    help="Config file (use --config=path)"
)
```

CLI: `angreal cmd --config=settings.toml` (not `--config settings.toml`)

### Exact Number of Values

Require exactly N values:

```python
@angreal.argument(
    name="point",
    long="point",
    number_of_values=2,
    help="X Y coordinates"
)
def plot(point):
    x, y = point  # Always receives exactly 2 values
```

CLI: `angreal plot --point 10 20`

### Value Count Constraints

Limit how many values can be provided:

```python
@angreal.argument(
    name="tags",
    long="tag",
    multiple_values=True,
    min_values=1,
    max_values=5,
    help="Tags (1-5 required)"
)
def tag(tags):
    for t in tags:
        print(f"Tag: {t}")
```

CLI: `angreal tag --tag foo --tag bar --tag baz`
