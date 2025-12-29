# Arguments

How to define and use arguments with the @argument decorator.

## Basic Usage

```python
import angreal

@angreal.command(name="greet", about="Greet someone")
@angreal.argument(name="name", long="name", help="Name to greet")
def greet(name="World"):
    print(f"Hello, {name}!")
```

## The @argument Decorator

```python
@angreal.argument(
    name,                   # Argument name (required, must match function parameter)
    *,
    short=None,             # Short flag, e.g., "v" for -v
    long=None,              # Long flag, e.g., "verbose" for --verbose
    help=None,              # Help text for --help output
    long_help=None,         # Extended help text
    required=None,          # Whether argument is required
    default_value=None,     # Default value (must be a string)
    takes_value=None,       # Whether argument takes a value (default True)
    is_flag=None,           # Whether this is a boolean flag
    python_type=None,       # Type for MCP schema: "str", "int", "float", "bool"
    multiple_values=None,   # Whether argument can be repeated
    number_of_values=None,  # Exact number of values expected
    min_values=None,        # Minimum number of values
    max_values=None,        # Maximum number of values
    require_equals=None     # Whether argument requires = for value
)
```

## Decorator Order

**Critical**: `@argument` decorators must come AFTER `@command` in source order:

```python
# Correct order
@angreal.command(name="build", about="Build project")
@angreal.argument(name="release", long="release", is_flag=True, takes_value=False)
@angreal.argument(name="target", long="target", help="Build target")
def build(release=False, target="default"):
    pass

# Wrong order - will fail
@angreal.argument(name="release", is_flag=True)  # Error!
@angreal.command(name="build", about="Build project")
def build(release=False):
    pass
```

## Argument Types

### String Arguments

```python
import angreal

@angreal.command(name="echo", about="Echo a message")
@angreal.argument(name="message", long="message", help="Message to display")
def echo(message="default"):
    print(message)
```

CLI: `angreal echo --message "Hello"`

### Integer Arguments

For MCP schema generation, specify `python_type`:

```python
import angreal

@angreal.command(name="repeat", about="Repeat N times")
@angreal.argument(name="count", long="count", python_type="int", help="Number of iterations")
def repeat(count=1):
    for i in range(int(count)):
        print(i)
```

CLI: `angreal repeat --count 5`

### Float Arguments

```python
import angreal

@angreal.command(name="threshold", about="Set threshold")
@angreal.argument(name="value", long="value", python_type="float", help="Score threshold")
def threshold(value=0.5):
    print(f"Threshold: {float(value)}")
```

CLI: `angreal threshold --value 0.75`

### Boolean Flags

Flags are boolean arguments that don't take a value. Use `is_flag=True` with `takes_value=False`:

```python
import angreal

@angreal.command(name="build", about="Build the project")
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

## Required vs Optional

### Required Arguments

```python
import angreal

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

CLI: `angreal deploy --target production`

The task will fail if `--target` is not provided.

### Optional with Defaults

Note: `default_value` must be a string:

```python
import angreal

@angreal.command(name="serve", about="Start server")
@angreal.argument(
    name="env",
    long="env",
    default_value="development",
    help="Environment (default: development)"
)
def serve(env="development"):
    print(f"Running in {env}")
```

CLI: `angreal serve` uses "development", `angreal serve --env staging` uses "staging"

## Short and Long Flags

```python
import angreal

@angreal.command(name="process", about="Process files")
@angreal.argument(
    name="verbose",
    short="v",
    long="verbose",
    is_flag=True,
    takes_value=False,
    help="Verbose output"
)
@angreal.argument(
    name="output",
    short="o",
    long="output",
    help="Output file path"
)
def process(verbose=False, output=None):
    pass
```

CLI: `angreal process -v -o out.txt` or `angreal process --verbose --output out.txt`

## Multiple Values

For arguments that can be specified multiple times:

```python
import angreal

@angreal.command(name="compile", about="Compile files")
@angreal.argument(
    name="file",
    long="file",
    multiple_values=True,
    help="Files to process (can specify multiple)"
)
def compile(file=None):
    files = file or []
    for f in files:
        print(f"Processing {f}")
```

CLI: `angreal compile --file a.txt --file b.txt --file c.txt`

## Combining Multiple Arguments

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
    help="Number of parallel jobs"
)
@angreal.argument(
    name="feature",
    long="feature",
    multiple_values=True,
    help="Features to enable"
)
def build(target="debug", verbose=False, jobs=4, feature=None):
    features = feature or []
    if verbose:
        print(f"Building {target} with {jobs} jobs")
        print(f"Features: {features}")
```

CLI: `angreal build --target release -v -j 8 --feature logging --feature metrics`

## Function Signature Matching

**Important**: The function parameter names must match argument `name` values:

```python
import angreal

@angreal.command(name="cmd", about="Example")
@angreal.argument(name="target", long="target", required=True)
@angreal.argument(name="verbose", long="verbose", is_flag=True, takes_value=False)
@angreal.argument(name="output", long="output")
def cmd(target, verbose=False, output=None):  # Parameter names match!
    pass
```

Use defaults in the function signature for optional arguments.

## Help Text Best Practices

Write clear, informative help text:

```python
# Good help text
@angreal.argument(
    name="env",
    long="env",
    help="Target environment: development, staging, or production"
)

# Better - includes default
@angreal.argument(
    name="port",
    long="port",
    python_type="int",
    default_value="8080",
    help="Server port (default: 8080)"
)

# Best - includes constraints
@angreal.argument(
    name="workers",
    long="workers",
    python_type="int",
    default_value="4",
    help="Number of worker processes (1-16, default: 4)"
)
```

## MCP Schema Generation

When exposed via MCP, arguments become tool input schema. Use `python_type` to specify the JSON schema type:

```python
@angreal.argument(name="target", long="target", python_type="str", required=True, help="Deploy target")
@angreal.argument(name="dry_run", long="dry-run", is_flag=True, takes_value=False, help="Dry run mode")
```

Generates MCP schema:
```json
{
  "type": "object",
  "properties": {
    "target": {
      "type": "string",
      "description": "Deploy target"
    },
    "dry_run": {
      "type": "boolean",
      "description": "Dry run mode"
    }
  },
  "required": ["target"]
}
```

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

### Verbosity Levels

```python
@angreal.argument(
    name="verbose",
    short="v",
    long="verbose",
    is_flag=True,
    takes_value=False,
    help="Verbose output"
)
@angreal.argument(
    name="quiet",
    short="q",
    long="quiet",
    is_flag=True,
    takes_value=False,
    help="Suppress output"
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
