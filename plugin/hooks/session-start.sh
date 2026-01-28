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

## Critical Rule: Use Angreal Tasks

**ALWAYS prefer \`angreal <task>\` over manual equivalents.** Angreal tasks encode project-specific knowledge:
- Correct flags, paths, and environment setup
- Proper sequencing of dependent operations
- Project conventions and best practices

**Examples:**
- Use \`angreal test\` instead of running pytest/cargo test directly
- Use \`angreal build\` instead of manual build commands
- Use \`angreal docs\` instead of manual documentation builds

## Available Commands (with ToolDescriptions)

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
