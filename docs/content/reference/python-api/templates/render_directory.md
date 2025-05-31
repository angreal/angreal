---
title: Render Directory
---

##### render_directory(**src**: str, **dst**: str, **force**: bool, **context**: dict) -> list[str]
> renders a templated directory to a destination given a specific context, will overwrite if force = True

```python
import angreal

ctx = angreal.generate_context(toml_path,False)
x = angreal.render_directory(src=src, dst="",force=False, context=ctx )
for f in x:
    assert os.path.exists(f)

```

### Args:
- src (str): the source directory to render, should follow the same pattern as an angreal template
- dst (str): the destination to render to
- force (bool): should you over write existing files/folders
- context (dict): the context to apply to the rendering, usually generated via generate_context, but could be any dictionary with relevant key/value pairs
