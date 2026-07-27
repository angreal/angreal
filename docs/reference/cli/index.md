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

The verbose flag can be repeated to raise the log level. Each count maps to a
specific level (logs are written to stderr):

| Flag | Log level |
|------|-----------|
| _(none)_ | `warn` (default) |
| `-v` | `info` |
| `-vv` | `debug` |
| `-vvv` (or more) | `trace` |

```bash
angreal -v init template/     # info-level output
angreal -vv init template/    # debug-level output
angreal -vvv init template/   # trace-level output (maximum verbosity)
```

Setting the `ANGREAL_DEBUG=true` environment variable forces debug-level logging
and takes precedence over the `-v` flags: when `ANGREAL_DEBUG` is set, the `-v`
count is ignored. See the
[`ANGREAL_DEBUG`](../configuration/index.md#angreal_debug) entry in the
configuration reference.

## Core Commands

### init

Initialize a new project from an Angreal template.

```bash
angreal init <TEMPLATE> [OPTIONS]
```

**Arguments:**
- `TEMPLATE` - Template source (local path, Git URL, GitHub shorthand, or OCI reference `oci://...`)

**Options:**
- `-f, --force` - Force the rendering of a template, even if paths/files already exist
- `-d, --defaults` - Use default values provided in the angreal.toml
- `-i, --in-place` - Render the template's contents into the current directory, stripping the template's top-level directory
- `--values <FILE>` - Provide values to template, bypassing template toml
- `--insecure` - Allow pulling an `oci://` template from a plain-HTTP registry (self-hosted registry without TLS)

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

# Render into the current directory instead of creating a project root
angreal init template/ --in-place
```

!!! note "In-place rendering"
    By default a template's single top-level directory (e.g. `{{ project_name }}/`)
    becomes a new project root inside the current directory. With `--in-place`,
    that top-level directory is stripped and its contents are rendered directly
    into the current directory — useful for scaffolding into a folder you've
    already created (for example an existing git repository). The template must
    have exactly one top-level templated directory, and `--force` is required to
    overwrite files that already exist. See
    [Render a Template In Place](../../how-to-guides/render-template-in-place.md).

{{< hint type=note >}}
**Available Templates**: Browse the official Angreal templates at [github.com/angreal](https://github.com/angreal) to find pre-built templates for various project types.
{{< /hint >}}

**Template Resolution:**

The template argument is first classified by scheme. A full Git URL (`https`,
`ssh`, `git`, `git+ssh`) or an existing local directory is used directly and
bypasses the bare-name expansion below:

- **Git URL**: cloned into the cache directory on first use, or fast-forward
  pulled there if it already exists.
- **Existing local directory**: used in place as the template (it must contain
  an `angreal.toml`).

For a **bare name** such as `python` (anything that is not a recognized URL and
is not an existing directory), Angreal resolves it in the following order:

1. `~/.angrealrc/<name>` — if this directory exists: when it is a Git checkout
   (contains `.git`), fast-forward pull it and use it; otherwise use the plain
   folder as a local template.
2. `./<name>` — a literal local path. If this directory exists and contains an
   `angreal.toml`, use it; if the directory exists without an `angreal.toml`,
   fail with an error.
3. `~/.angrealrc/angreal/<name>` — if this directory exists and is a Git
   checkout, fast-forward pull it and use it; if it exists but is not a Git
   checkout, fail with an error.
4. Otherwise, clone `https://github.com/angreal/<name>.git` into the cache
   directory; if that repository does not exist, fail with "template not found".

!!! note "Cache directory"
    `~/.angrealrc/` is Angreal's template cache directory, where cloned or
    downloaded templates are stored. It is created and managed automatically.
    See [Global Cache Directory](../configuration/index.md#global-cache-directory).

{{< hint type=info >}}
For a detailed explanation (including a decision tree) of how Angreal resolves and processes templates, see [Angreal Init Behavior](/angreal/explanation/angreal_init_behaviour/).
{{< /hint >}}

### tree

Get a structured view of all available commands and tasks in an Angreal project.

```bash
angreal tree [OPTIONS]
```

**Options:**
- `-l, --long` - Include full tool descriptions for AI agent guidance

**Examples:**

```bash
# Short format: commands with arguments and descriptions
angreal tree

# Long format: includes full ToolDescription prose
angreal tree --long
```

**Output Formats:**

The short format (default) shows commands with their argument signatures and short descriptions:
```
test:
  all                                      - Run complete test suite
  rust [--unit-only] [--integration-only]  - Run Rust tests
```

The long format (`--long`) adds the full `ToolDescription` prose for each command, including:
- When to use / when NOT to use guidance
- Example invocations
- Risk level annotations (safe, read_only, destructive)

This enables AI agents to understand available commands and make informed decisions about when and how to use them.

### mcp

Start a Model Context Protocol (MCP) server that exposes the project's task tree as system instructions to MCP-aware AI clients. Project-only command (available only inside an Angreal project containing `.angreal/`).

```bash
angreal mcp
```

The server speaks JSON-RPC 2.0 over stdio, reading requests from standard input and writing responses to standard output. It takes no flags or arguments.

To connect an AI assistant to this server, see the how-to guide [Connect an AI Assistant](../../how-to-guides/connect-an-ai-assistant.md).

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

## Exit Codes

Angreal returns the following process exit codes:

| Code | Meaning |
|------|---------|
| `0` | Success. The task returned a truthy value or `None`, or called `sys.exit(0)`. |
| `1` | General failure. The task returned `False` or another falsy value, a command was missing, or Angreal itself encountered an error. |
| `56` | An unhandled Python exception was raised while running a task. |
| `N` | Custom code. A task that calls `sys.exit(N)` or returns integer `N` propagates that value as the exit code. |

## See Also

- [Quick Start](/angreal/quick-start) - Getting started with Angreal
- [Python API Reference](/angreal/reference/python-api) - Task definition API
- [Configuration Reference](/angreal/reference/configuration) - Configuration file formats
- [How-to Guides](/angreal/how-to-guides) - Common task examples
