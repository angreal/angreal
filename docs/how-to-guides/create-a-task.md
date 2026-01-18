---
title: Create a Command
weight: 10
---

# Create a Command

Every Angreal project needs commands to automate its workflows. This guide walks through creating a command from scratch, explaining each step along the way.

## Setting Up the Task File

Commands live in Python files within your project's `.angreal` directory. Angreal discovers these files by looking for the `task_` prefix, so your file must be named accordingly. Create a new file such as `.angreal/task_build.py` to get started.

Inside the file, import the angreal module at the top. This module provides the decorators and utilities needed to define commands.

```python
import angreal
```

## Defining a Basic Command

A command is simply a Python function with the `@angreal.command` decorator applied. The decorator transforms your function into a CLI command that users can invoke through Angreal.

```python
import angreal

@angreal.command(name='build', about='Build the project for distribution')
def build_command():
    print("Building the project...")
```

The `name` parameter sets the command name that users type, while `about` provides a short description shown in help output. After saving this file, running `angreal build` will execute your function.

## Adding Command Arguments

Most commands need to accept input from users. The `@angreal.argument` decorator adds command-line arguments to your function. Stack multiple argument decorators to accept several inputs.

```python
import angreal

@angreal.command(name='build', about='Build the project for distribution')
@angreal.argument(
    name='target',
    long='target',
    short='t',
    takes_value=True,
    help='Build target (dev or prod)'
)
@angreal.argument(
    name='clean',
    long='clean',
    is_flag=True,
    help='Clean build artifacts first'
)
def build_command(target='dev', clean=False):
    if clean:
        print("Cleaning previous build...")
    print(f"Building for {target} environment...")
```

The function parameters should match your argument names and provide sensible defaults. Users can now run `angreal build --target prod --clean` to customize the command's behavior.

## Providing AI Agent Guidance

Angreal supports AI agent integration through the `ToolDescription` class. When an AI agent needs to understand your project's commands, it can read these descriptions to make informed decisions about when and how to use each command.

```python
import angreal

@angreal.command(
    name='deploy',
    about='Deploy the application',
    tool=angreal.ToolDescription("""
Deploy the application to the specified environment.

## When to use
Use this command after successful builds when you need to push
changes to staging or production environments.

## When NOT to use
Avoid using this command during active development or when tests
are failing. Never deploy directly to production without first
deploying to staging.

## Examples
```
angreal deploy --environment staging
angreal deploy --environment production --dry-run
```
""", risk_level="destructive")
)
def deploy_command():
    # deployment logic
    pass
```

The `risk_level` parameter helps AI agents understand the command's potential impact. Use `"safe"` for commands that only read data or produce output, `"read_only"` for commands that access external systems without modifying them, and `"destructive"` for commands that modify state or could cause data loss.

AI agents can view these descriptions using the `angreal tree --long` command, which displays the full tool description prose for each available command.

## Complete Example

Here's a fully-featured command that demonstrates all the concepts together.

```python
import angreal
import subprocess
import sys

@angreal.command(
    name='test',
    about='Run the project test suite',
    tool=angreal.ToolDescription("""
Execute the project's test suite using pytest.

## When to use
Run this command after making code changes to verify nothing is broken.
Use it before committing code or creating pull requests.

## When NOT to use
Skip this if you're only changing documentation or configuration files
that don't affect runtime behavior.

## Examples
```
angreal test
angreal test --coverage --verbose
angreal test --filter "test_auth"
```
""", risk_level="safe")
)
@angreal.argument(
    name='coverage',
    long='coverage',
    short='c',
    is_flag=True,
    help='Generate coverage report'
)
@angreal.argument(
    name='verbose',
    long='verbose',
    short='v',
    is_flag=True,
    help='Show verbose output'
)
@angreal.argument(
    name='filter',
    long='filter',
    short='f',
    takes_value=True,
    help='Run only tests matching this pattern'
)
def test_command(coverage=False, verbose=False, filter=None):
    """Run the test suite with the specified options."""
    cmd = [sys.executable, '-m', 'pytest']

    if coverage:
        cmd.extend(['--cov=src', '--cov-report=html'])

    if verbose:
        cmd.append('-v')

    if filter:
        cmd.extend(['-k', filter])

    result = subprocess.run(cmd)
    sys.exit(result.returncode)
```

## See Also

- [Add Arguments to Commands](/angreal/how-to-guides/add-arguments) - Detailed argument configuration
- [Create a Task Group](/angreal/how-to-guides/create-task-group) - Organizing related commands
- [Command Decorator Reference](/angreal/reference/python-api/commands/command_decorator) - Full API documentation
