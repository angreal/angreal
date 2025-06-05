---
title: Cleanup Entrypoints
---

##### cleanup_entrypoints() -> None:
> Remove all registered command aliases and clean up completely

Removes all command aliases created with register_entrypoint and cleans up the registry.

**Raises:**
- `OSError`: If unable to remove script files or registry

```python
import angreal

# Create multiple aliases
angreal.register_entrypoint("tool1")
angreal.register_entrypoint("tool2")
angreal.register_entrypoint("tool3")

# List current aliases
print("Before cleanup:", angreal.list_entrypoints())
# Output: ['tool1', 'tool2', 'tool3']

# Remove all aliases
angreal.cleanup_entrypoints()

# Verify cleanup
print("After cleanup:", angreal.list_entrypoints())
# Output: []
```

**What Gets Removed:**
- All executable scripts from `~/.local/bin/`
- The entire aliases registry file
- Cross-platform cleanup for all registered aliases

**Use Cases:**
- **Development cleanup**: Remove test aliases during development
- **Uninstallation**: Clean up all aliases before uninstalling angreal
- **Fresh start**: Reset alias configuration
