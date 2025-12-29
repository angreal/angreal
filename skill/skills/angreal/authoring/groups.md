# Command Groups

How to organize related tasks using groups.

## Why Groups?

Groups organize related commands into hierarchies:

```
angreal test all          # test group
angreal test unit
angreal test integration

angreal docs build        # docs group
angreal docs preview

angreal docker build      # nested: docker group
angreal docker compose up # nested: docker.compose group
```

Without groups, all commands are top-level, which gets messy in large projects.

## Creating a Group

Use `command_group()` to create a reusable group decorator:

```python
import angreal

# Create a reusable group decorator
test = angreal.command_group(name="test", about="Testing commands")

@test()
@angreal.command(name="all", about="Run all tests")
def test_all():
    pass

@test()
@angreal.command(name="unit", about="Run unit tests")
def test_unit():
    pass
```

This creates:
- `angreal test all`
- `angreal test unit`

## Group Decorator Order

**Important**: The `@group()` decorator must come BEFORE `@command`:

```python
# Correct order
@test()                           # Group first
@angreal.command(name="all")      # Command second
def test_all():
    pass

# Wrong order - will fail
@angreal.command(name="all")      # This won't work!
@test()
def test_all():
    pass
```

## Multiple Groups in One File

A single task file can define multiple groups:

```python
import angreal

# Define groups
dev = angreal.command_group(name="dev", about="Development utilities")
test = angreal.command_group(name="test", about="Testing commands")

# Dev commands
@dev()
@angreal.command(name="setup", about="Set up development environment")
def dev_setup():
    pass

@dev()
@angreal.command(name="check-deps", about="Check dependencies")
def dev_check_deps():
    pass

# Test commands
@test()
@angreal.command(name="all", about="Run all tests")
def test_all():
    pass
```

## Nested Groups

Groups can be nested for deeper hierarchies:

```python
import angreal

# Parent group
docker = angreal.command_group(name="docker", about="Docker commands")

# Child group (applied after parent)
compose = angreal.command_group(name="compose", about="Docker Compose commands")

@docker()
@angreal.command(name="build", about="Build Docker image")
def docker_build():
    pass

@docker()
@compose()
@angreal.command(name="up", about="Start services")
def docker_compose_up():
    pass

@docker()
@compose()
@angreal.command(name="down", about="Stop services")
def docker_compose_down():
    pass
```

This creates:
- `angreal docker build`
- `angreal docker compose up`
- `angreal docker compose down`

## Group Descriptions

Groups can have descriptions shown in help:

```python
dev = angreal.command_group(
    name="dev",
    about="Development utilities and environment setup"
)
```

When user runs `angreal dev --help`, they see this description.

## Inline Group Decorator

For one-off groups, use `@angreal.group()` directly:

```python
import angreal

@angreal.group(name="utils", about="Utility commands")
@angreal.command(name="clean", about="Clean build artifacts")
def utils_clean():
    pass
```

But `command_group()` is preferred when multiple commands share a group.

## Group Organization Patterns

### By Domain

```python
db = angreal.command_group(name="db", about="Database operations")
api = angreal.command_group(name="api", about="API operations")
ui = angreal.command_group(name="ui", about="UI operations")
```

### By Lifecycle

```python
dev = angreal.command_group(name="dev", about="Development")
test = angreal.command_group(name="test", about="Testing")
build = angreal.command_group(name="build", about="Building")
deploy = angreal.command_group(name="deploy", about="Deployment")
```

### By Tool

```python
docker = angreal.command_group(name="docker", about="Docker commands")
npm = angreal.command_group(name="npm", about="NPM commands")
cargo = angreal.command_group(name="cargo", about="Cargo commands")
```

## Best Practices

### Keep Groups Focused

Each group should have a clear, single purpose:

```python
# Good: Focused groups
test = angreal.command_group(name="test", about="Run tests")
lint = angreal.command_group(name="lint", about="Code linting")

# Bad: Mixed concerns
misc = angreal.command_group(name="misc", about="Various commands")
```

### Limit Nesting Depth

Two levels is usually enough:

```python
# Good: Two levels
angreal docker compose up

# Questionable: Three levels
angreal docker compose service restart
```

### Consistent Naming

Use consistent naming across the project:

```python
# Consistent verb-based names
test = angreal.command_group(name="test")  # "test all", "test unit"
build = angreal.command_group(name="build")  # "build release", "build debug"

# Or consistent noun-based names
tests = angreal.command_group(name="tests")  # "tests run", "tests list"
builds = angreal.command_group(name="builds")  # "builds create", "builds list"
```

Pick one style and stick with it.
