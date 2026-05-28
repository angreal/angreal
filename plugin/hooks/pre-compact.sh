#!/bin/bash
# PreCompact hook for angreal projects
# Re-injects essential context before compaction to ensure authoritative guidance persists

# Exit silently if not in an angreal project
if [ ! -d "$CLAUDE_PROJECT_DIR/.angreal" ]; then
    exit 0
fi

# Check if angreal is installed
if ! command -v angreal &> /dev/null; then
    exit 0
fi

# Get the command tree (run from project directory)
cd "$CLAUDE_PROJECT_DIR" || exit 0
TREE_OUTPUT=$(angreal tree --long 2>/dev/null)

# Build comprehensive context to preserve through compaction
read -r -d '' CONTEXT << EOF
## CONTEXT RESTORED: Angreal Project

### CRITICAL: Angreal IS the Operational Task Orchestration System

**Angreal tasks are the authoritative way to run operations** in this project. They encode project-specific knowledge — correct flags, paths, environment setup, and sequencing that manual commands will get wrong.

**When an angreal task exists for an operation, running the underlying command directly via Bash is WRONG.** Before running ANY build, test, lint, docs, or deploy command:
1. Check the task list below
2. If an angreal task covers the operation, **USE IT**
3. Only use manual commands when no angreal task covers the need

### Available Tasks (with ToolDescriptions)

\`\`\`
${TREE_OUTPUT}
\`\`\`

### Operational Essentials

**Top-level CLI**: \`tree\` / \`tree --long\` (discover), \`mcp\` (start MCP stdio server for AI clients), \`init <template>\` (scaffold; \`--in-place\` renders into cwd), \`alias create\`, \`completion\` (bash/zsh), \`-v\`/\`-vv\`/\`-vvv\` (verbosity), \`<command> --help\` (per-task help). Set \`ANGREAL_DEBUG=true\` to force debug logging; \`ANGREAL_NO_AUTO_COMPLETION=1\` to suppress completion install in CI.

**Exit codes**: \`0\` success (also \`None\`/\`True\`); \`1\` general failure (also \`False\` or Angreal-level errors); \`56\` unhandled Python exception; \`N\` custom (task returns int \`N\` or raises \`SystemExit(N)\`). Propagate subprocess exit codes via \`return result.returncode\`.

### Skills for Angreal Development
- \`/angreal-usage\` - Running and discovering tasks
- \`/angreal-authoring\` - Creating tasks
- \`/angreal-arguments\` - Adding arguments
- \`/angreal-tool-descriptions\` - Writing AI-friendly ToolDescriptions
- \`/angreal-integrations\` - VirtualEnv, Git, Docker, Flox
- \`/angreal-init\` - Adding angreal to an existing project
- \`/angreal-templates\` - Templates + official templates (\`angreal init python\`, \`--in-place\`)
- \`/angreal-mcp\` - Expose tasks to AI assistants via \`angreal mcp\` server
- \`/angreal-patterns\` - Best practices
EOF

# Escape the context for JSON
ESCAPED_CONTEXT=$(printf '%s' "$CONTEXT" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))')

# Output JSON for Claude - PreCompact uses systemContext for higher precedence
cat << ENDJSON
{
    "systemContext": ${ESCAPED_CONTEXT}
}
ENDJSON

exit 0
