---
title: Using Docker Integration
---

Angreal provides some minimal integration to docker containers.

To create a [Container]{.title-ref} object:

``` {.sourceCode .python}
from angreal.integrations.docker import Container

c = Container('python:3.5')
c.pull()
c.run()
# or #
c = Container('Dockerfile')
c.build(context='.',
        tag='latest',
        )
```
