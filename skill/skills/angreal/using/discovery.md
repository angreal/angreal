# Task Discovery

How to find and understand available tasks in an angreal project.

## Checking for Angreal Project

Before using tasks, verify you're in an angreal project:

```
# Look for .angreal/ directory
ls .angreal/
```

If `.angreal/` doesn't exist, you're not in an angreal project. Either:
- Navigate to a project root
- Initialize a new project with a template

## Discovering Available Tasks

### Via MCP Tools

The angreal MCP server exposes all tasks as tools. Tool names follow the pattern:
```
angreal_<group>_<command>
```

Examples:
- `angreal_test_all` - Run all tests
- `angreal_docs_build` - Build documentation
- `angreal_dev_check_deps` - Check dependencies

### Via CLI

```bash
# List all commands
angreal --help

# List commands in a group
angreal test --help

# Get help for a specific command
angreal test all --help
```

## Understanding Task Descriptions

Each task has metadata that helps you understand when to use it:

### About (Short Description)
One-line summary shown in help output. Tells you WHAT the task does.

### Tool Description (Rich Guidance)
Detailed prose that tells you:
- **When to use** - Scenarios where this task is appropriate
- **When NOT to use** - Situations to avoid
- **Examples** - Concrete invocation examples
- **Preconditions** - What needs to be true before running
- **Output** - What to expect from the task

### Risk Level
Tasks declare their risk level:
- `safe` - No destructive effects, safe to run anytime
- `read_only` - Only reads/reports, makes no changes
- `destructive` - May modify or delete data

## Reading Tool Descriptions

When an MCP tool has a `ToolDescription`, read it carefully before invoking:

```
Tool: angreal_deploy_production

Description: Deploy to production environment

Deploys the current build to the production environment.

## When to use
- After all tests pass on staging
- When release is approved

## When NOT to use
- Directly from feature branches
- Without staging validation

## Examples
```
angreal deploy production --version v1.2.3
```

## Preconditions
- Build artifacts exist
- Staging tests passed
- Release approval obtained

Risk: destructive
```

**Key insight**: Tool descriptions are written by task authors to guide you. Trust them.

## Task Groups

Tasks are organized into groups by function:

| Common Group | Purpose |
|--------------|---------|
| `dev` | Development utilities |
| `test` | Testing commands |
| `docs` | Documentation generation |
| `build` | Build and compilation |
| `deploy` | Deployment and release |

Groups can be nested: `docker.compose.up` means the `up` command in the `compose` subgroup of `docker`.

## Choosing the Right Task

### For Testing
1. Look for `test` group
2. Check for `all`, `unit`, `integration` variants
3. Read descriptions to understand coverage

### For Building
1. Look for `build` or `dev` group
2. Check for `release` vs `debug` options
3. Note any prerequisites (like `check-deps`)

### For Documentation
1. Look for `docs` group
2. Typically `build` and `preview` commands
3. May require external tools (hugo, mkdocs)

### When Unsure
1. Start with `angreal --help` to see all groups
2. Drill into promising groups
3. Read tool descriptions before executing
4. Start with read-only tasks to explore safely

## Task Arguments

Discover arguments for a task:

```bash
angreal test all --help
```

Or check the MCP tool schema, which lists:
- Argument names
- Types (string, bool, int)
- Whether required
- Default values
- Help text
