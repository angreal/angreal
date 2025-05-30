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

```markdown
# {{ project_name }}

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
```

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

## Advanced Templating

### Nested Variables

Create complex data structures:

```toml
[database]
type = "postgresql"
host = "localhost"
port = 5432

[api]
version = "v1"
base_url = "/api/{{ api.version }}"
```

Use in templates:

```yaml
# config.yml
database:
  type: {{ database.type }}
  host: {{ database.host }}
  port: {{ database.port }}

api:
  version: {{ api.version }}
  base_url: {{ api.base_url }}
```

### Comments and Documentation

Use Tera comments to document your templates:

```markdown
{# This section generates the project README #}
# {{ project_name }}

{# Only include Docker section if Docker is enabled #}
{% if use_docker %}
## Docker Support
{# ... docker instructions ... #}
{% endif %}
```

### File Extension Handling

For files that conflict with Tera syntax, use the `.tera` extension:

```bash
# Template file: package.json.tera
{
  "name": "{{ project_name }}",
  "version": "1.0.0",
  "scripts": {
    {% if include_tests %}
    "test": "jest"
    {% endif %}
  }
}
```

The `.tera` extension is automatically removed during template processing.

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

## Template Development Workflow

### 1. Create Template Structure

```bash
mkdir my-template
cd my-template

# Create basic structure
touch angreal.toml
mkdir "{{ project_name }}"
touch "{{ project_name }}/README.md"
```

### 2. Test Template Locally

```bash
# Test from parent directory
cd ..
angreal init ./my-template test-project

# Check results
ls test-project/
cat test-project/README.md
```

### 3. Iterate and Improve

```bash
# Make changes to template
cd my-template
# Edit files...

# Test again
cd ..
rm -rf test-project
angreal init ./my-template test-project
```

### 4. Debug Template Issues

Use verbose output to debug template problems:

```bash
angreal -vv init ./my-template test-project
```

Common issues:
- **Syntax errors**: Check Tera syntax in template files
- **Missing variables**: Ensure all variables are defined in `angreal.toml`
- **File permissions**: Check that template files are readable

## Best Practices

### Template Organization

1. **Keep it simple** - Start with basic variable substitution
2. **Use meaningful defaults** - Provide sensible default values
3. **Document variables** - Comment your `angreal.toml` file
4. **Test thoroughly** - Test with different variable combinations

### Variable Naming

```toml
# Good: Clear, descriptive names
project_name = "my-project"
author_email = "user@example.com"
database_url = "postgresql://localhost/db"

# Avoid: Unclear or confusing names
pn = "project"
email = "user@example.com"  # Could be any email
db = "postgresql://localhost/db"
```

### File Structure

```bash
# Good: Organized structure
my-template/
├── angreal.toml
├── {{ project_name }}/
│   ├── src/
│   ├── tests/
│   ├── docs/
│   └── README.md
└── .angreal/
    └── init.py

# Avoid: Flat structure with many files
my-template/
├── angreal.toml
├── file1.py
├── file2.py
├── file3.py
└── ...
```

### Version Control

Always include in your template repository:

```gitignore
# .gitignore for template repo
.DS_Store
*.pyc
__pycache__/
.pytest_cache/
test-*/  # Ignore test initialization directories
```

## Example: Python Package Template

Here's a complete example for a Python package template:

### angreal.toml

```toml
[template]
name = "python-package"
description = "Modern Python package template"
version = "1.0.0"

# Package information
package_name = "my_package"
package_description = "A Python package"
author_name = "Your Name"
author_email = "your.email@example.com"
license = "MIT"

# Optional features
use_poetry = true
include_cli = false
include_tests = true
python_version = "3.8"
```

### Directory Structure

```bash
python-package/
├── angreal.toml
├── {{ package_name }}/
│   ├── pyproject.toml.tera
│   ├── README.md
│   ├── {{ package_name }}/
│   │   ├── __init__.py
│   │   └── {% if include_cli %}cli.py{% endif %}
│   └── {% if include_tests %}tests/{% endif %}
│       └── {% if include_tests %}test_{{ package_name }}.py{% endif %}
└── .angreal/
    ├── init.py
    └── task_dev.py
```

### Template Files

**README.md:**
```markdown
# {{ package_name }}

{{ package_description }}

## Installation

```bash
pip install {{ package_name }}
```

{% if include_cli %}
## CLI Usage

```bash
{{ package_name }} --help
```
{% endif %}

## Development

{% if use_poetry %}
```bash
poetry install
poetry run pytest
```
{% else %}
```bash
pip install -e .
python -m pytest
```
{% endif %}

## License

{{ license }}
```

**pyproject.toml.tera:**
```toml
[build-system]
requires = ["setuptools>=45", "wheel", "setuptools_scm[toml]>=6.2"]

[project]
name = "{{ package_name }}"
description = "{{ package_description }}"
authors = [{name = "{{ author_name }}", email = "{{ author_email }}"}]
license = {text = "{{ license }}"}
requires-python = ">={{ python_version }}"

{% if include_cli %}
[project.scripts]
{{ package_name }} = "{{ package_name }}.cli:main"
{% endif %}

{% if use_poetry %}
[tool.poetry]
name = "{{ package_name }}"
version = "0.1.0"
description = "{{ package_description }}"
authors = ["{{ author_name }} <{{ author_email }}>"]

[tool.poetry.dependencies]
python = "^{{ python_version }}"
{% if include_cli %}
click = "^8.0.0"
{% endif %}

{% if include_tests %}
[tool.poetry.group.dev.dependencies]
pytest = "^6.0.0"
{% endif %}
{% endif %}
```

This template demonstrates most Angreal templating features in a real-world context.

## Sharing Templates

### Version Control

```bash
cd my-template
git init
git add .
git commit -m "Initial template"
git remote add origin https://github.com/username/my-template.git
git push -u origin main
```

### Usage by Others

```bash
angreal init https://github.com/username/my-template.git new-project
# or
angreal init username/my-template new-project
```

## See Also

- [Tera Documentation](https://tera.netlify.app/docs/) - Complete templating syntax
- [Your First Angreal](/tutorials/your_first_angreal) - Step-by-step tutorial
- [Configuration Reference](/reference/configuration) - angreal.toml format details
- [CLI Reference](/reference/cli) - Template initialization commands