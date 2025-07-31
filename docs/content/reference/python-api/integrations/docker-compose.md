---
title: "angreal.integrations.docker.compose"
weight: 25
---

# Docker Compose Integration

The Docker Compose integration provides a Pythonic interface to manage multi-container Docker applications using Docker Compose. It uses subprocess execution to ensure full compatibility with all Docker Compose features and versions.

## Functions

### compose()

Create a DockerCompose instance for managing a compose stack.

```python
compose(file: str, project_name: Optional[str] = None) -> DockerCompose
```

**Parameters:**
- `file` (str): Path to the docker-compose.yml file
- `project_name` (Optional[str]): Override the default project name

**Returns:**
- `DockerCompose`: An instance for managing the compose stack

**Example:**
```python
from angreal.integrations.docker import compose

# Basic usage
stack = compose("docker-compose.yml")

# With custom project name
stack = compose("docker-compose.dev.yml", project_name="myapp-dev")
```

## Classes

### DockerCompose

Manages Docker Compose operations for a specific compose file.

#### Properties

- `compose_file` (str): Path to the compose file
- `working_dir` (str): Working directory for compose operations
- `project_name` (Optional[str]): Project name if specified

#### Methods

##### is_available()

Check if Docker Compose is available on the system.

```python
@staticmethod
is_available() -> bool
```

**Returns:**
- `bool`: True if Docker Compose (v1 or v2) is available

**Example:**
```python
if not DockerCompose.is_available():
    print("Docker Compose is not installed")
    return
```

##### up()

Start services defined in the compose file.

```python
up(detach: bool = True, 
   build: bool = False, 
   remove_orphans: bool = False,
   force_recreate: bool = False,
   no_recreate: bool = False,
   services: Optional[List[str]] = None) -> ComposeResult
```

**Parameters:**
- `detach` (bool): Run containers in the background (default: True)
- `build` (bool): Build images before starting containers (default: False)
- `remove_orphans` (bool): Remove containers for services not in compose file (default: False)
- `force_recreate` (bool): Recreate containers even if configuration unchanged (default: False)
- `no_recreate` (bool): Don't recreate containers if they exist (default: False)
- `services` (Optional[List[str]]): Specific services to start (default: all services)

**Returns:**
- `ComposeResult`: Operation result with success status and output

**Example:**
```python
# Start all services in detached mode
result = stack.up(detach=True)

# Build and start specific services
result = stack.up(build=True, services=["web", "db"])

# Force recreate all containers
result = stack.up(force_recreate=True, remove_orphans=True)
```

##### down()

Stop and remove containers, networks, and optionally volumes.

```python
down(volumes: bool = False,
     remove_orphans: bool = False,
     timeout: Optional[str] = None) -> ComposeResult
```

**Parameters:**
- `volumes` (bool): Remove named volumes (default: False)
- `remove_orphans` (bool): Remove containers for services not in compose file (default: False)
- `timeout` (Optional[str]): Shutdown timeout in seconds (default: 10)

**Returns:**
- `ComposeResult`: Operation result

**Example:**
```python
# Stop and remove containers
result = stack.down()

# Also remove volumes
result = stack.down(volumes=True)

# Custom timeout
result = stack.down(timeout="30")
```

##### restart()

Restart services.

```python
restart(services: Optional[List[str]] = None,
        timeout: Optional[str] = None) -> ComposeResult
```

**Parameters:**
- `services` (Optional[List[str]]): Specific services to restart (default: all)
- `timeout` (Optional[str]): Shutdown timeout in seconds

**Returns:**
- `ComposeResult`: Operation result

**Example:**
```python
# Restart all services
result = stack.restart()

# Restart specific service
result = stack.restart(services=["web"])
```

##### logs()

View output from services.

```python
logs(services: Optional[List[str]] = None,
     follow: bool = False,
     timestamps: bool = False,
     tail: Optional[str] = None,
     since: Optional[str] = None) -> ComposeResult
```

**Parameters:**
- `services` (Optional[List[str]]): Specific services (default: all)
- `follow` (bool): Follow log output (default: False)
- `timestamps` (bool): Show timestamps (default: False)
- `tail` (Optional[str]): Number of lines to show from end
- `since` (Optional[str]): Show logs since timestamp or relative time

**Returns:**
- `ComposeResult`: Operation result with logs in stdout

**Example:**
```python
# Get last 100 lines from web service
result = stack.logs(services=["web"], tail="100")

# Follow logs with timestamps
result = stack.logs(follow=True, timestamps=True)

# Logs from last hour
result = stack.logs(since="1h")
```

##### ps()

List containers.

```python
ps(all: bool = False,
   quiet: bool = False,
   services: bool = False,
   filter_services: Optional[List[str]] = None) -> ComposeResult
```

**Parameters:**
- `all` (bool): Show all containers including stopped (default: False)
- `quiet` (bool): Only display container IDs (default: False)
- `services` (bool): Display services (default: False)
- `filter_services` (Optional[List[str]]): Filter by service names

**Returns:**
- `ComposeResult`: Operation result with container list

**Example:**
```python
# List running containers
result = stack.ps()

# Show all containers
result = stack.ps(all=True)

# List services only
result = stack.ps(services=True)
```

##### build()

Build or rebuild services.

