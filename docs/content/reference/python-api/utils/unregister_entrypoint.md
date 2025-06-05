---
title: Unregister Entrypoint
---

##### unregister_entrypoint(name: str) -> None:
> Remove a command alias created with register_entrypoint

Removes the executable script and updates the registry to unregister a command alias.

**Parameters:**
- `name` (str): The name of the alias to remove

**Raises:**
- `ValueError`: If the alias is not registered
- `OSError`: If unable to remove the script file

```python
import angreal

# Create an alias
angreal.register_entrypoint("temp-tool")

# List current aliases
print("Before:", angreal.list_entrypoints())

# Remove the alias
angreal.unregister_entrypoint("temp-tool")

# Verify removal
print("After:", angreal.list_entrypoints())
```

**What Gets Removed:**
- The executable script from `~/.local/bin/`
- The entry from the aliases registry
- Cross-platform cleanup (handles both Unix scripts and Windows .bat files)

**Note:**
If the script file doesn't exist but the alias is registered, the function will still clean up the registry entry.
