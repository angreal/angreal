---
title: Task Discovery and the .angreal Directory
weight: 10
---

# Task Discovery and the .angreal Directory

This document explains how Angreal discovers and loads tasks, and the role of the `.angreal` directory in project organization.

## The .angreal Directory

Every Angreal project contains a `.angreal` directory at its root. This directory serves as the container for all project-specific automation, including task definitions, configuration, and shared modules. When Angreal starts, it searches for this directory to determine whether it's operating within a project context.

The presence of a `.angreal` directory fundamentally changes Angreal's behavior. Outside of an Angreal project, the CLI offers commands like `init` for creating new projects from templates. Inside a project, these standard commands are replaced entirely by the project's own task definitions. This design ensures that project-specific automation always takes precedence.

## Directory Structure

A typical `.angreal` directory contains:

```
.angreal/
├── angreal.toml          # Optional project configuration
├── task_build.py         # Task file for build commands
├── task_test.py          # Task file for test commands
├── task_deploy.py        # Task file for deployment commands
└── shared/               # Optional shared Python modules
    ├── __init__.py
    └── utils.py
```

The `angreal.toml` file stores project metadata and configuration values that can be accessed from tasks using `angreal.get_context()`. This file is optional but useful for storing project-specific settings that tasks might need.

## Task File Discovery

Angreal discovers tasks through a specific naming convention. When loading a project, it searches for Python files matching the pattern `task_*.py` within the `.angreal` directory. Only files that follow this naming convention are loaded as task modules.

The discovery process works as follows. First, Angreal walks up the directory tree from the current working directory, looking for a `.angreal` directory. Once found, it scans that directory for files matching the `task_*.py` pattern. Each matching file is loaded as a Python module, and any functions decorated with `@angreal.command` or registered through command groups are added to the available command set.

This naming convention serves two purposes. It clearly identifies which files contain task definitions, making the project structure easy to understand at a glance. It also allows other Python files to exist in the `.angreal` directory without being treated as task entry points, which is useful for shared utilities and helper modules.

## Project Detection

Angreal determines project context by walking up the directory tree from your current location. This means you can run Angreal commands from any subdirectory within a project, and it will still find and load the project's tasks.

For example, if your project structure looks like this:

```
my-project/
├── .angreal/
│   └── task_build.py
├── src/
│   └── module/
│       └── code.py
└── tests/
```

You can run `angreal build` from `my-project/`, `my-project/src/`, or `my-project/src/module/`, and Angreal will find the `.angreal` directory and load the build task. This behavior matches how tools like Git find their repository root.

## Task Registration

When a task file is loaded, Angreal executes it as a Python module. During this execution, the `@angreal.command` decorator registers each decorated function as an available command. The decorator captures metadata about the command including its name, description, arguments, and any command group associations.

Consider this task file:

```python
import angreal

@angreal.command(name="greet", about="Greet the user")
@angreal.argument(name="name", long="name", takes_value=True)
def greet_command(name="World"):
    print(f"Hello, {name}!")
```

When Angreal loads this file, the `@angreal.command` decorator executes and registers `greet_command` as a command named "greet". The function itself isn't called during loading; it's stored for later invocation when the user runs `angreal greet`.

## Command Groups

Tasks can be organized into groups using `angreal.command_group`. Groups create a hierarchical command structure similar to subcommands in other CLI tools.

```python
import angreal

# Create a group
db = angreal.command_group(name="db", about="Database operations")

@db()
@angreal.command(name="migrate", about="Run database migrations")
def migrate():
    print("Running migrations...")

@db()
@angreal.command(name="seed", about="Seed the database")
def seed():
    print("Seeding database...")
```

These commands become available as `angreal db migrate` and `angreal db seed`. Groups can be nested to create deeper hierarchies, though keeping the structure shallow generally improves usability.

## Shared Modules

Task files often need to share code. You can create additional Python modules in the `.angreal` directory for this purpose. These modules should not follow the `task_*.py` naming pattern, as that would cause Angreal to treat them as task entry points.

A common pattern is to create a `shared/` subdirectory:

```
.angreal/
├── task_build.py
├── task_test.py
└── shared/
    ├── __init__.py
    └── config.py
```

Task files can then import from the shared module:

```python
import angreal
import sys
from pathlib import Path

# Add .angreal to Python path for imports
sys.path.insert(0, str(Path(angreal.get_root())))

from shared.config import get_settings

@angreal.command(name="build", about="Build the project")
def build():
    settings = get_settings()
    # Use settings...
```

## Project Tasks vs Template Tasks

There is an important distinction between tasks that live in a project's `.angreal` directory and tasks that are part of an Angreal template.

Project tasks are the automation commands for a specific project. They exist in the project's `.angreal` directory and are committed to version control along with the rest of the project. These tasks run whenever someone works with that project.

Template tasks are different. They exist within an Angreal template and run during the `angreal init` process to help scaffold a new project. Template tasks might prompt for configuration values, create directories, or perform initial setup. Once the template has been applied, the template's tasks are no longer relevant; the new project has its own `.angreal` directory with its own tasks.

When you run `angreal init some-template`, Angreal downloads or locates the template, runs any template-level tasks, and renders the template files into a new project. The resulting project has its own `.angreal` directory, completely independent of the template that created it.

## Execution Context

When a task runs, several context values are available. The `angreal.get_root()` function returns the path to the `.angreal` directory, which is useful for constructing paths relative to the project root. The `angreal.get_context()` function returns the parsed contents of `angreal.toml` as a dictionary, providing access to project configuration.

Tasks execute with the current working directory unchanged from where the user invoked Angreal. If a user runs `angreal build` from `my-project/src/`, the task's current directory is `my-project/src/`, not `my-project/` or `my-project/.angreal/`. Tasks that need to operate from a specific directory should explicitly change to that directory or use absolute paths constructed from `angreal.get_root()`.

## Troubleshooting Discovery

If your tasks aren't being discovered, check these common issues:

The file must be named with the `task_` prefix and `.py` extension. A file named `build_task.py` or `task_build.txt` will not be discovered.

The file must be directly inside the `.angreal` directory, not in a subdirectory. A file at `.angreal/tasks/task_build.py` will not be discovered; it must be at `.angreal/task_build.py`.

The file must be valid Python that can be imported. Syntax errors or import failures will prevent the task from loading. Check the Angreal output with verbose mode (`angreal -v`) to see loading errors.

The decorated function must use `@angreal.command` to register itself. Simply defining a function in a task file does not make it available as a command.

## See Also

- [Command System Guide](/angreal/reference/python-api/commands/commands_guide) - How to define commands
- [CLI Reference](/angreal/reference/cli) - Command-line interface documentation
- [Angreal Init Behavior](/angreal/explanation/angreal_init_behaviour) - Template resolution
