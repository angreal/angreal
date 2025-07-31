---
title: "Use Docker Compose Integration"
weight: 40
---

# Use Docker Compose Integration

This guide shows how to use Angreal's Docker Compose integration to manage multi-container applications.

## Quick Reference Card

### Essential Commands

```python
from angreal.integrations.docker import compose

# Initialize
stack = compose("docker-compose.yml")                    # Basic
stack = compose("docker-compose.yml", "project-name")    # With project name

# Lifecycle
stack.up(detach=True)           # Start services
stack.down()                    # Stop and remove
stack.restart()                 # Restart all
stack.stop()                    # Stop (keep containers)
stack.start()                   # Start stopped containers

# Monitoring
stack.ps()                      # List containers
stack.logs(tail="50")           # View logs
stack.config()                  # Validate config

# Operations
stack.build()                   # Build images
stack.pull()                    # Pull images
stack.exec("web", ["bash"])     # Run command
```

### Common Patterns

```python
# Development workflow
stack.down(volumes=True)                          # Clean slate
stack.up(detach=True, build=True)                # Rebuild & start
stack.logs(services=["web"], follow=True)         # Watch logs

# Service management
stack.up(services=["db"])                         # Start specific
stack.restart(services=["web", "worker"])         # Restart multiple
stack.exec("db", ["psql", "-U", "postgres"])     # Database access

# Debugging
result = stack.up()
if not result.success:
    print(result.stderr)                          # Check errors
```

### Key Options

| Method | Key Parameters | Purpose |
|--------|---------------|----------|
| `up()` | `detach`, `build`, `services` | Start containers |
| `down()` | `volumes`, `remove_orphans` | Clean up |
| `logs()` | `follow`, `tail`, `services` | View output |
| `exec()` | `user`, `env`, `workdir` | Run commands |
| `build()` | `no_cache`, `pull` | Build images |

## Prerequisites

- Docker and Docker Compose installed
- A `docker-compose.yml` file in your project
- Angreal with Docker integration

## Quick Start

```python
from angreal.integrations.docker import compose

# Start services
stack = compose("docker-compose.yml")
result = stack.up(detach=True)

if result.success:
    print("Services started")
else:
    print(f"Failed: {result.stderr}")
```

## Common Operations

### Start Services

```python
# Basic start
stack.up(detach=True)

# Build and start
stack.up(detach=True, build=True)

# Start specific services
stack.up(detach=True, services=["web", "db"])
```

### Monitor Services

```python
# Check status
result = stack.ps()
print(result.stdout)

# View logs
stack.logs(tail="100")
stack.logs(services=["web"], follow=True)
```

### Execute Commands

```python
# Run commands in containers
stack.exec("web", ["bash"])
stack.exec("web", ["python", "manage.py", "migrate"])
stack.exec("db", ["psql", "-U", "postgres"], user="postgres")
```

### Manage Lifecycle

```python
# Restart services
stack.restart()
stack.restart(services=["web"])

# Stop without removing
stack.stop()

# Start stopped containers
stack.start()
```

### Build and Update

```python
# Build images
stack.build()
stack.build(no_cache=True, services=["web"])

# Pull latest images
stack.pull()
```

### Clean Up

```python
# Stop and remove containers
stack.down()

# Also remove volumes
stack.down(volumes=True)

# Remove orphaned containers
stack.down(remove_orphans=True)
```

## Working with Environments

Use different compose files for different environments:

```python
# Development
dev = compose("docker-compose.dev.yml", project_name="myapp-dev")
dev.up(detach=True)

# Production
prod = compose("docker-compose.prod.yml", project_name="myapp-prod")
prod.up(detach=True)
```

## Create Angreal Tasks

### Basic Task

```python
# .angreal/docker_tasks.py
import angreal
from angreal.integrations.docker import compose

@angreal.command(name="dev-up", about="Start development stack")
@angreal.option("--build", is_flag=True, help="Rebuild images")
def start_dev(build=False):
    """Start development services"""
    stack = compose("docker-compose.dev.yml")

    result = stack.up(detach=True, build=build)
    if result.success:
        print("✓ Services started")
        print(stack.ps().stdout)
    else:
        print(f"✗ Failed: {result.stderr}")
        return 1
```

### Database Reset Task

```python
@angreal.command(name="db-reset", about="Reset database")
def reset_database():
    """Reset database to clean state"""
    stack = compose("docker-compose.yml")

    # Stop and remove volumes
    stack.down(volumes=True)

    # Start fresh database
    result = stack.up(detach=True, services=["db"])
    if not result.success:
        return 1

    # Run migrations
    import time
    time.sleep(5)  # Wait for DB

    result = stack.exec("web", ["python", "manage.py", "migrate"])
    return 0 if result.success else 1
```

### Test Runner

```python
@angreal.command(name="test", about="Run tests in Docker")
def run_tests():
    """Run integration tests"""
    stack = compose("docker-compose.test.yml")

    try:
        # Start test environment
        if not stack.up(detach=True, build=True).success:
            return 1

        # Run tests
        result = stack.exec("test", ["pytest", "-v"])
        print(result.stdout)
        return 0 if result.success else 1

    finally:
        # Clean up
        stack.down(volumes=True)
```

## Error Handling

All operations return a `ComposeResult` with:
- `success`: Boolean indicating success
- `exit_code`: Process exit code
- `stdout`: Command output
- `stderr`: Error messages

```python
result = stack.up(detach=True)
if not result.success:
    print(f"Error: {result.stderr}")
    print(f"Exit code: {result.exit_code}")
```

## Best Practices

1. **Check availability**: Use `DockerCompose.is_available()` before operations
2. **Use project names**: Isolate environments with unique project names
3. **Check results**: Always verify `result.success`
4. **Clean up**: Use `down()` to remove resources when done
5. **Handle errors**: Check `stderr` for error details

## Quick Reference

```python
from angreal.integrations.docker import compose

stack = compose("docker-compose.yml", project_name="myapp")

# Lifecycle
stack.up(detach=True, build=True)    # Start services
stack.down(volumes=True)              # Stop and clean up
stack.restart(services=["web"])       # Restart services

# Monitoring
stack.ps(all=True)                    # List containers
stack.logs(follow=True, tail="100")   # View logs

# Operations
stack.build(no_cache=True)            # Build images
stack.exec("web", ["bash"])           # Run commands
stack.pull()                          # Update images
```

See the [API Reference](/reference/python-api/integrations/docker-compose/) for complete details.
