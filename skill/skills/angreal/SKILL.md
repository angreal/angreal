---
name: angreal
description: Task automation with angreal. Auto-activates for projects with .angreal/ directory. Covers running tasks (angreal <command>), discovering tasks (angreal tree), and authoring new tasks with @angreal.command decorators.
---

# Angreal Task Automation Skill

This skill teaches angreal task automation - both using existing tasks as an AI agent and authoring new tasks as a developer.

## What Angreal Is For

Angreal is focused on **development tasks within software projects**:
- Building and testing code
- Running linters and formatters
- Generating documentation
- Managing development workflows
- Project-specific automation

**Angreal is NOT for**:
- Installing or maintaining software distributions
- System-level package management
- Deployment pipelines to production (though it can trigger them)
- CI/CD infrastructure setup

Think of angreal as your project's `make` or `npm run` - development-time task automation.

## Prerequisites

- Working within an angreal project (has `.angreal/` directory)
- `angreal` CLI installed and available

## What This Skill Provides

This skill teaches **when** and **why** to use tasks, plus how to author new ones.

- When to use which task
- How to discover and understand available tasks
- How to author effective task definitions
- Common patterns and anti-patterns

## Two Perspectives

### Using Tasks (AI Agent)

See [using/discovery.md](using/discovery.md) for:
- Finding available tasks in a project
- Understanding task descriptions and arguments
- Choosing the right task for the job

See [using/execution.md](using/execution.md) for:
- Invoking tasks correctly
- Handling arguments and flags
- Interpreting output and errors

See [using/workflows.md](using/workflows.md) for:
- Common multi-task workflows
- Task sequencing patterns
- Project lifecycle tasks

### Authoring Tasks (Developer)

See [authoring/basics.md](authoring/basics.md) for:
- Creating task files
- The `@command` decorator
- Basic task structure

See [authoring/groups.md](authoring/groups.md) for:
- Organizing tasks with `@group`
- Creating command hierarchies
- Reusable group decorators

See [authoring/tool-descriptions.md](authoring/tool-descriptions.md) for:
- Writing effective `ToolDescription`
- Guiding AI agents with prose
- Risk levels and annotations

See [authoring/arguments.md](authoring/arguments.md) for:
- The `@argument` decorator
- Argument types and validation
- Flags, defaults, and requirements

See [authoring/best-practices.md](authoring/best-practices.md) for:
- Naming conventions
- Error handling patterns
- Organization strategies

## Common Patterns

See [patterns/testing.md](patterns/testing.md) for test automation patterns.
See [patterns/documentation.md](patterns/documentation.md) for doc generation patterns.
See [patterns/development.md](patterns/development.md) for dev workflow patterns.

## Quick Reference

### Task File Location
```
project/
└── .angreal/
    ├── utils.py         # Shared utilities across tasks
    ├── task_dev.py      # Development tasks
    ├── task_test.py     # Testing tasks
    ├── task_docs.py     # Documentation tasks
    └── task_deploy.py   # Deployment tasks
```

You can create shared modules (like `utils.py`) and import them across task files.

### Basic Task Structure
```python
import angreal

@angreal.command(
    name="build",
    about="Build the project",
    tool=angreal.ToolDescription("""
Build the project for distribution.

## When to use
- Before releasing a new version
- Testing production builds

## Examples
```
angreal build
angreal build --release
```
""", risk_level="safe")
)
@angreal.argument(name="release", long="release", is_flag=True, takes_value=False, help="Build in release mode")
def build(release=False):
    # Implementation
    pass
```

### Key Principles

- **Tasks are discoverable** - Use `angreal tree` to see all commands and arguments
- **Descriptions guide AI** - Use `angreal tree --long` for full ToolDescription prose
- **Groups organize** - Related commands share a group (e.g., `angreal test all`)
- **Arguments are typed** - Specify `python_type` for proper conversion
- **Errors are informative** - Return meaningful messages on failure
