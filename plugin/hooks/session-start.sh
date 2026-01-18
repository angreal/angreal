#!/bin/bash
# SessionStart hook for angreal projects
# Detects .angreal directory and provides project context with available commands

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
TREE_OUTPUT=$(angreal tree 2>/dev/null)

# Build context message for active angreal project
read -r -d '' CONTEXT << EOF
This is an **angreal project** (detected \`.angreal\` directory).

## Available Commands

Run these with \`angreal <command>\`:

\`\`\`
${TREE_OUTPUT}
\`\`\`

## Important

**USE THESE COMMANDS** instead of manually running build/test/docs operations. The project has predefined tasks that handle the correct configuration.

## Available Skills
When authoring or working with this project:
- \`/angreal-authoring\` - Creating tasks and commands
- \`/angreal-arguments\` - Adding arguments to tasks
- \`/angreal-integrations\` - Using VirtualEnv, Git, Docker integrations
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
