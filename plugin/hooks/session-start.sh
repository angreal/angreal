#!/bin/bash
# SessionStart hook for angreal projects
# Detects .angreal directory and provides project context

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

# Build context message for active angreal project
read -r -d '' CONTEXT << 'EOF'
This is an **angreal project** (detected `.angreal` directory).

## Quick Reference
- Run `angreal` to see available tasks
- Run `angreal <task>` to execute a task
- Run `angreal tree` to see command structure

## Available Skills
When authoring or working with this project:
- `/angreal-authoring` - Creating tasks and commands
- `/angreal-arguments` - Adding arguments to tasks
- `/angreal-integrations` - Using VirtualEnv, Git, Docker integrations
- `/angreal-patterns` - Development best practices
EOF

# Output JSON for Claude
cat << ENDJSON
{
    "hookSpecificOutput": {
        "hookEventName": "SessionStart",
        "additionalContext": "$(echo "$CONTEXT" | sed 's/"/\\"/g' | tr '\n' ' ')"
    }
}
ENDJSON

exit 0
