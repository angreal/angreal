---
name: angreal-mcp
description: This skill should be used when the user asks to "start the angreal mcp server", "expose angreal tasks to Claude / Cursor / an AI agent", "configure .mcp.json for angreal", "set up MCP for my angreal project", "connect angreal to an AI assistant", "what does angreal mcp do", "AI agent integration with angreal", or needs guidance on the built-in Model Context Protocol server (`angreal mcp`), what it exposes, how MCP clients should be configured to connect to it, and how `ToolDescription` and `risk_level` flow into agent context.
version: 2.8.7
---

# Angreal MCP Server

`angreal mcp` is a built-in [Model Context Protocol](https://modelcontextprotocol.io/) stdio server that injects a project's task tree into any connected MCP client at handshake time. It is the portable, client-agnostic way to make a project's automation discoverable to AI assistants.

## What It Actually Is (and Isn't)

**It is**: structured context/prompt injection delivered over the MCP protocol. The server implements MCP 2024-11-05 and responds to five methods (`initialize`, `tools/list`, `resources/list`, `prompts/list`, `ping`). The entire useful payload is the `instructions` string returned in the `initialize` response — a markdown document containing:

1. A preamble: "Angreal IS the operational task orchestration system for this project."
2. The decision rule: "Before running ANY build/test/lint/docs/deploy command, check the task list; use the angreal task if one exists."
3. The full task tree (equivalent to `angreal tree --long`), including each command's name, argument signature, `about` line, and any `ToolDescription` prose + `risk_level`.

That `instructions` field is a documented MCP feature meant exactly for this purpose — context an agent carries for the duration of the session. The client receives it once at connect, treats it as system-level guidance, and proceeds.

**It is NOT** a callable-tool MCP server. `tools/list`, `resources/list`, and `prompts/list` all return `[]`. The agent never calls back into the server after the handshake — it runs tasks the normal way (`angreal <command>` via its own shell tool).

## Why No Callable Tools — This Is Deliberate

Exposing each angreal task as an MCP tool would seem more "MCP-native," but it doesn't fit the workload:

- **Angreal tasks tend to be long-running** (test suites, builds, deploys, doc generation). MCP tool-call semantics assume bounded, request/response interactions — long-running calls break client timeouts, lose streaming output, and create awkward reconnect/recovery edges that the agent has to handle.
- **It would duplicate the CLI surface.** Every task would exist in two execution channels (shell and MCP tool call) with different error paths, different env handling, and different output capture. Agents would have to learn two ways to invoke the same thing.
- **The shell is already the right channel.** Agents have a battle-tested bash/shell tool with streaming output, exit codes, env passthrough, and ergonomic interruption. There is no reason to re-implement any of that inside MCP.

So Angreal splits the concerns cleanly: **MCP is the discovery channel, the shell is the execution channel.** The handshake teaches the agent what tasks exist and when to use them; the agent then invokes them through the channel that's actually good at long-running processes.

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
| `tools/list` | `[]` |
| `resources/list` | `[]` |
| `prompts/list` | `[]` |
| `ping` | `{}` (health check) |
| anything else | JSON-RPC `-32601 Method not found` |

The `initialize` response is the only one that carries real payload. The empty `*/list` responses exist so MCP-conformant clients don't error out when they probe for tools/resources/prompts at startup.

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
