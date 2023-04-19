---
title: Include Template Variables
---

You may want to protect a template variable from rendering during the
initialization process. (Say having a Jinja template that is part of the
angreal template but shouldn\'t actually be templated at
initialization.)

Within a File
=============

``` {.sourceCode .bash}
{% raw %}
  Hello  {{ template.variable }}.
{% endraw % }
```

will render as:

``` {.sourceCode .bash}
Hello {{ template.variable }}.
```

Within a file/directory name
============================

``` {.sourceCode .bash}
├── README.md
├── VERSION
├── angreal.json
├── setup.py
└── {% raw %}{{ angreal.name }}{% endraw %}
```

will render as

``` {.sourceCode .bash}
├── README.md
├── VERSION
├── angreal.json
├── setup.py
└── {{ angreal.name }}
```
