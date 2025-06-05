---
title: Aliasing System Architecture
weight: 40
---

# Aliasing System Architecture

This document explains how Angreal's aliasing system works internally to enable white-labeling and custom command names.

## Overview

The aliasing system creates executable wrapper scripts that redirect to angreal while maintaining the illusion of a separate command. When you run `angreal alias create mycompany-tool`, it generates a script in `~/.local/bin/mycompany-tool` that imports and calls angreal.

## Core Components

### Wrapper Script Generation

Platform-specific executable scripts are created in `~/.local/bin/`:

**Unix/Linux/macOS**:
```python
#!/usr/bin/env python3
# ANGREAL_ALIAS: alias-name
import sys
try:
    import angreal
    angreal.main()
except ImportError:
    print(f"Error: angreal not installed. Remove alias: rm {script_path}", file=sys.stderr)
    sys.exit(1)
```

**Windows**:
```batch
@echo off
REM ANGREAL_ALIAS: alias-name
python -m angreal %*
```

### Registry Management

A JSON registry at `~/.angrealrc/aliases.json` tracks registered aliases:

```json
["mycompany-tool", "project-cli", "devops-helper"]
```

## Implementation Details

### Cross-Platform Handling

The Rust code generates platform-appropriate scripts:

```rust
#[cfg(unix)]
{
    // Create Python script with shebang
    // Set executable permissions (755)
}

#[cfg(windows)]
{
    // Create .bat file
    // No permission changes needed
}
```

### Argument Preservation

Wrapper scripts preserve all command-line arguments:
- Unix: `sys.argv` contains alias name and all arguments
- Windows: `%*` passes all arguments unchanged

From angreal's perspective, `sys.argv[0]` contains the alias name instead of "angreal", so help messages show the custom command name.

## Design Decisions

### Why Wrapper Scripts Instead of Symlinks?

Symlinks were rejected because:
1. Shell completion wouldn't work properly
2. Error messages would show "angreal" instead of custom name
3. Cross-platform complexity (Windows symlinks)

### Why ~/.local/bin?

- No sudo required (user-level installation)
- Standard XDG convention
- Per-user isolation
- Easy cleanup

### Why Python Scripts Instead of Shell Scripts?

- Better error handling for missing angreal
- Cross-platform compatibility
- Direct access to angreal module

## Security

- **Conflict prevention**: Checks for existing commands before creation
- **User-only**: Uses `~/.local/bin` and `~/.angrealrc` (no system modifications)
- **Minimal permissions**: 755 on Unix
- **Atomic registry updates**: Prevents corruption during concurrent access

## Error Handling

**Missing angreal**: Wrapper script provides clear error message and removal instructions

**Registry corruption**: System recovers by resetting to empty registry

**Path conflicts**: Registration fails with helpful error message

The aliasing system provides transparent command redirection while maintaining full angreal functionality and shell integration.
