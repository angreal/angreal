---
title: Connect an AI Assistant
weight: 15
---

# Connect an AI Assistant

This guide shows you how to let an MCP-aware AI assistant (such as Claude Code, Cursor, or Claude Desktop) see and run this project's Angreal tasks. After you finish, the assistant will know which tasks exist, what each one does, and how risky it is — and it will prefer your Angreal tasks over running raw shell commands.

## Prerequisites

- Angreal is installed and on your `PATH` (run `angreal --version` to confirm).
- You are working inside an Angreal project — a directory containing an `.angreal/` folder with `task_*.py` files. The `mcp` command is project-only; it is not available outside an Angreal project.

## How it works

Angreal ships a built-in Model Context Protocol (MCP) server, started with `angreal mcp`. The MCP client (your AI assistant) launches this command as a stdio subprocess. The server then feeds the client your project's **task tree as context**.

Concretely, `angreal mcp`:

- Speaks JSON-RPC 2.0 over stdio — it reads requests on standard input and writes responses on standard output.
- Responds to `initialize` by returning your task tree (each task's description and `risk_level`) as the server's **instructions**, which the client surfaces to the assistant as system-level context.
- Advertises **no callable tools, resources, or prompts** — `tools/list`, `resources/list`, and `prompts/list` all return empty lists.

In other words, your tasks reach the assistant as *context*, not as individually callable MCP tools. The assistant reads the task tree, decides which task fits the job, and then runs it through the shell (for example, `angreal test all`). It does not invoke tasks through an MCP tool call.

Because the server communicates over stdio, you normally do not run it by hand in a terminal — the client starts and stops it for you. You can still run `angreal mcp` manually as a quick sanity check that it launches (it will sit waiting for JSON-RPC input; press `Ctrl-C` to exit).

## Step 1: Configure your client

All of the clients below use the standard `mcpServers` stdio shape: a server named `angreal` whose command is `angreal` with the single argument `mcp`. Pick the configuration for your client.

### Claude Code

Create a project-scoped `.mcp.json` at the repository root:

```json
{
  "mcpServers": {
    "angreal": { "command": "angreal", "args": ["mcp"] }
  }
}
```

Because this file lives at the project root, Claude Code launches `angreal mcp` from inside the project, so the project-only `mcp` command resolves correctly.

### Cursor

Create `.cursor/mcp.json` in the project, using the same `mcpServers` shape:

```json
{
  "mcpServers": {
    "angreal": { "command": "angreal", "args": ["mcp"] }
  }
}
```

### Claude Desktop

Add an `angreal` entry to the `mcpServers` block in your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "angreal": {
      "command": "angreal",
      "args": ["mcp"],
      "cwd": "/abs/path/to/your/project"
    }
  }
}
```

Claude Desktop's config is global rather than project-scoped, so you must make sure the server starts **inside** your Angreal project — otherwise the project-only `mcp` command is unavailable. If your client supports a working-directory setting (shown above as `"cwd"`), point it at the absolute path of your project. If it does not, you will need to launch the client from within the project directory so that `angreal mcp` runs there.

## Step 2: Make your tasks useful to the assistant

The assistant only sees what your task tree exposes. A bare command shows just its name and one-line description. To give the assistant real guidance — when to use a task, when not to, and how dangerous it is — attach an `angreal.ToolDescription(...)` with a `risk_level` to your `@angreal.command`:

```python
import angreal

@angreal.command(
    name="test",
    about="Run the test suite",
    tool=angreal.ToolDescription(
        """Run the full test suite. Use this before committing or opening a PR.
        Do not use it to run a single test file — invoke pytest directly for that.""",
        risk_level="safe",
    ),
)
def test():
    ...
```

Preview exactly what the assistant will receive — the same long-form task tree the MCP server sends as context — with:

```bash
angreal tree --long
```

For the full mechanics of writing tasks and tool descriptions, see [Create a Command](create-a-task.md) and the [Command Decorator Reference](/angreal/reference/python-api/commands/command_decorator).

## Step 3: Verify

1. Restart or reload your AI client so it picks up the new MCP server configuration.
2. Ask the assistant to list the available Angreal tasks, or to describe a specific one (for example, "what Angreal tasks can you run here, and what does the test task do?").

If the assistant can enumerate your tasks and summarize their descriptions and risk levels, the connection is working — the task tree from `angreal mcp` is reaching the assistant as context. From here, the assistant can pick the right task and run it through the shell.
