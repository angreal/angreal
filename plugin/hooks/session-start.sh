#!/bin/bash
# SessionStart hook for angreal projects
# Detects .angreal directory and provides authoritative guidance on using angreal

# Exit silently if not in an angreal project
if [ ! -d "$CLAUDE_PROJECT_DIR/.angreal" ]; then
    exit 0
fi

# Check if angreal is installed
if ! command -v angreal &> /dev/null; then
    cat << 'ENDJSON'
{
    "hookSpecificOutput": {
        "hookEventName": "SessionStart",
        "additionalContext": "WARNING: This is an angreal project (`.angreal` directory found) but the `angreal` command is not installed or not in PATH. Install it with: `pip install angreal` or `uv pip install angreal`"
    }
}
ENDJSON
    exit 0
fi

# Get the command tree (run from project directory)
cd "$CLAUDE_PROJECT_DIR" || exit 0
TREE_OUTPUT=$(angreal tree --long 2>/dev/null)

# Build authoritative context message
read -r -d '' CONTEXT << EOF
This is an **angreal project** (detected \`.angreal\` directory).

## CRITICAL: Angreal IS the Operational Task Orchestration System for This Project

**Angreal tasks are the authoritative way to run operations** in this project — build, test, lint, deploy, docs, and any other automated workflow. They encode project-specific knowledge: correct flags, paths, environment setup, dependency sequencing, and conventions that manual commands will get wrong.

**When an angreal task exists for an operation, running the underlying command directly via Bash is WRONG.** Before running ANY build, test, lint, docs, or deploy command:
1. Check the task list below
2. If an angreal task covers the operation, **USE IT** — do not run the underlying tool directly
3. Only use manual commands when no angreal task covers the need

**How to discover tasks:**
- \`angreal tree\` — list all available tasks (short form)
- \`angreal tree --long\` — list with full descriptions
- \`angreal <command> --help\` — help for a specific task

## Operational Essentials

### Top-level CLI surface
- \`angreal tree\` / \`angreal tree --long\` — discover tasks
- \`angreal mcp\` — start MCP stdio server so AI clients can consume this project's task tree
- \`angreal init <template>\` — scaffold a new project from a template (bare name → \`github.com/angreal/<name>\`); add \`--in-place\` / \`-i\` to render into the current directory instead of a new subdir
- \`angreal alias create <name>\` — create a shell alias for an angreal command
- \`angreal completion\` — install shell completion (auto-detects bash/zsh from \`\$SHELL\`)
- \`-v\` / \`-vv\` / \`-vvv\` — increase verbosity. Set \`ANGREAL_DEBUG=true\` to force debug logging regardless of \`-v\` flags. Set \`ANGREAL_NO_AUTO_COMPLETION=1\` in CI to suppress completion auto-install.

### Exit codes
| Code | Meaning |
|------|---------|
| \`0\` | Success — also returned for \`None\` or \`True\` from the task function |
| \`1\` | General failure — also returned for \`False\`, or when Angreal itself errors (missing task, no \`.angreal\`, template not found) |
| \`56\` | Unhandled Python exception in the task (Angreal-specific — distinguishes task crashes from other failures) |
| \`N\`  | Custom — task function returns int \`N\` (non-zero) or raises \`SystemExit(N)\` |

Tasks should propagate subprocess exit codes (\`return result.returncode\`) rather than swallowing them.

## Available Tasks (with ToolDescriptions)

\`\`\`
${TREE_OUTPUT}
\`\`\`

## Available Skills
When authoring or working with angreal tasks:
- \`/angreal-usage\` - Running and discovering tasks
- \`/angreal-authoring\` - Creating tasks and commands
- \`/angreal-arguments\` - Adding arguments to tasks
- \`/angreal-tool-descriptions\` - Writing AI-friendly ToolDescriptions
- \`/angreal-integrations\` - Using VirtualEnv, Git, Docker, Flox integrations
- \`/angreal-init\` - Adding angreal to an existing project
- \`/angreal-templates\` - Creating templates and using the official ones (\`angreal init python\`, \`--in-place\`)
- \`/angreal-mcp\` - Exposing tasks to AI assistants via the built-in \`angreal mcp\` server
- \`/angreal-patterns\` - Development best practices
EOF

# Escape the context for JSON (handle newlines and quotes)
ESCAPED_CONTEXT=$(printf '%s' "$CONTEXT" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))')

# Output JSON for Claude
cat << ENDJSON
{
    "hookSpecificOutput": {
        "hookEventName": "SessionStart",
        "additionalContext": ${ESCAPED_CONTEXT}
    }
}
ENDJSON

exit 0
