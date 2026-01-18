---
title: Include Template Variables
weight: 30
---

# Include Template Variables

You may want to protect a template variable from rendering during the initialization process (for example, having a Jinja template that is part of the Angreal template but shouldn't be templated at initialization).

## Within a File

```bash
{% raw %}
  Hello  {{ template.variable }}.
{% endraw %}
```

will render as:

```bash
Hello {{ template.variable }}.
```

## Within a File/Directory Name

```bash
├── README.md
├── VERSION
├── angreal.json
├── setup.py
└── {% raw %}{{ angreal.name }}{% endraw %}
```

will render as

```bash
├── README.md
├── VERSION
├── angreal.json
├── setup.py
└── {{ angreal.name }}
```
