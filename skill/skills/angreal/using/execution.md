# Task Execution

How to properly invoke angreal tasks and handle their output.

## Basic Invocation

```bash
angreal <group> <command> [arguments]

# Examples
angreal test all
angreal test rust --unit-only
angreal build --release
angreal docs serve --prod
```

## Arguments

### Flags (Boolean Arguments)

Flags are boolean switches, typically defaulting to `false`:

```bash
angreal build --release
angreal test rust --unit-only
```

### Value Arguments

Arguments that take a value:

```bash
angreal deploy --version v1.2.3
angreal test completion --shell=bash
```

### Required Arguments

Some arguments are required. The task will fail if not provided. Check `angreal tree` to see which arguments are available.

## Output Interpretation

### Success

Tasks print output to stdout/stderr. A zero exit code indicates success:

```bash
angreal test all
# ... test output ...
# Exit code 0 = success
```

### Failure

Non-zero exit codes indicate failure. Error messages appear in output:

```bash
angreal test all
# FAIL: The following test suites failed: Python tests
# Exit code 1 = failure
```

## Handling Failures

### Expected Failures

Some failures are normal:
- Test failures indicate code issues, not task issues
- Build failures indicate compilation errors
- Lint failures indicate code style issues

**Action**: Fix the underlying issue, then re-run.

### Task Errors

Errors in the task itself:
- Missing dependencies
- Invalid arguments
- Permission issues

**Action**: Check error message, resolve prerequisites, retry.

### Environment Errors

Problems with the execution environment:
- Not in angreal project
- Python not available
- Missing tools

**Action**: Verify environment, install dependencies, check paths.

## Execution Patterns

### Run and Check

Always check the exit code before proceeding:

```bash
angreal build --release && angreal test all && angreal deploy
```

### Sequential Tasks

When tasks depend on each other, run sequentially and verify each succeeds:

```
1. angreal build --release
   → Check exit code before continuing

2. angreal test all
   → Check exit code before continuing

3. angreal deploy --version v1.2.3
```

### Independent Tasks

When tasks don't depend on each other, they can run in parallel.

## Working Directory

Tasks run in the project root (where `.angreal/` is located). Tasks should use `angreal.get_root().parent` to find the project root if they need absolute paths.

**Note**: `angreal.get_root()` returns the path to the `.angreal/` directory, not the project root. Use `.parent` to get the actual project root.

## Capturing Output

For tasks that produce important output:

1. Read stdout/stderr from the command
2. Look for file paths or artifacts mentioned
3. Some tasks write to specific locations (e.g., `dist/`, `docs/build/`)
