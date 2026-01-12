---
name: angreal-usage
description: This skill should be used when the user asks to "run an angreal task", "execute angreal command", "discover angreal tasks", "list angreal commands", "use angreal tree", "find available tasks", "what tasks are available", or needs guidance on running tasks, interpreting task output, task workflows, or choosing the right task for a job.
version: 2.8.0
---

# Using Angreal Tasks

Learn to discover, run, and chain angreal tasks effectively.

## Prerequisites

- Working within an angreal project (has `.angreal/` directory)
- `angreal` CLI installed and available

## Discovering Tasks

### Quick Discovery

```bash
angreal tree
```

Shows all commands with arguments and short descriptions:

```
dev: development utilities
  check-deps - Verify required development tools are installed
test: commands for testing the application
  all - Run complete test suite
  rust [--unit-only] - Run Rust tests
docs: commands for documentation
  build [--draft] - Build documentation site
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
angreal --help              # List all commands
angreal test rust --help    # Help for specific command
```

## Running Tasks

### Basic Invocation

```bash
angreal <group> <command> [arguments]

# Examples
angreal test all
angreal test rust --unit-only
angreal build --release
angreal docs serve --prod
```

### Arguments

**Flags** (boolean switches):
```bash
angreal build --release
angreal test rust --unit-only
```

**Value arguments**:
```bash
angreal deploy --version v1.2.3
angreal test completion --shell=bash
```

## Output and Exit Codes

- Exit code `0` = success
- Non-zero exit code = failure

```bash
angreal test all && angreal deploy  # Chain with &&
```

## Common Workflows

### Before Committing

```bash
angreal test unit      # Fast unit tests first
angreal test all       # Full suite if units pass
angreal lint check     # Check code style
```

### Release Process

```bash
angreal test all           # Ensure tests pass
angreal build --release    # Create release build
angreal docs build         # Update documentation
angreal deploy staging     # Deploy to staging
```

### Debugging

```bash
angreal test <specific-test> --verbose  # Verbose output
# Check stdout/stderr for error details
# Fix issue, re-run specific test
angreal test all                        # Verify no regressions
```

## Task Groups

Tasks are organized by function:

| Group | Purpose |
|-------|---------|
| `dev` | Development utilities |
| `test` | Testing commands |
| `docs` | Documentation |
| `build` | Build and compilation |
| `deploy` | Deployment and release |

Groups can nest: `angreal docker compose up`

## Choosing the Right Task

1. Run `angreal tree` to see available commands
2. Use `angreal tree --long` for detailed guidance
3. Start with read-only tasks to explore safely
4. Trust ToolDescriptions - they're written to guide you
