#!/bin/bash
# PreCompact hook for angreal projects
# Re-injects essential context before compaction to ensure it persists

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
TREE_OUTPUT=$(angreal tree 2>/dev/null)

# Build concise context to preserve through compaction
read -r -d '' CONTEXT << EOF
## Angreal Project Context (preserve this)

This is an angreal project. **Use these predefined commands** instead of manual operations:

\`\`\`
${TREE_OUTPUT}
\`\`\`

Run with: \`angreal <command>\`
EOF

# Escape the context for JSON
ESCAPED_CONTEXT=$(printf '%s' "$CONTEXT" | python3 -c 'import json,sys; print(json.dumps(sys.stdin.read()))')

# Output JSON for Claude
cat << ENDJSON
{
    "hookSpecificOutput": {
        "hookEventName": "PreCompact",
        "additionalContext": ${ESCAPED_CONTEXT}
    }
}
ENDJSON

exit 0
