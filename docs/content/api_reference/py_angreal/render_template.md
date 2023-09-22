---
title : Render Template
---


##### get_template( **template**: str, **context**: dict[str,str]) -> str:
> render a template string given a context

```python
import angreal

def render_template():
    x = angreal.render_template("Hello {{ name }}!", dict(name='world'))

    assert x == "Hello world!"
```
