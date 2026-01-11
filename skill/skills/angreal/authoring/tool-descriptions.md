# Tool Descriptions

How to write effective ToolDescription prose for AI agent guidance.

## Why Tool Descriptions Matter

When AI agents discover tasks via `angreal tree --long`, they see:
- The command name
- Argument signatures
- **Your ToolDescription**

The ToolDescription is the primary guidance agents use to decide whether to call your task and how to use it correctly. Think of it as a mini-prompt that teaches agents when and how to use your tool.

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
        - After successful build and test passes
        - When user explicitly requests deployment
        - As the final step in a release workflow

        ## When NOT to use
        - Directly from feature branches
        - Without running tests first
        - When build artifacts don't exist

        ## Examples
        ```
        angreal deploy --env staging
        angreal deploy --env production --version v1.2.3
        ```

        ## Preconditions
        - Build must complete successfully
        - All tests must pass
        - User must have deployment credentials configured
        """,
        risk_level="destructive"
    )
)
@angreal.argument(name="env", long="env", required=True, help="Target environment")
@angreal.argument(name="version", long="version", help="Version tag")
def deploy(env, version=None):
    pass
```

## ToolDescription Constructor

```python
angreal.ToolDescription(
    description,      # The prose description (required)
    *,
    risk_level=None   # "safe", "read_only", or "destructive" (default: "safe")
)
```

## Risk Levels

| Level | Meaning | Use When |
|-------|---------|----------|
| `safe` | No destructive effects | Default. Build, test, lint tasks |
| `read_only` | Only reads/reports | Status checks, info gathering |
| `destructive` | May modify or delete | Deploy, clean, database migrations |

Risk levels help agents make informed decisions about tool use.

## Writing Effective Descriptions

### Structure Your Description

Use markdown sections for clarity:

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
    Concrete invocation examples

    ## Preconditions
    What must be true before running

    ## Output
    What to expect from the tool
    """,
    risk_level="safe"
)
```

### Be Specific About Context

Bad:
```python
tool=angreal.ToolDescription("Runs tests")
```

Good:
```python
tool=angreal.ToolDescription(
    """
    Run the complete test suite including unit and integration tests.

    ## When to use
    - Before committing changes to verify nothing is broken
    - After pulling changes to verify local environment works
    - As part of CI/CD validation

    ## Output
    Returns exit code 0 if all tests pass, non-zero otherwise.
    Test results are printed to stdout with coverage summary.
    """
)
```

### Include Concrete Examples

Show exact invocations:

```python
tool=angreal.ToolDescription(
    """
    Build documentation from source.

    ## Examples
    ```
    # Build docs to default output directory
    angreal docs build

    # Build with specific output path
    angreal docs build --output ./site

    # Build in watch mode for development
    angreal docs build --watch
    ```
    """
)
```

### Document Failure Modes

Help agents understand what can go wrong:

```python
tool=angreal.ToolDescription(
    """
    Install project dependencies.

    ## Common Failures
    - **Network error**: Check internet connectivity
    - **Permission denied**: May need sudo or virtual environment
    - **Version conflict**: Check requirements.txt for pinned versions

    ## Recovery
    If installation fails, try:
    1. Clear cache: `angreal dev clean-cache`
    2. Retry installation
    """
)
```

### Describe Relationships

Explain how tasks relate to each other:

```python
tool=angreal.ToolDescription(
    """
    Run unit tests only (fast).

    ## Related Tasks
    - `test.integration` - Run integration tests (slower)
    - `test.all` - Run complete test suite
    - `lint.check` - Check code style (run before tests)

    ## Typical Workflow
    1. `angreal lint check` - Fix style issues first
    2. `angreal test unit` - Fast feedback loop
    3. `angreal test all` - Full validation before commit
    """
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

- Write generic descriptions that could apply to any task
- Assume the agent knows project-specific context
- Leave out critical preconditions
- Skip the risk_level for destructive operations
- Write walls of text without structure

## Examples by Task Type

### Build Task

```python
tool=angreal.ToolDescription(
    """
    Build the project for release distribution.

    ## When to use
    - Creating a release build for deployment
    - Generating optimized artifacts
    - Before running deployment tasks

    ## Preconditions
    - All dependencies installed (`angreal dev check-deps`)
    - Source code compiles without errors

    ## Output
    - Build artifacts written to `dist/`
    - Build logs written to stdout
    - Returns 0 on success, non-zero on failure
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
    - After pulling changes that include new migrations
    - During deployment to update schema
    - Setting up new development environment

    ## When NOT to use
    - On production without backup
    - Without reviewing migration contents first

    ## Preconditions
    - Database connection configured
    - Database backup exists (for production)

    ## Recovery
    If migration fails:
    1. Check migration logs for error
    2. Fix the issue in migration file
    3. Run `angreal db rollback` to undo partial changes
    4. Retry migration
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
    - Starting work to understand current state
    - Debugging issues to gather context
    - Verifying environment is correctly configured

    ## Output
    Reports on:
    - Git branch and uncommitted changes
    - Dependency status
    - Build state
    - Test results summary
    """,
    risk_level="read_only"
)
```
