# Task Execution

How to properly invoke angreal tasks and handle their output.

## Basic Invocation

### Via MCP

MCP tools accept a JSON object with:
- `command_path` - The task to run (e.g., "test.all")
- `args` - Object containing argument values

```json
{
  "command_path": "test.all",
  "args": {
    "verbose": true
  }
}
```

### Via CLI

```bash
angreal <group> <command> [arguments]

# Examples
angreal test all
angreal test all --verbose
angreal build --release
```

## Arguments

### Flags (Boolean Arguments)

Flags are boolean switches, typically defaulting to `false`:

```bash
# CLI
angreal build --release

# MCP
{"args": {"release": true}}
```

### Value Arguments

Arguments that take a value:

```bash
# CLI
angreal deploy --version v1.2.3

# MCP
{"args": {"version": "v1.2.3"}}
```

### Required Arguments

Some arguments are required. The task will fail if not provided:

```bash
# This will fail - name is required
angreal init

# This works
angreal init --name my-project
```

## Output Interpretation

### Success Output

Tasks return structured JSON via MCP:

```json
{
  "command": "test.all",
  "result": "success",
  "return_value": "0",
  "stdout": "...",
  "stderr": "...",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

Key fields:
- `result`: "success" or "error"
- `return_value`: The function's return value (if any)
- `stdout`: Captured standard output
- `stderr`: Captured standard error

### Error Output

When tasks fail:

```json
{
  "command": "test.all",
  "result": "error",
  "error": "Tests failed: 3 failures",
  "stdout": "...",
  "stderr": "...",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

The `error` field contains the error message. Check `stdout` and `stderr` for details.

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

Always check the result:

```python
# Good: Check result
result = run_angreal_task("test.all")
if result["result"] != "success":
    # Handle failure
    pass

# Bad: Assume success
run_angreal_task("test.all")
proceed_to_deploy()  # Dangerous!
```

### Sequential Tasks

When tasks depend on each other, run sequentially and check each:

```
1. angreal build --release
   → Check success before continuing

2. angreal test all
   → Check success before continuing

3. angreal deploy --version v1.2.3
```

### Independent Tasks

When tasks don't depend on each other, they can run in parallel:

```
# These can run in parallel
- angreal test unit
- angreal test integration
- angreal lint check
```

## Timeouts

Angreal MCP server has a 10-minute timeout for long-running tasks. For tasks that might exceed this:

1. Check if the task has a `--quick` or similar option
2. Consider running via CLI instead of MCP
3. Break into smaller sub-tasks

## Working Directory

Tasks run in the project root (where `.angreal/` is located). Tasks should use `angreal.get_root().parent` to find the project root if they need absolute paths. Note that `angreal.get_root()` returns the path to the `.angreal/` directory, not the project root.

## Capturing Output

For tasks that produce important output:

1. Check `stdout` in the result
2. Look for file paths or artifacts mentioned
3. Some tasks write to specific locations (e.g., `dist/`, `docs/build/`)
