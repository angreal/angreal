---
title: Template System Guide
weight: 20
---

# Template System Guide

Angreal's template system allows you to generate files and directories from templates. This guide explains how to use the template functions effectively.

## Overview

The template system consists of three main components:

1. **Render Template** - Render individual template files
2. **Render Directory** - Render entire directories of templates
3. **Generate Context** - Create context variables for templates

Angreal uses [Tera](https://keats.github.io/tera/docs/) (a Rust template engine similar to Jinja2) for template rendering.

## Template Variables

Templates can include variables using the `{{ variable }}` syntax:

```
# {{ project_name }}

A project created by {{ author }}

## Installation

```

These variables are replaced with actual values when the template is rendered.

## Rendering a Single Template

To render a single template file:

```python
import angreal

def create_readme():
    # Create context with variables
    context = {
        "project_name": "My Awesome Project",
        "author": "Jane Smith",
        "year": 2025
    }

    # Render a template
    angreal.render_template(
        source_template="templates/README.md.tera",
        target_path="README.md",
        context=context
    )
```

## Rendering a Directory of Templates

To render an entire directory of templates:

```python
import angreal

def create_project():
    # Create context with variables
    context = {
        "project_name": "My Awesome Project",
        "author": "Jane Smith",
        "year": 2025,
        "use_pytest": True
    }

    # Render a directory
    angreal.render_directory(
        source_directory="templates/project",
        target_directory="my-project",
        context=context
    )
```

This will process all files in the `templates/project` directory, replacing variables with values from the context.

## Working with Template Paths

Template and directory paths can be:

1. **Absolute paths** - Starting with `/`
2. **Relative paths** - Relative to the current directory

Example:

```python
import angreal
import os

# Get absolute paths
template_dir = os.path.join(angreal.get_root(), "templates")

# Render using absolute path
angreal.render_template(
    source_template=os.path.join(template_dir, "README.md.tera"),
    target_path="README.md",
    context={"project_name": "My Project"}
)
```

## Template Context

The template context contains the variables available in templates. You can generate a context interactively:

```python
import angreal

def initialize_project():
    # Generate context interactively
    context = angreal.generate_context(
        template_path="templates/project",
        interactive=True
    )

    # Render the project
    angreal.render_directory(
        source_directory="templates/project",
        target_directory=context.get("project_name", "new-project"),
        context=context
    )
```

This will prompt the user for values defined in the template configuration.

## Conditional Rendering

Templates can include conditional logic:

```
# {{ project_name }}

{% if use_pytest %}
## Testing

This project uses pytest for testing.
{% else %}
## Testing

This project uses unittest for testing.
{% endif %}
```

## Related Documentation

- [render_template](render_template) - Full API reference for `angreal.render_template`
- [render_directory](render_directory) - Full API reference for `angreal.render_directory`
- [generate_context](generate_context) - Full API reference for `angreal.generate_context`
- [Tera Documentation](https://keats.github.io/tera/docs/) - Documentation for the Tera template engine
