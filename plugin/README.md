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
| `angreal-templates` | "create a template", "angreal.toml", "Tera templating" | Creating reusable templates for others |
| `angreal-patterns` | "test tasks", "best practices", "error handling" | Testing, development, and documentation patterns |
| `angreal-integrations` | "use Git in task", "create venv", "docker-compose", "render template" | Built-in Git, VirtualEnv, Docker, and Tera templating |

## Auto-Activation

Skills automatically activate for projects containing a `.angreal/` directory.

## Installation

### Via Claude Code Plugin System

```bash
claude plugins add angreal
```

### Local Development

```bash
claude --plugin-dir /path/to/angreal/skill
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
- `angreal.integrations.docker.DockerCompose` - Docker Compose operations
- `@venv_required` decorator for automatic venv handling

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

## Version

2.8.0

## License

MIT