```python
build(services: Optional[List[str]] = None,
      no_cache: bool = False,
      pull: bool = False,
      parallel: bool = False) -> ComposeResult
```

**Parameters:**
- `services` (Optional[List[str]]): Specific services to build (default: all)
- `no_cache` (bool): Don't use cache (default: False)
- `pull` (bool): Always pull newer base images (default: False)
- `parallel` (bool): Build in parallel (default: False)

**Returns:**
- `ComposeResult`: Operation result

**Example:**
```python
# Build all services
result = stack.build()

# Force rebuild without cache
result = stack.build(no_cache=True, pull=True)

# Build specific service
result = stack.build(services=["web"])
```

##### start()

Start existing containers.

```python
start(services: Optional[List[str]] = None) -> ComposeResult
```

**Parameters:**
- `services` (Optional[List[str]]): Specific services to start

**Returns:**
- `ComposeResult`: Operation result

##### stop()

Stop running containers without removing them.

```python
stop(services: Optional[List[str]] = None,
     timeout: Optional[str] = None) -> ComposeResult
```

**Parameters:**
- `services` (Optional[List[str]]): Specific services to stop
- `timeout` (Optional[str]): Shutdown timeout in seconds

**Returns:**
- `ComposeResult`: Operation result

##### exec()

Execute a command in a running container.

```python
exec(service: str,
     command: List[str],
     detach: bool = False,
     tty: bool = True,
     user: Optional[str] = None,
     workdir: Optional[str] = None,
     env: Optional[Dict[str, str]] = None) -> ComposeResult
```

**Parameters:**
- `service` (str): Service name
- `command` (List[str]): Command to execute
- `detach` (bool): Detached mode (default: False)
- `tty` (bool): Allocate a pseudo-TTY (default: True)
- `user` (Optional[str]): User to run command as
- `workdir` (Optional[str]): Working directory inside container
- `env` (Optional[Dict[str, str]]): Environment variables

**Returns:**
- `ComposeResult`: Operation result

**Example:**
```python
# Run bash in web container
result = stack.exec("web", ["bash"])

# Run command as specific user
result = stack.exec("db", ["psql", "-U", "postgres"], user="postgres")

# Run with environment variables
result = stack.exec("app", ["python", "script.py"], env={"DEBUG": "1"})
```

##### pull()

Pull service images.

```python
pull(services: Optional[List[str]] = None) -> ComposeResult
```

**Parameters:**
- `services` (Optional[List[str]]): Specific services (default: all)

**Returns:**
- `ComposeResult`: Operation result

##### config()

Validate and view the compose configuration.

```python
config(quiet: bool = False,
       services: bool = False,
       volumes: bool = False) -> ComposeResult
```

**Parameters:**
- `quiet` (bool): Only validate configuration (default: False)
- `services` (bool): Print service names (default: False)
- `volumes` (bool): Print volume names (default: False)

**Returns:**
- `ComposeResult`: Operation result with configuration

### ComposeResult

Result object returned by all Docker Compose operations.

#### Attributes

- `success` (bool): Whether the operation succeeded
- `exit_code` (int): Process exit code
- `stdout` (str): Standard output from the command
- `stderr` (str): Standard error output from the command

**Example:**
```python
result = stack.up(detach=True)
if result.success:
    print("Stack started successfully")
else:
    print(f"Failed with exit code {result.exit_code}")
    print(f"Error: {result.stderr}")
```

## Complete Example

```python
from angreal.integrations.docker import compose

# Create and manage a development stack
def manage_dev_stack():
    # Check Docker Compose availability
    if not DockerCompose.is_available():
        print("Docker Compose is not available")
        return 1
    
    # Initialize stack
    stack = compose("docker-compose.dev.yml", project_name="myapp-dev")
    
    # Pull latest images
    print("Pulling latest images...")
    result = stack.pull()
    if not result.success:
        print(f"Pull failed: {result.stderr}")
        return 1
    
    # Build and start services
    print("Starting services...")
    result = stack.up(detach=True, build=True, remove_orphans=True)
    if not result.success:
        print(f"Failed to start: {result.stderr}")
        return 1
    
    # Check status
    result = stack.ps()
    print("Running services:")
    print(result.stdout)
    
    # View logs
    result = stack.logs(services=["web"], tail="20")
    print("Recent web logs:")
    print(result.stdout)
    
    # Execute database migration
    print("Running migrations...")
    result = stack.exec("web", ["python", "manage.py", "migrate"])
    if not result.success:
        print(f"Migration failed: {result.stderr}")
    
    return 0
```

## Error Handling

All methods return a `ComposeResult` object. Check the `success` attribute to determine if the operation succeeded:

```python
result = stack.up(detach=True)
if not result.success:
    # Handle error
    print(f"Error: {result.stderr}")
    print(f"Exit code: {result.exit_code}")
```

Common error scenarios:
- Docker Compose not installed: Check with `DockerCompose.is_available()`
- Invalid compose file: Constructor will raise `IOError`
- Service not found: Check stderr for "no such service"
- Container not running: Exec commands will fail with appropriate message

## Docker Compose Version Compatibility

The integration automatically detects and uses the appropriate Docker Compose version:
1. Tries `docker compose` (v2) first
2. Falls back to `docker-compose` (v1) if v2 not available
3. All features work with both versions