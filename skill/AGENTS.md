# Angreal Task Automation Agent

This agent configuration teaches angreal task automation - both using existing tasks and authoring new ones.

## Overview

Angreal is a task automation and project templating system that combines:
- **Rust core** for performance and reliability
- **Python tasks** for flexibility and ease of authoring
- **MCP integration** for AI agent access

## Agent Capabilities

### Using Tasks

When working with angreal projects, this agent understands:

- **Task Discovery**: How to find and understand available commands
- **Execution Patterns**: Proper invocation with arguments
- **Error Handling**: Interpreting failures and recovering
- **Common Workflows**: Testing, building, documentation

### Authoring Tasks

When creating or modifying tasks, this agent understands:

- **Decorators**: `@command`, `@argument`, `@group`
- **Tool Descriptions**: Writing effective `ToolDescription` for AI agents
- **Argument Patterns**: Types, flags, defaults, validation
- **Best Practices**: Organization, naming, error handling

## Key Behaviors

### When Using Tasks

1. Check if in an angreal project (`.angreal/` directory exists)
2. Discover available tasks via MCP or `angreal --help`
3. Read tool descriptions to understand when/how to use
4. Execute with appropriate arguments
5. Handle errors gracefully

### When Authoring Tasks

1. Place task files in `.angreal/task_*.py`
2. Use descriptive names that reflect the action
3. Provide `ToolDescription` with examples and guidance
4. Group related commands logically
5. Include proper error handling and feedback

### Task Selection

| Need | Approach |
|------|----------|
| Run tests | Look for `test` group commands |
| Build project | Look for `build` or `dev` commands |
| Generate docs | Look for `docs` group commands |
| Deploy/release | Look for `deploy` or `release` commands |

## Anti-Patterns to Avoid

- Running tasks without checking tool descriptions first
- Creating tasks without `about` descriptions
- Missing error handling in task implementations
- Mixing unrelated functionality in single tasks
- Hardcoding paths instead of using `angreal.get_root()`

## Reference Documentation

Detailed guidance is available in:
- `skill/using/` - Task discovery, execution, and workflows
- `skill/authoring/` - Writing tasks, decorators, and descriptions
- `skill/patterns/` - Common task patterns for different scenarios

The Angreal MCP server provides tool parameters and operational details.
