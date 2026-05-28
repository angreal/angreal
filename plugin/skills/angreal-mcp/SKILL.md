---
name: angreal-mcp
description: This skill should be used when the user asks to "start the angreal mcp server", "expose angreal tasks to Claude / Cursor / an AI agent", "configure .mcp.json for angreal", "set up MCP for my angreal project", "connect angreal to an AI assistant", "what does angreal mcp do", "AI agent integration with angreal", or needs guidance on the built-in Model Context Protocol server (`angreal mcp`), what it exposes, how MCP clients should be configured to connect to it, and how `ToolDescription` and `risk_level` flow into agent context.
version: 2.8.7
---

# Angreal MCP Server

Angreal ships with a built-in [Model Context Protocol](https://modelcontextprotocol.io/) server (`angreal mcp`) that injects a project's task tree into any connected MCP client. This is the canonical way to make a project's automation available to AI assistants without writing plugin code.

## What It Is

`angreal mcp` is a stdio-based MCP server implementing protocol version `2024-11-05`. It exposes **no MCP tools and no resources** — its only job is **context injection**.

When an MCP client connects, the server's `initialize` response includes an `instructions` markdown document containing:

1. A preamble: "Angreal tasks are authoritative for this project's operations."
2. A decision rule: "If an angreal task covers the operation, use it instead of running the underlying tool directly."
3. The full task tree (same content as `angreal tree --long`), including every command's name, argument signature, `about` line, and any `ToolDescription` prose + `risk_level`.

That `instructions` field is loaded into the client's system context, so the AI agent knows about every task the project exposes from the moment it connects — no trial-and-error discovery, no manual onboarding.

## Running the Server

```bash
angreal mcp
```

Must be run from inside an angreal project (a directory containing `.angreal/`). The server reads JSON-RPC from stdin and writes responses to stdout. It is not a long-lived daemon — MCP clients launch one process per connection.

## Configuring an MCP Client

### Claude Code (`.mcp.json`)

In the project root or `~/.claude/.mcp.json`:

```json
{
  "mcpServers": {
    "angreal": {
      "command": "angreal",
      "args": ["mcp"],
      "cwd": "/absolute/path/to/your/project"
    }
  }
}
```

The `cwd` field is critical — it must point to the directory containing `.angreal/`. Without it the server has no project to introspect and will exit. Use an absolute path; MCP clients don't always resolve relative paths predictably.

### Other MCP Clients

Any MCP client that supports stdio servers will work. The pattern is the same: invoke `angreal mcp` with `cwd` set to the angreal project root. Consult the specific client's documentation for the exact config file location and schema.

## Supported MCP Methods

| Method | Behavior |
|--------|----------|
| `initialize` | Returns server info, capabilities, and the task instructions document |
| `tools/list` | Returns `[]` (empty) — Angreal exposes context, not callable MCP tools |
| `resources/list` | Returns `[]` |
| `prompts/list` | Returns `[]` |
| `ping` | Returns `{}` (health check) |

Because no MCP tools are exposed, the agent does not "call" angreal through MCP — it learns *that* angreal exists and *what* tasks are available, then invokes `angreal <command>` through its own shell/bash tool. This is intentional: it keeps the agent's existing shell tooling as the single execution path and avoids duplicating every task as an MCP tool.

## How `ToolDescription` Flows In

Tasks decorated with `tool=angreal.ToolDescription(...)` get their full prose and `risk_level` included in the MCP `instructions` document. This is the primary reason to write ToolDescriptions:

- **No `tool=`** → MCP sees just `name [args] - about line`
- **With `tool=`** → MCP sees the full When-to-use / When-NOT / Examples / Recovery prose plus a `risk_level` tag

See the `angreal-tool-descriptions` skill for how to write effective descriptions. The MCP server is what surfaces them; without `angreal mcp` (or `angreal tree --long`), nobody sees them.

## When to Recommend the MCP Server

Recommend setting it up when the user:

- Wants their AI assistant to "just know" the project's commands
- Is repeatedly correcting an agent that runs `pytest` directly instead of `angreal test python`
- Is onboarding a new project to Claude Code / Cursor / similar and wants automation discoverability without writing a plugin
- Asks "how do I make my agent use angreal tasks?"

If the user is already inside a Claude Code session in an angreal project, the SessionStart hook in this plugin already injects similar context — `angreal mcp` is for *other* MCP clients (Cursor, Continue, custom agents) or for Claude Code instances where the angreal plugin isn't installed.

## Troubleshooting

| Symptom | Likely cause |
|---------|--------------|
| Server exits immediately on connect | `cwd` doesn't contain `.angreal/` |
| Agent doesn't know about new tasks | MCP client caches `initialize` — restart the client after editing task files |
| `instructions` is missing tool descriptions | Tasks lack `tool=angreal.ToolDescription(...)` — only `name`/`about` will be exposed |
| `angreal: command not found` from MCP client | Use the absolute path to the `angreal` binary in the `command` field, or ensure the client's `PATH` includes it |
