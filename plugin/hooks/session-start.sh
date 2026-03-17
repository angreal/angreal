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

## Available Tasks (with ToolDescriptions)

\`\`\`
${TREE_OUTPUT}
\`\`\`

## Available Skills
When authoring or working with angreal tasks:
- \`/angreal-authoring\` - Creating tasks and commands
- \`/angreal-arguments\` - Adding arguments to tasks
- \`/angreal-integrations\` - Using VirtualEnv, Git, Docker, Flox integrations
- \`/angreal-patterns\` - Development best practices
- \`/angreal-usage\` - Running and discovering tasks
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
