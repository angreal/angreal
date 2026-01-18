---
title: "Render Template"
weight: 1
---

# render_template

Render a template string using Tera templating engine with the provided context.

## Signature

```python
angreal.render_template(template: str, context: dict) -> str
```

## Parameters

- **template** (str): Template string with Tera syntax (e.g., `{{ variable }}`)
- **context** (dict): Dictionary containing variables to substitute in the template

## Returns

- **str**: Rendered template string with variables substituted

## Example

```python
import angreal

# Basic variable substitution
result = angreal.render_template("Hello {{ name }}!", {"name": "world"})
assert result == "Hello world!"

# Multiple variables
template = "{{ project }} version {{ version }} by {{ author }}"
context = {
    "project": "Angreal",
    "version": "2.0.0",
    "author": "Your Name"
}
result = angreal.render_template(template, context)
# Returns: "Angreal version 2.0.0 by Your Name"

# Conditional rendering
template = """
{% if use_docker %}
Docker support enabled
{% else %}
Docker support disabled
{% endif %}
"""
result = angreal.render_template(template, {"use_docker": True})
# Returns: "Docker support enabled"
```

## Template Syntax

Uses Tera templating syntax (similar to Jinja2):

- **Variables**: `{{ variable_name }}`
- **Conditionals**: `{% if condition %}...{% endif %}`
- **Loops**: `{% for item in list %}...{% endfor %}`
- **Filters**: `{{ variable | filter_name }}`

## Common Use Cases

- Rendering file contents during template initialization
- Generating configuration files with user-provided values
- Creating dynamic content in Angreal tasks

## See Also

- [Template Development Guide](/angreal/how-to-guides/create-templates) - Complete templating guide
- [Tera Documentation](https://keats.github.io/tera/docs/) - Template syntax reference
