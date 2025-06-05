---
title: "CLI Reference"
weight: 1
---

# CLI Reference

Complete command-line interface documentation for Angreal.

## Synopsis

```bash
angreal [OPTIONS] <SUBCOMMAND>
```

## Global Options

- `-h, --help` - Print help information
- `-v, --verbose` - Verbose level (may be used multiple times for more verbosity)
- `-V, --version` - Print version information

### Verbose Levels

The verbose flag can be repeated for increased verbosity:

```bash
angreal -v init template/     # Basic verbose output
angreal -vv init template/    # More detailed output
angreal -vvv init template/   # Maximum verbosity
```

## Core Commands

### init

Initialize a new project from an Angreal template.

```bash
angreal init <TEMPLATE> [OPTIONS]
```

**Arguments:**
- `TEMPLATE` - Template source (local path, Git URL, or GitHub shorthand)

**Options:**
- `-f, --force` - Force the rendering of a template, even if paths/files already exist
- `-d, --defaults` - Use default values provided in the angreal.toml
- `--values <FILE>` - Provide values to template, bypassing template toml

**Template Sources & Examples:**

```bash
# Local template
angreal init ./my-template

# Git repository
angreal init https://github.com/user/template.git

# GitHub catalog template
angreal init template

# With options
angreal init template/ --force --defaults
angreal init template/ --values values.toml
```

{{< hint type=note >}}
**Available Templates**: Browse the official Angreal templates at [github.com/angreal](https://github.com/angreal) to find pre-built templates for various project types.
{{< /hint >}}

**Template Resolution:**
Angreal resolves templates in the following order:
1. Local path if it exists
2. Path in `~/.angrealrc/` if it exists
3. GitHub repository at `https://github.com/angreal/template_name`
4. Git repository at the specified URL

{{< hint type=info >}}
For a detailed explanation of how Angreal resolves and processes templates, see [Angreal Init Behavior](/angreal/explanation/angreal_init_behaviour/).
{{< /hint >}}

### tree

Get a structured view of all available commands and tasks in an Angreal project.

```bash
angreal tree [OPTIONS]
```

**Options:**
- `--json` - Output command structure in JSON format for machine processing

**Examples:**

```bash
# Human-readable tree view
angreal tree

# JSON output for tooling integration
angreal tree --json
```

### alias

Create and manage command aliases for white-labeling Angreal.

```bash
angreal alias <SUBCOMMAND>
```

**Subcommands:**
- `create <name>` - Create a new command alias
- `remove <name>` - Remove an existing alias
- `list` - List all registered aliases

**Examples:**

```bash
# Create a custom command name
angreal alias create mycompany-tool

# Now you can use 'mycompany-tool' instead of 'angreal'
mycompany-tool init template/

# List all aliases
angreal alias list

# Remove an alias
angreal alias remove mycompany-tool
```

### completion

Install and manage shell completion for Angreal.

```bash
angreal completion <SUBCOMMAND>
```

**Subcommands:**
- `install [shell]` - Install shell completion (bash, zsh)
- `uninstall [shell]` - Uninstall shell completion
- `status` - Show completion installation status

**Examples:**

```bash
# Install completion for current shell
angreal completion install

# Install for specific shell
angreal completion install bash

# Check completion status
angreal completion status

# Uninstall completion
angreal completion uninstall
```

### help

Print help information for Angreal or a specific subcommand.

```bash
angreal help [SUBCOMMAND]
```

**Examples:**

```bash
# General help
angreal help

# Help for a specific command
angreal help init

# Alternative syntax
angreal init --help
```

## Project-Specific Commands

When you're inside an Angreal project (a directory containing `.angreal/`), the CLI behavior changes significantly.

{{< hint type=warning >}}
**Important**: When inside an Angreal project, you lose access to the `init` command and instead get the project's custom tasks. To use `init`, you must run it from outside any Angreal project directory.
{{< /hint >}}

### Task Discovery

Angreal automatically discovers tasks by:

1. Looking for `.angreal/` directory in current or parent directories
2. Loading Python files matching the pattern `task_*.py`
3. Registering functions decorated with `@angreal.command`
4. **Replacing** the default commands with project-specific ones

### Command Context Loading

Angreal's available commands are determined by the nearest `.angreal/` directory found when walking up the directory tree from your current location.

**Outside any Angreal project:**
```bash
angreal --help

# SUBCOMMANDS:
#     help    Print this message or the help of the given subcommand(s)
#     init    Initialize an Angreal template from source.
```

**Inside an Angreal project:**
```bash
cd my-angreal-project/
angreal --help

# SUBCOMMANDS:
#     help     Print this message or the help of the given subcommand(s)
#     build    Build the project
#     test     Run tests
#     deploy   Deploy to production
#
# Note: 'init' is no longer available!
```

### Running Project Tasks

```bash
# Run a project task
angreal build

# Get help for a project task
angreal build --help

# Run with arguments
angreal deploy --environment staging --dry-run
```

## Common Errors

### Template Not Found

```bash
# Error: Template not found
angreal init nonexistent/template

# Solutions:
# 1. Check the template URL/path
# 2. Ensure you have internet access for remote templates
# 3. Check if the repository is public
# 4. Try with full Git URL
```

### Task Not Found

```bash
# Error: Task 'build' not found
angreal build

# Solutions:
# 1. Check if you're in an Angreal project:
ls .angreal/

# 2. List available tasks:
angreal --help

# 3. Check task file naming:
ls .angreal/task_*.py
```

### Command Not Found

```bash
# Error: angreal command not found

# Linux/macOS:
which angreal
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Windows:
where angreal
# Add to PATH: %APPDATA%\Python\Scripts
```

## See Also

- [Quick Start](/angreal/quick-start) - Getting started with Angreal
- [Python API Reference](/angreal/reference/python-api) - Task definition API
- [Configuration Reference](/angreal/reference/configuration) - Configuration file formats
- [How-to Guides](/angreal/how-to-guides) - Common task examples
