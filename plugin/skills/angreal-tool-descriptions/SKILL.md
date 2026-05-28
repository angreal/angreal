---
name: angreal-tool-descriptions
description: This skill should be used when the user asks to "write a ToolDescription", "add AI guidance to task", "document task for AI", "set risk level", "write tool description prose", "guide AI agents", "MCP tool description", "make tasks AI-friendly", "expose task to angreal mcp", or needs guidance on angreal.ToolDescription, risk_level, or writing prose that the MCP server and `angreal tree --long` will show to agents.
version: 2.8.7
---

# Angreal ToolDescriptions

Write effective ToolDescription prose to guide AI agents using your tasks.

## Why ToolDescriptions Matter

ToolDescriptions are the contract between your tasks and any AI agent that consumes them. They surface in two places:

1. **`angreal mcp`** — the built-in MCP stdio server injects every task's ToolDescription into the `initialize` response sent to connected MCP clients (Claude Code, etc.). This is the *primary* reason to write good descriptions: it's how agents discover what your project's tasks do without trial and error.
2. **`angreal tree --long`** — the same prose shows up in human-facing CLI output.

Think of a ToolDescription as a mini-prompt that teaches agents when and how to use your tool. Without one, the MCP server only exposes the command name and `about` line — agents have to guess.

## Basic Usage

```python
import angreal

@angreal.command(
    name="deploy",
    about="Deploy to environment",
    tool=angreal.ToolDescription(
        """
        Deploy the application to a specified environment.

        ## When to use
        - After successful build and tests pass
        - When user explicitly requests deployment

        ## When NOT to use
        - Directly from feature branches
        - Without running tests first

        ## Examples
        ```
        angreal deploy --env staging
        angreal deploy --env production --version v1.2.3
        ```
        """,
        risk_level="destructive"
    )
)
def deploy(env, version=None):
    pass
```

## ToolDescription Constructor

```python
angreal.ToolDescription(
    description,          # Prose description (required)
    risk_level="safe"     # "safe", "read_only", or "destructive"
)
```

## Risk Levels

| Level | Meaning | Use When |
|-------|---------|----------|
| `safe` | No destructive effects | Default. Build, test, lint tasks |
| `read_only` | Only reads/reports | Status checks, info gathering |
| `destructive` | May modify or delete | Deploy, clean, database migrations |

An invalid string (typo, anything outside the three values above) is **not** an error — Angreal logs a warning and silently falls back to `"safe"`. Watch for this when you intended `"destructive"` and got `"safe"` instead: check `angreal tree --long` to confirm the risk level rendered correctly.

## Structuring Descriptions

Use consistent markdown sections:

```python
tool=angreal.ToolDescription(
    """
    One-line summary of what the tool does.

    ## When to use
    - Scenario 1
    - Scenario 2

    ## When NOT to use
    - Anti-pattern 1
    - Anti-pattern 2

    ## Examples
    Concrete invocation examples with real arguments

    ## Preconditions
    What must be true before running

    ## Output
    What to expect from the tool
    """,
    risk_level="safe"
)
```

## Writing Effective Descriptions

### Be Specific About Context

**Bad:**
```python
tool=angreal.ToolDescription("Runs tests")
```

**Good:**
```python
tool=angreal.ToolDescription(
    """
    Run the complete test suite including unit and integration tests.

    ## When to use
    - Before committing changes
    - After pulling changes to verify environment
    - As part of CI/CD validation

    ## Output
    Returns exit code 0 if all tests pass, non-zero otherwise.
    """
)
```

### Include Concrete Examples

```python
tool=angreal.ToolDescription(
    """
    Build documentation from source.

    ## Examples
    ```
    # Build to default output
    angreal docs build

    # Build with specific output path
    angreal docs build --output ./site

    # Build in watch mode
    angreal docs build --watch
    ```
    """
)
```

### Document Failure Modes

```python
tool=angreal.ToolDescription(
    """
    Install project dependencies.

    ## Common Failures
    - **Network error**: Check internet connectivity
    - **Permission denied**: May need sudo or virtual environment
    - **Version conflict**: Check requirements.txt

    ## Recovery
    If installation fails:
    1. Clear cache: `angreal dev clean-cache`
    2. Retry installation
    """
)
```

### Describe Relationships

```python
tool=angreal.ToolDescription(
    """
    Run unit tests only (fast).

    ## Related Tasks
    - `test.integration` - Integration tests (slower)
    - `test.all` - Complete test suite
    - `lint.check` - Check style (run before tests)

    ## Typical Workflow
    1. `angreal lint check`
    2. `angreal test unit` - Fast feedback
    3. `angreal test all` - Full validation
    """
)
```

## Examples by Task Type

### Build Task

```python
tool=angreal.ToolDescription(
    """
    Build the project for release distribution.

    ## When to use
    - Creating a release build
    - Before running deployment tasks

    ## Preconditions
    - Dependencies installed (`angreal dev check-deps`)
    - Source compiles without errors

    ## Output
    - Artifacts written to `dist/`
    - Returns 0 on success
    """,
    risk_level="safe"
)
```

### Database Migration

```python
tool=angreal.ToolDescription(
    """
    Apply pending database migrations.

    ## When to use
    - After pulling changes with new migrations
    - During deployment to update schema

    ## When NOT to use
    - On production without backup
    - Without reviewing migration contents

    ## Recovery
    If migration fails:
    1. Check migration logs
    2. Run `angreal db rollback`
    3. Fix and retry
    """,
    risk_level="destructive"
)
```

### Status Check

```python
tool=angreal.ToolDescription(
    """
    Check project status and health.

    ## When to use
    - Starting work to understand state
    - Debugging to gather context
    - Verifying environment setup

    ## Output
    Reports on git branch, dependencies, build state, test summary.
    """,
    risk_level="read_only"
)
```

## Best Practices

### Do

- Write from the agent's perspective
- Include specific, actionable guidance
- Use markdown formatting for structure
- Document preconditions and postconditions
- Show example invocations with realistic arguments
- Explain error scenarios and recovery

### Don't

- Write generic descriptions
- Assume the agent knows project context
- Leave out critical preconditions
- Skip risk_level for destructive operations
- Write walls of text without structure
