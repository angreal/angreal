---
title: Register Entrypoint
---

##### register_entrypoint(name: str) -> None:
> Create a command alias for angreal that allows white-labeling and custom command names

Creates an executable script in `~/.local/bin/` that redirects to angreal, enabling you to use custom command names for your organization or project.

**Parameters:**
- `name` (str): The name for the new command alias

**Raises:**
- `ValueError`: If the alias name conflicts with existing commands
- `OSError`: If unable to create the script file or set permissions

```python
import angreal

# Create a custom command alias
angreal.register_entrypoint("mycompany-tool")

# Now you can use 'mycompany-tool' instead of 'angreal'
# Example: mycompany-tool init template/
# Example: mycompany-tool build --release
```

**Generated Script Location:**
- Unix/Linux/macOS: `~/.local/bin/{name}`
- Windows: `~/.local/bin/{name}.bat`

**Registry:**
Aliases are tracked in `~/.angrealrc/aliases.json` for management purposes.

**Use Cases:**
- **White-labeling**: Rebrand angreal for your organization
- **Project-specific tools**: Create project-specific command names
- **Team workflows**: Standardize command names across teams
