---
title: "Commands Module"
weight: 1
---

# Commands Module

The commands module provides decorators and functions for defining Angreal tasks.

## Available Functions

- [`@command`](/reference/python-api/commands/command_decorator) - Define a command
- [`@argument`](/reference/python-api/commands/argument_decorator) - Add arguments to commands
- [`command_group`](/reference/python-api/commands/command_group) - Create command groups

## Quick Examples

### Simple Command
```python
import angreal

@angreal.command(name="test", about="Run tests")
def test_command():
    """Run the test suite."""
    print("Running tests...")
```

### Command with Arguments
```python
@angreal.command(name="deploy", about="Deploy application")
@angreal.argument(name="env", help="Environment to deploy to")
@angreal.argument(name="dry_run", long="dry-run", is_flag=True, help="Perform a dry run")
def deploy_command(env, dry_run=False):
    """Deploy to the specified environment."""
    if dry_run:
        print(f"Would deploy to {env}")
    else:
        print(f"Deploying to {env}")
```

## See Also

- [Commands Guide](/reference/python-api/commands/commands_guide) - Comprehensive guide
- [Create a Task](/how-to-guides/create-a-task) - How-to guide
