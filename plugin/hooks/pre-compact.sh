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
## Angreal Project Context (PRESERVE THIS)

This is an angreal project with predefined automation tasks.

### Critical Rule: ALWAYS Use Angreal Tasks

**NEVER run manual equivalents when an angreal task exists.** Use \`angreal <task>\` because tasks encode:
- Project-specific configuration and flags
- Correct environment setup and dependencies
- Proper sequencing of operations
- Tested, reliable workflows

**Decision Framework:**
1. Before running ANY build/test/docs/deploy command, check available tasks below
2. If an angreal task exists for the operation, USE IT
3. Only use manual commands when no angreal task covers the need
4. Pay attention to risk levels: safe, read_only, or destructive

### Available Commands (with ToolDescriptions)

\`\`\`
${TREE_OUTPUT}
\`\`\`

### Quick Reference
- \`angreal tree\` - List all commands (short form)
- \`angreal <command> --help\` - Help for specific command

### Skills for Angreal Development
- \`/angreal-authoring\` - Creating tasks
- \`/angreal-arguments\` - Adding arguments
- \`/angreal-integrations\` - VirtualEnv, Git, Docker, Flox
- \`/angreal-patterns\` - Best practices
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
