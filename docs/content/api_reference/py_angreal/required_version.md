---
title : Required Version
---


##### required_version(**specifier**: str) -> None:
> set a required version of angreal for a template to use, can use boundary conditions like "`>`,`<`,`>=`,`<=`, `!=`"

{{% notice warning %}}
raises `EnvironmentError` if angreal binary doesn't fit within boundary condition requested
{{% /notice %}}


```python
import angreal

angreal.required_version(">2.0.0")
angreal.required_version("2.0.6")
angreal.required_version("=2.0.4")
```
