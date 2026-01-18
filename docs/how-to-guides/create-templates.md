---
title: "Create Templates"
weight: 1
---

# How to Create Angreal Templates

Learn how to create reusable project templates with Angreal's Tera templating engine.

## Prerequisites

- Angreal installed and working
- Basic understanding of file structures
- Familiarity with TOML format

## Template Structure

Every Angreal template follows this structure:

```
my-template/
├── angreal.toml              # Template configuration
├── {{ variable_name }}/      # Templated directories
├── static_file.txt           # Static files (copied as-is)
├── templated_file.md.tera    # Templated files
└── .angreal/                 # Optional: template tasks
    ├── init.py               # Post-initialization script
    └── task_*.py             # Template-specific tasks
```

## Basic Template Configuration

### Create angreal.toml

The `angreal.toml` file defines variables that users will provide:

```toml
# Required template metadata
[template]
name = "my-awesome-template"
description = "A template for awesome projects"
version = "1.0.0"

# Template variables with defaults
project_name = "my-project"
author_name = "Anonymous"
license = "MIT"
use_docker = false
```

### Variable Types

Angreal supports several variable types:

```toml
# String variables
project_name = "default-name"
description = "Default description"

# Boolean variables
use_docker = false
include_tests = true

# Numeric variables
port = 8080
timeout = 30

# Choice variables (user selects from options)
license = "MIT"  # Could be MIT, Apache-2.0, GPL-3.0, etc.
```

## File and Directory Templating

### Templated Directory Names

Use Tera syntax in directory names:

```bash
mkdir "{{ project_name }}"
mkdir "{{ project_name }}/src"
mkdir "{{ project_name }}/tests"
```

When initialized with `project_name = "my-app"`, creates:
- `my-app/`
- `my-app/src/`
- `my-app/tests/`

### Templated File Names

Files can also use template variables:

```bash
touch "{{ project_name }}.py"
touch "test_{{ project_name }}.py"
```

### File Content Templating

#### Simple Variable Substitution

Create `README.md`:

```markdown
# {{ project_name }}

{{ description }}

## Author

Created by {{ author_name }}

## License

This project is licensed under the {{ license }} license.
```

#### Conditional Content

Use Tera's conditional syntax:

<pre><code class="language-markdown">
{{ project_name }}

{{ description }}

{% if use_docker %}
## Docker

This project includes Docker support.

```bash
docker build -t {{ project_name }} .
docker run {{ project_name }}
```
{% endif %}

{% if include_tests %}
## Testing

Run tests with:

```bash
python -m pytest tests/
```
{% endif %}
</code></pre>

#### Loops and Lists

Define lists in `angreal.toml`:

```toml
dependencies = ["requests", "click", "pydantic"]
```

Use in templates:

```python
# requirements.txt
{% for dep in dependencies %}
{{ dep }}
{% endfor %}
```

Or for more complex structures:

```python
# setup.py
install_requires = [
{% for dep in dependencies %}
    "{{ dep }}",
{% endfor %}
]
```

#### Filters

Tera provides useful filters:

```markdown
# {{ project_name | title }}

Package name: {{ project_name | lower | replace(from=" ", to="-") }}
Class name: {{ project_name | title | replace(from=" ", to="") }}
```

Common filters:
- `upper` / `lower` - Change case
- `title` - Title case
- `replace(from="x", to="y")` - Replace text
- `trim` - Remove whitespace
- `length` - Get length



### File Extension Handling

For files that contain Tera-like syntax that you don't want to be processed, use the `{% raw %}` tag:

```bash
# Template file: package.json
{% raw %}
{
  "name": "{{ project_name }}",
  "version": "1.0.0",
  "scripts": {
    {% if include_tests %}
    "test": "jest"
    {% endif %}
  }
}
{% endraw %}
```

The `{% raw %}` tag tells Tera to treat everything between the tags as literal text, preventing any template processing within that block. This is particularly useful for:
- JSON files that might contain curly braces
- Template files you keep as part of a task
- Configuration files with Tera-like syntax
- Documentation files that need to show template examples
- Any file where you want to preserve the exact syntax

## Template Tasks and Initialization

### Post-Initialization Script

Create `.angreal/init.py` to run code after template processing:

```python
"""Post-initialization script."""
import os
import subprocess

def init():
    """Run after template is rendered."""

    # Get template variables
    context = angreal.get_context()
    project_name = context.get('project_name', 'project')

    print(f"Initializing {project_name}...")

    # Initialize git repository
    subprocess.run(['git', 'init'], cwd=project_name)

    # Install dependencies if specified
    if context.get('install_dependencies', False):
        print("Installing dependencies...")
        subprocess.run(['pip', 'install', '-r', 'requirements.txt'],
                      cwd=project_name)

    print("Template initialization complete!")
    print(f"Next steps:")
    print(f"  cd {project_name}")
    print(f"  # Start developing!")
```

### Template-Specific Tasks

Add tasks that are useful for projects created from your template:

```python
# .angreal/task_setup.py
import angreal
import subprocess

@angreal.command(name="setup", about="Set up development environment")
@angreal.argument(name="install_deps", long="install-deps", is_flag=True,
                  help="Install Python dependencies")
def setup_project(install_deps=False):
    """Set up the development environment."""

    if install_deps:
        print("Installing dependencies...")
        subprocess.run(['pip', 'install', '-r', 'requirements.txt'])

    print("Creating virtual environment...")
    subprocess.run(['python', '-m', 'venv', '.venv'])

    print("Setup complete!")
```
