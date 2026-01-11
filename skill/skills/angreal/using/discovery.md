# Task Discovery

How to find and understand available tasks in an angreal project.

## Checking for Angreal Project

Before using tasks, verify you're in an angreal project:

```bash
ls .angreal/
```

If `.angreal/` doesn't exist, you're not in an angreal project.

## Discovering Available Tasks

### Quick Discovery

```bash
angreal tree
```

Shows all commands with arguments and short descriptions:

```
dev: development utilities
  check-deps - Verify required development tools are installed
test: commands for testing the application and library
  all - Run complete test suite (Python, Rust, completion)
  rust [--unit-only] [--integration-only] - Run Rust tests
  python - Run Python unit tests
docs: commands for documentation tasks
  build [--draft] - Build documentation site
  serve [--prod] - Start local documentation server
```

### Detailed Discovery (AI Guidance)

```bash
angreal tree --long
```

Includes full ToolDescription prose for each command:
- **When to use** - Appropriate scenarios
- **When NOT to use** - Situations to avoid
- **Examples** - Concrete invocations
- **Risk level** - safe, read_only, or destructive

### Traditional Help

```bash
# List all commands
angreal --help

# Get help for a specific command
angreal test rust --help
```

## Understanding Task Output

### Short Format (`angreal tree`)

Each line shows:
- Command name
- Arguments in brackets: `[--flag]` or `[--option=<type>]`
- Short description from `about`

### Long Format (`angreal tree --long`)

Adds the full `ToolDescription` content that task authors write to guide AI usage:

```
test: commands for testing
  all - Run complete test suite

    Run the complete test suite including Python, Rust, completion tests.

    ## When to use
    - Before major releases
    - After significant changes

    ## When NOT to use
    - During rapid development cycles

    ## Examples
    ```
    angreal test all
    ```
    Risk level: safe
```

**Key insight**: ToolDescriptions are written by task authors specifically to guide AI agents. Trust them.

## Task Groups

Tasks are organized into groups by function:

| Common Group | Purpose |
|--------------|---------|
| `dev` | Development utilities |
| `test` | Testing commands |
| `docs` | Documentation generation |
| `build` | Build and compilation |
| `deploy` | Deployment and release |

Groups can be nested: `docker compose up` means the `up` command in the `compose` subgroup of `docker`.

## Choosing the Right Task

### For Testing
1. Run `angreal tree` to see available test commands
2. Look for `all`, `unit`, `integration` variants
3. Use `angreal tree --long` to understand coverage

### For Building
1. Look for `build` or `dev` groups
2. Check for `--release` or `--draft` flags
3. Note any prerequisites (like `check-deps`)

### For Documentation
1. Look for `docs` group
2. Typically `build` and `serve` commands
3. May require external tools (hugo, mkdocs)

### When Unsure
1. Run `angreal tree` first to see all available commands
2. Use `angreal tree --long` for detailed guidance
3. Start with read-only tasks to explore safely
