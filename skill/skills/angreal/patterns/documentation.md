# Documentation Patterns

How to document angreal tasks for users and AI agents.

## Documentation Layers

Angreal tasks have multiple documentation touchpoints:

| Layer | Audience | Location |
|-------|----------|----------|
| `about` | CLI users | `--help` output |
| `long_about` | CLI users | `--help` detailed output |
| `help` (args) | CLI users | Argument help text |
| `ToolDescription` | AI agents | MCP tool discovery |
| Code comments | Developers | Source code |
| External docs | All users | README, docs site |

## CLI Documentation

### about Parameter

Short description shown in command listings:

```python
@angreal.command(
    name="build",
    about="Build the project for distribution"  # Keep under 60 chars
)
```

Output in `angreal --help`:
```
COMMANDS:
    build    Build the project for distribution
    test     Run the test suite
    deploy   Deploy to environment
```

### long_about Parameter

Extended description for detailed help:

```python
@angreal.command(
    name="deploy",
    about="Deploy to environment",
    long_about="""
    Deploy the application to a specified environment.

    This command handles the full deployment lifecycle:
    - Builds optimized release artifacts
    - Uploads to target environment
    - Runs database migrations
    - Verifies deployment health

    Supported environments: development, staging, production
    """
)
```

Output in `angreal deploy --help`:
```
Deploy the application to a specified environment.

This command handles the full deployment lifecycle:
- Builds optimized release artifacts
- Uploads to target environment
- Runs database migrations
- Verifies deployment health

Supported environments: development, staging, production

USAGE:
    angreal deploy [OPTIONS]
...
```

### Argument Help Text

Document each argument clearly:

```python
@angreal.argument(
    name="env",
    long="env",
    required=True,
    help="Target environment: development, staging, or production"
)
@angreal.argument(
    name="version",
    long="version",
    help="Version tag to deploy (default: latest build)"
)
@angreal.argument(
    name="dry_run",
    long="dry-run",
    is_flag=True,
    takes_value=False,
    help="Show what would be done without making changes"
)
```

## MCP/AI Documentation

### ToolDescription for Agents

AI agents need richer context than CLI users. Use `ToolDescription`:

```python
@angreal.command(
    name="deploy",
    about="Deploy to environment",
    tool=angreal.ToolDescription(
        """
        Deploy the application to a specified environment.

        ## When to use
        - After all tests pass on staging
        - When release is approved by stakeholder
        - As final step in release workflow

        ## When NOT to use
        - Directly from feature branches (deploy from main only)
        - Without running tests first
        - Without staging validation for production deploys

        ## Examples
        ```
        # Deploy to staging for testing
        angreal deploy --env staging

        # Deploy specific version to production
        angreal deploy --env production --version v1.2.3

        # Dry run to see what would happen
        angreal deploy --env production --dry-run
        ```

        ## Preconditions
        - Build artifacts exist in dist/
        - Tests have passed (check `angreal test all`)
        - User has deployment credentials configured
        - For production: staging validation complete

        ## Output
        - Deployment progress printed to stdout
        - Returns 0 on success, 1 on failure
        - Deployment URL printed on success
        """,
        risk_level="destructive"
    )
)
```

### Structuring ToolDescription

Use consistent markdown sections:

```markdown
## When to use
Scenarios where this task is appropriate.

## When NOT to use
Anti-patterns and contraindications.

## Examples
Concrete CLI invocations with real arguments.

## Preconditions
What must be true before running.

## Output
What to expect from successful execution.

## Related Tasks
Other tasks that complement this one.

## Troubleshooting
Common issues and how to resolve them.
```

## Code Documentation

### Docstrings

Document task functions for developers:

```python
@angreal.command(name="build", about="Build the project")
@angreal.argument(name="release", long="release", is_flag=True, takes_value=False)
def build(release=False):
    """
    Build project artifacts.

    This function:
    1. Validates the build environment
    2. Compiles source code
    3. Generates distribution artifacts

    Args:
        release: If True, build optimized release artifacts.
                 If False, build debug artifacts with symbols.

    Returns:
        0 on success, 1 on failure

    Raises:
        No exceptions raised - errors returned as exit codes
    """
    pass
```

### Inline Comments

Comment non-obvious logic:

```python
def deploy(env, version=None):
    # Use latest build if no version specified
    version = version or get_latest_build_version()

    # Staging deploys skip the approval check
    if env == "production":
        if not check_approval(version):
            print("Error: Production deploy requires approval")
            return 1

    # Blue-green deployment: deploy to inactive, then switch
    inactive_slot = get_inactive_slot(env)
    deploy_to_slot(inactive_slot, version)
    switch_active_slot(env, inactive_slot)
```

## External Documentation

### README Task Reference

Include task summary in project README:

```markdown
## Available Tasks

### Development
- `angreal dev setup` - Set up development environment
- `angreal dev check-deps` - Verify dependencies installed

### Testing
- `angreal test all` - Run complete test suite
- `angreal test unit` - Run unit tests only
- `angreal test integration` - Run integration tests

### Deployment
- `angreal deploy --env <env>` - Deploy to environment

Run `angreal --help` for full command reference.
```

### Detailed Task Documentation

For complex tasks, create dedicated documentation:

```markdown
# Deployment Guide

## Overview
The `deploy` command handles application deployment to all environments.

## Usage
```bash
angreal deploy --env <environment> [--version <version>] [--dry-run]
```

## Environments

### Development
- Auto-deployed on commit to main
- No approval required
- URL: https://dev.example.com

### Staging
- Deploy manually for testing
- No approval required
- URL: https://staging.example.com

### Production
- Requires approval from tech lead
- Must pass staging validation first
- URL: https://example.com

## Prerequisites
1. AWS credentials configured
2. VPN connected (for staging/prod)
3. Deployment approval (prod only)

## Troubleshooting

### "Permission denied" error
Ensure AWS credentials are current:
```bash
aws sso login
```

### "Health check failed" error
Check application logs:
```bash
angreal logs --env <environment> --tail 100
```
```

## Best Practices

### Consistency

Use consistent terminology:
- Same verbs across similar tasks (run tests, run build → test, build)
- Same argument names for common concepts (--env, --verbose, --output)
- Same section headers in ToolDescriptions

### Audience Awareness

| Audience | Needs | Provide |
|----------|-------|---------|
| New users | Getting started | about, examples |
| Power users | Full options | long_about, all args |
| AI agents | Decision guidance | ToolDescription |
| Developers | Implementation | docstrings, comments |

### Keep Updated

Documentation rots. When changing tasks:
1. Update `about` if behavior changes
2. Update `ToolDescription` if usage guidance changes
3. Update argument `help` if semantics change
4. Update external docs if user workflow changes
