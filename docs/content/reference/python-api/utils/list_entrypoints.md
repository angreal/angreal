---
title: List Entrypoints
---

##### list_entrypoints() -> List[str]:
> List all registered command aliases created with register_entrypoint

Returns a list of all command aliases that have been registered for angreal.

**Returns:**
- `List[str]`: List of registered alias names

```python
import angreal

# Create some aliases
angreal.register_entrypoint("myproject")
angreal.register_entrypoint("company-tool")

# List all registered aliases
aliases = angreal.list_entrypoints()
print("Registered aliases:", aliases)
# Output: ['myproject', 'company-tool']

# Check if a specific alias exists
if "myproject" in aliases:
    print("myproject alias is registered")
```

**Registry Location:**
The function reads from `~/.angrealrc/aliases.json` where all registered aliases are tracked.
