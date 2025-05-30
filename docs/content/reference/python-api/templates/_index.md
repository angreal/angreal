---
title: "Templates Module"
weight: 2
---

# Templates Module

The templates module provides functions for working with Angreal project templates.

## Available Functions

- [`render_template`](/reference/python-api/templates/render_template) - Render a single template file
- [`render_directory`](/reference/python-api/templates/render_directory) - Render a directory of templates
- [`generate_context`](/reference/python-api/templates/generate_context) - Generate template context

## Template Format

- [angreal.toml Format](/reference/python-api/templates/angreal_toml_format) - Template configuration file format
- [Template Guide](/reference/python-api/templates/template_guide) - Comprehensive templating guide

## Quick Example

```python
import angreal
from pathlib import Path

# Render a single template
angreal.render_template(
    template_path="template.txt.j2",
    output_path="output.txt",
    context={"name": "My Project"}
)

# Render a directory
angreal.render_directory(
    source_dir=Path("templates"),
    output_dir=Path("output"),
    context={"project_name": "awesome"}
)
```

## See Also

- [Include Jinja Templates](/how-to-guides/include-jinja-templates) - How-to guide
- [Your First Angreal](/tutorials/your_first_angreal) - Tutorial with templates
