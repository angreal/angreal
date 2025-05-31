---
title : Get Context
---


##### get_context() -> dict:
> get the context that was used to render the original template, returns an empty dictionary if no angreal.toml is found

```python
import angreal

# Get the context as a dictionary
context = angreal.get_context()

# Example: Accessing values from the context
if context:
    print("Project name:", context.get("project_name", "Not set"))
    print("Version:", context.get("version", "Not set"))

# Example: Setting a new value in the context
context["new_key"] = "new_value"
print("Updated context:", context)
