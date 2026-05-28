# Angreal Plugin for Claude Code

Task automation skills for angreal projects. Focused skills for each job to be done.

## Skills

| Skill | Trigger Phrases | Purpose |
|-------|-----------------|---------|
| `angreal-usage` | "run angreal", "discover tasks", "angreal tree" | Running and discovering tasks |
| `angreal-authoring` | "create a task", "write task file", "@angreal.command" | Creating task files in existing projects |
| `angreal-arguments` | "add arguments", "@angreal.argument", "add flags" | The @argument decorator |
| `angreal-tool-descriptions` | "write ToolDescription", "AI guidance", "risk level" | Writing AI-friendly task descriptions |
| `angreal-init` | "initialize angreal", "add angreal to project" | Adding angreal to existing projects |
| `angreal-templates` | "create a template", "angreal.toml", "Tera templating", "in-place", "official templates" | Creating reusable templates for others (and consuming the official ones) |
| `angreal-patterns` | "test tasks", "best practices", "error handling" | Testing, development, and documentation patterns |
| `angreal-integrations` | "use Git in task", "create venv", "docker-compose", "use Docker class", "use Flox", "render template" | Built-in Git, VirtualEnv, Docker, Flox, and Tera templating |
| `angreal-mcp` | "expose tasks to Claude / Cursor", "set up MCP for angreal", "AI agent integration" | The built-in `angreal mcp` server and `.mcp.json` configuration |

## Auto-Activation

Skills automatically activate for projects containing a `.angreal/` directory.

## Installation

### Via Claude Code Plugin System

```bash
# Add the angreal marketplace, then install the plugin
/plugin marketplace add angreal/angreal
/plugin install angreal@angreal-angreal
```

### Local Development

```bash
claude --plugin-dir /path/to/angreal/plugin
```

## Skill Details

### angreal-usage
For AI agents running tasks:
- Discovering available tasks with `angreal tree`
- Running tasks with proper arguments
- Understanding task output and exit codes
- Common multi-task workflows

### angreal-authoring
For developers creating tasks:
- Task file structure (`task_*.py` in `.angreal/`)
- The `@command` decorator
- Command groups with `command_group()`
- Naming conventions and organization

### angreal-arguments
For adding CLI arguments:
- The `@argument` decorator
- Flags, required args, defaults
- Type conversion with `python_type`
- Multiple values

### angreal-tool-descriptions
For AI-friendly tasks:
- Writing `ToolDescription` prose
- Risk levels (safe, read_only, destructive)
- Structuring guidance for AI agents
- Best practices for discoverability

### angreal-init
For adding angreal to existing projects:
- Creating `.angreal/` directory structure
- Starter task file templates
- Migrating from Makefile/scripts
- Project setup best practices

### angreal-templates
For creating reusable templates others can consume:
- Template structure (`angreal.toml`, templated files)
- Tera templating engine (variables, conditionals, loops)
- Post-initialization scripts (`.angreal/init.py`)
- In-place rendering (`angreal init <template> --in-place`)
- The official angreal templates (`python`, `rust`, `data-science`, ...)
- Publishing and sharing templates

### angreal-patterns
For implementation patterns:
- Unit testing task functions
- Mocking `angreal.get_root()`
- Verbose/quiet/dry-run modes
- Subprocess handling
- Error handling patterns

### angreal-integrations
For built-in tool integrations:
- `angreal.render_template()` / `angreal.render_directory()` - Tera templating for file scaffolding
- `angreal.integrations.git.Git` - Repository operations
- `angreal.integrations.venv.VirtualEnv` - Virtual environment management
- `angreal.integrations.docker.Docker` - Low-level Docker client (containers, images, networks, volumes)
- `angreal.integrations.docker.DockerCompose` - Docker Compose operations
- `angreal.integrations.flox.Flox` - Cross-language environments and services
- `@venv_required` / `@flox_required` decorators for automatic environment handling

### angreal-mcp
For making project tasks discoverable by AI assistants:
- The built-in `angreal mcp` stdio server (Model Context Protocol)
- `.mcp.json` configuration for Claude Code / Cursor / other MCP clients
- How `ToolDescription` and `risk_level` flow into agent context
- When to recommend MCP setup vs. relying on this plugin's SessionStart hook

## Quick Reference

### Basic Task

```python
import angreal

@angreal.command(name="build", about="Build the project")
@angreal.argument(name="release", long="release", is_flag=True, takes_value=False)
def build(release=False):
    project_root = angreal.get_root().parent  # .angreal/ -> project root
    # Implementation
    return 0
```

### Task Discovery

```bash
angreal tree           # List all commands
angreal tree --long    # Include ToolDescription prose
```

### Start a Project from an Official Template

```bash
angreal init python          # github.com/angreal/python
angreal init rust            # github.com/angreal/rust
angreal init python --in-place   # render into the current directory
```

A bare name resolves to `https://github.com/angreal/<name>`. Browse the full,
current catalog at [github.com/angreal](https://github.com/angreal).

## License

MIT
