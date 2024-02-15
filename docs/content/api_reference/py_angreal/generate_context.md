---
title: Generate Context
---

##### generate_context((**path**: str, **take_input**: bool) -> dict[strin,Any]
> function that generates a context from a given toml document

```python
import angreal

toml = "angreal.toml"
ctx = angreal.generate_context(path=toml,take_input=False)
assert isinstance(ctx,dict)
assert ctx.get("key_1") == "value_1"
assert ctx.get("key_2") == 1

```
### Args:
- path (str): the path to the toml to use for generating a context
- take_input (bool): whether values should be requested from stdin


The TOML file format is the same as the `angreal.toml` used in the project template, no headers and just key value pairs. It also supports compounded templating where a previously captured key's value can render into subsequent default values.


```bash
key="value"
key2=1
```
