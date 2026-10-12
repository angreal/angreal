---
title: "Required Version"
---

##### required_version(**specifier**: str) -> None:
> set a required version of angreal for a template to use, can use boundary conditions like "`>`,`<`,`>=`,`<=`, `!=`"

:::caution
raises `EnvironmentError` if angreal binary doesn't fit within boundary condition requested
:::


```python
import angreal

angreal.required_version(">2.0.0")
angreal.required_version("2.0.6")      # exact match (no operator)
angreal.required_version("==2.0.4")    # explicit equality
```
