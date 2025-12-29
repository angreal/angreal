# Common Workflows

Multi-task workflows for common development scenarios.

## Development Setup

When starting work on a project:

```
1. angreal dev check-deps
   → Verify all required tools are installed

2. (If using virtual environment)
   angreal dev setup-venv
   → Create/activate Python environment

3. angreal build
   → Build the project to verify setup
```

## Test Workflow

Before committing changes:

```
1. angreal test unit
   → Run fast unit tests first

2. angreal test integration
   → Run slower integration tests

3. angreal lint check
   → Check code style

4. (If all pass) Commit changes
```

Or if an `all` command exists:

```
1. angreal test all
   → Runs complete test suite
```

## Documentation Workflow

When updating documentation:

```
1. angreal docs build
   → Generate documentation

2. angreal docs preview
   → Start local preview server

3. (Review in browser)

4. (Make edits, rebuild)

5. (When satisfied) Commit changes
```

## Release Workflow

Typical release process:

```
1. angreal test all
   → Ensure tests pass

2. angreal build --release
   → Create release build

3. angreal docs build
   → Update documentation

4. angreal deploy staging
   → Deploy to staging (if available)

5. (Manual verification)

6. angreal deploy production --version vX.Y.Z
   → Deploy to production
```

## Debugging Workflow

When investigating issues:

```
1. angreal test <specific-test> --verbose
   → Run failing test with verbose output

2. Check stdout/stderr for error details

3. (Fix issue)

4. angreal test <specific-test>
   → Verify fix

5. angreal test all
   → Ensure no regressions
```

## Continuous Integration

Typical CI pipeline tasks:

```
# Install dependencies
angreal dev check-deps

# Run linting
angreal lint check

# Run tests
angreal test all

# Build
angreal build --release

# (Optional) Build docs
angreal docs build
```

## Task Chaining

Some tasks naturally chain together:

### Build → Test → Deploy
```
angreal build --release && angreal test all && angreal deploy staging
```

### Clean → Build → Test
```
angreal clean && angreal build && angreal test all
```

## Error Recovery

### Test Failures
1. Read test output to identify failures
2. Fix code issues
3. Re-run specific failing tests first
4. Run full suite when specific tests pass

### Build Failures
1. Check compiler/build errors
2. Verify dependencies are installed
3. Try clean build: `angreal clean && angreal build`

### Deployment Failures
1. Check deployment logs
2. Verify credentials/permissions
3. Check target environment status
4. Retry with verbose logging if available

## Project-Specific Workflows

Projects often have custom workflows. Check:

1. `angreal --help` for available task groups
2. README or CONTRIBUTING docs for workflow guidance
3. `.angreal/` directory for task implementations
4. Tool descriptions for task-specific workflows
