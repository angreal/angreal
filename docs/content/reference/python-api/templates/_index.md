---
title: Templates System
weight: 20
---


The template system allows you to generate files and directories from templates. It forms the foundation of Angreal's project creation capabilities.

## Overview

Angreal's template system consists of:

- **Template Rendering** - Generate files from templates
- **Directory Rendering** - Process entire directories of templates
- **Context Generation** - Create variables for templates
- **angreal.toml Format** - Configure template variables and prompts

Templates use the [Tera](https://keats.github.io/tera/docs/) templating engine, which provides Jinja2-like syntax.

## Key Components

| Component | Description | Documentation |
|-----------|-------------|---------------|
| `render_template` | Render a single template file | [API Reference](render_template) |
| `render_directory` | Process a directory of templates | [API Reference](render_directory) |
| `generate_context` | Create context variables for templates | [API Reference](generate_context) |
| `angreal.toml` | Define template variables and prompts | [Format Reference](angreal_toml_format) |

## Comprehensive Guide

For a complete walkthrough of using templates, see the [Template System Guide](template_guide/).

## Example

```python
import angreal
import os

def create_project():
    # Generate context for templates
    context = angreal.generate_context(
        template_path="templates/project",
        interactive=True
    )

    # Render a directory of templates
    angreal.render_directory(
        source_directory="templates/project",
        target_directory=context.get("project_name", "new-project"),
        context=context
    )
```

## Related Documentation

<!-- Geekdoc automatically generates child page navigation -->
