---
title: "Reference"
weight: 4
geekdocCollapseSection: true
---

# Reference Documentation

Complete technical reference for all Angreal APIs, configurations, and commands.

## API Documentation

### [Python API](/angreal/reference/python-api)
Detailed reference for the Python API used in task definitions.

{{< button relref="/reference/python-api" >}}View Python Docs{{< /button >}}

## Command Line Interface

- [CLI Reference](/angreal/reference/cli/) - Complete command-line interface documentation
- [Configuration](/angreal/reference/configuration/) - Configuration file formats and options

## Quick Links

### Python API Modules

- [`angreal.command`](/angreal/reference/python-api/commands/) - Task definition decorators
- [`angreal.template`](/angreal/reference/python-api/templates/) - Template rendering functions
- [`angreal.utils`](/angreal/reference/python-api/utils/) - Utility functions
- [`angreal.integrations.venv`](/angreal/reference/python-api/integrations/venv/) - Virtual environment management

### Common Patterns

```python
# Basic command
@angreal.command(name="task", about="Description")
def my_task():
    pass

# Command with arguments
@angreal.argument(name="input", help="Input file")
def task_with_args(input):
    pass

# Command group
group = angreal.command_group(name="group")
@group()
@angreal.command(name="subcommand")
def grouped_command():
    pass
```

## Using the Reference

- **API Details**: For complete parameter lists and return types
- **Examples**: Most entries include usage examples
- **Cross-references**: Links to related functionality
- **Version info**: Note which version introduced features
