# Angreal MCP Server

An MCP (Model Context Protocol) server that provides AI assistants with discovery capabilities for [angreal](https://github.com/angreal/angreal) projects.

## Overview

This server exposes angreal's command tree structure to MCP-compatible clients, enabling AI assistants to:
- Discover available commands and tasks in your angreal project
- Understand project structure and capabilities
- Execute commands with context-aware parameter selection
- Check angreal installation and project status

## Prerequisites

- [angreal](https://github.com/angreal/angreal) must be installed and available in your PATH
- Rust toolchain (for building from source)

## Installation

```bash
# Install from crates.io
cargo install angreal_mcp

# Or install from git (latest development version)
cargo install --git https://github.com/angreal/angreal
```

## Usage

### With Claude Desktop

Add to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "angreal": {
      "command": "angreal-mcp",
      "args": []
    }
  }
}
```

### With Cline (VS Code)

Add to your Cline configuration:

```json
{
  "mcp": {
    "servers": [
      {
        "name": "angreal",
        "command": ["angreal-mcp"]
      }
    ]
  }
}
```

### Command Line Testing

You can test the MCP server directly via command line:

```bash
# List available tools
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | angreal-mcp

# Get angreal command tree
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "angreal_tree", "arguments": {"format": "json"}}}' | angreal-mcp
```

## Available Tools

### `angreal_check`
Check if the current directory is an angreal project and get project status including available commands.

**When to use:**
- Starting work in a new directory to understand if angreal is available
- Troubleshooting angreal-related issues
- Getting an overview of project capabilities before using other tools

**Returns:**
Comprehensive status including installation status, project detection, and available commands

### `angreal_tree`
Get a structured view of all available angreal commands and tasks in the project.

**When to use:**
- Discovering what commands are available in an angreal project
- Understanding project structure before executing tasks
- Planning automation workflows

**Parameters:**
- `format` (optional): Output format - `"json"` for structured data (default) or `"human"` for readable tree display

**Returns:**
Structured list of available commands, tasks, and their descriptions

### `angreal_run`
Execute an angreal command or task with optional arguments.

**When to use:**
- Running discovered angreal commands and tasks
- Automating project workflows
- Executing build, test, or deployment tasks

**Parameters:**
- `command` (required): The angreal command/task to execute
- `args` (optional): Additional arguments, options, and flags as an array

**Returns:**
Command output including both stdout and stderr for complete results

## Agent Usage Guide

When working in angreal projects, use these tools for intelligent command discovery and execution:

### Tool Usage Workflow
1. **Start with discovery**: Use `angreal_check` to verify project status and capabilities
2. **Explore commands**: Use `angreal_tree` to see available commands with rich metadata
3. **Execute intelligently**: Use `angreal_run` with context-aware parameter selection

### Best Practices
- Always discover available commands with `angreal_tree` before executing
- Verify you're in an angreal project directory with `angreal_check`
- Execute commands individually rather than chaining complex operations
- Each argument should be a separate array element for proper shell safety

### Common Patterns
```bash
# Check project status
angreal_check

# Discover available commands
angreal_tree {"format": "json"}

# Run a test command
angreal_run {"command": "test"}

# Run with arguments
angreal_run {"command": "build", "args": ["--release", "--verbose"]}
```

### Troubleshooting
- If MCP server becomes unavailable, restart your MCP client to reinitialize
- Check that angreal binary is installed and accessible in PATH
- Verify you're in an angreal project directory (contains .angreal/ folder)
- Use `angreal_check` first to diagnose issues

## Development

### Project Structure

```
angreal-mcp/
├── src/
│   ├── main.rs      # Main server entry point
│   ├── server.rs    # MCP server implementation
│   └── tools/       # Tool implementations
│       ├── mod.rs
│       ├── angreal_command_tool.rs
│       └── dynamic_tools.rs
├── Cargo.toml
└── README.md
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly
cargo run
```

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## License

This project is licensed under MIT.