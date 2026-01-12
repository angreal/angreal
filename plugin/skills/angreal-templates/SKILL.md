---
name: angreal-templates
description: This skill should be used when the user asks to "create an angreal template", "make a project template", "build a reusable template", "share a template", "write angreal.toml", "use Tera templating", "template variables", "conditional templates", or needs guidance on creating templates that others can consume via `angreal init`, template configuration, Tera syntax, or publishing templates.
version: 2.8.0
---

# Creating Angreal Templates

Create reusable project templates that others can consume via `angreal init <template>`.

## What Templates Are For

Templates let you create reusable project scaffolds. Users initialize new projects with:

```bash
angreal init https://github.com/user/my-template
angreal init /path/to/local/template
```

This is different from `angreal-init` (adding angreal to existing projects) - templates create entirely new projects from scratch.

## Template Structure

Every angreal template follows this structure:

```
my-template/
├── angreal.toml              # Template configuration (required)
├── {{ project_name }}/       # Templated directories
│   ├── src/
│   └── tests/
├── README.md                 # Templated files
├── static_file.txt           # Static files (copied as-is)
└── .angreal/                 # Optional: post-init tasks
    ├── init.py               # Post-initialization script
    └── task_*.py             # Template-specific tasks
```

## Template Configuration (angreal.toml)

The `angreal.toml` file defines template variables at the root level:

```toml
# Variables with defaults - users will be prompted for these
project_name = "my-project"
author_name = "Anonymous"
description = "A new project"
license = "MIT"
use_docker = false
python_version = "3.11"

# Optional: Custom prompts for better UX
[prompt]
project_name = "Enter your project name"
author_name = "Enter your name"
license = "Choose a license (MIT, Apache-2.0, GPL-3.0)"

# Optional: Validation rules
[validation]
project_name.not_empty = true
project_name.length_min = 3
license.allowed_values = ["MIT", "Apache-2.0", "GPL-3.0"]
```

### Variable Types

```toml
# String variables
project_name = "default-name"
author = "Your Name"

# Boolean variables
use_docker = false
include_tests = true

# Numeric variables
port = 8080

# List variables
dependencies = ["requests", "click", "pydantic"]
```

### Custom Prompts ([prompt] section)

Customize what users see when providing values:

```toml
[prompt]
project_name = "Enter your project name (lowercase, no spaces)"
author_email = "Your email address"
use_docker = "Include Docker support? (y/n)"
```

### Validation Rules ([validation] section)

Validate user input with these options:

```toml
[validation]
# Required field
project_name.not_empty = true

# String length
username.length_min = 3
username.length_max = 20

# Numeric range
port.type = "integer"
port.min = 1024
port.max = 65535

# Allowed values
license.allowed_values = ["MIT", "Apache-2.0", "GPL-3.0"]

# Regex pattern
email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
```

## Tera Templating Engine

Angreal uses [Tera](https://keats.github.io/tera/docs/) (similar to Jinja2) for templating.

### Variable Substitution

```markdown
# {{ project_name }}

{{ description }}

Created by {{ author_name }}
```

### Conditionals

```markdown
# {{ project_name }}

{% if use_docker %}
## Docker

Build and run with Docker:

```bash
docker build -t {{ project_name }} .
docker run {{ project_name }}
```
{% endif %}

{% if include_tests %}
## Testing

```bash
pytest tests/
```
{% endif %}
```

### Loops

```python
# requirements.txt
{% for dep in dependencies %}
{{ dep }}
{% endfor %}
```

### Filters

```markdown
# {{ project_name | title }}

Package: {{ project_name | lower | replace(from=" ", to="-") }}
Class: {{ project_name | title | replace(from=" ", to="") }}
```

Common filters:
- `upper` / `lower` - Change case
- `title` - Title case
- `replace(from="x", to="y")` - Replace text
- `trim` - Remove whitespace

## Templated Names

### Directory Names

```
my-template/
└── {{ project_name }}/
    ├── {{ project_name }}/
    │   └── __init__.py
    └── tests/
```

With `project_name = "my-app"` creates:
```
my-app/
├── my-app/
│   └── __init__.py
└── tests/
```

### File Names

```
{{ project_name }}.py
test_{{ project_name }}.py
```

## Escaping Template Syntax

Use `{% raw %}` to preserve template syntax in output files:

```
{% raw %}
{
  "name": "{{ package_name }}",
  "scripts": {
    "test": "jest"
  }
}
{% endraw %}
```

This outputs the literal `{{ package_name }}` without substitution. Use for:
- JSON files with curly braces
- Jinja/Tera templates you want to include in generated projects
- Documentation showing template examples

## Conditional File/Directory Creation

Control which files and directories are created based on template variables.

### Conditional File Content (Empty = No File)

Create files that are empty (and thus effectively skipped) when conditions aren't met:

```
{# Dockerfile - only has content if use_docker is true #}
{% if use_docker %}
FROM python:{{ python_version }}
WORKDIR /app
COPY . .
RUN pip install -e .
CMD ["python", "-m", "{{ project_name }}"]
{% endif %}
```

### Conditional Directories via Post-Init Script

Use `.angreal/init.py` to create or remove directories based on variables:

```python
# .angreal/init.py
import shutil
import os
import angreal

def init():
    context = angreal.get_context()
    project = context.get('project_name', 'project')

    # Conditionally create directories
    if context.get('include_docs', False):
        os.makedirs(f"{project}/docs", exist_ok=True)

    # Remove directories if feature not selected
    if not context.get('use_docker', False):
        docker_path = f"{project}/docker"
        if os.path.exists(docker_path):
            shutil.rmtree(docker_path)

    # Remove optional files
    if not context.get('include_ci', False):
        ci_file = f"{project}/.github/workflows/ci.yml"
        if os.path.exists(ci_file):
            os.remove(ci_file)
```

### Pattern: Optional Feature Directories

Structure your template with optional feature directories:

```
my-template/
├── angreal.toml
├── {{ project_name }}/
│   ├── src/                    # Always included
│   ├── tests/                  # Always included
│   ├── docker/                 # Removed if use_docker=false
│   │   ├── Dockerfile
│   │   └── docker-compose.yml
│   ├── docs/                   # Removed if include_docs=false
│   │   └── index.md
│   └── .github/                # Removed if include_ci=false
│       └── workflows/
│           └── ci.yml
└── .angreal/
    └── init.py                 # Handles conditional removal
```

```toml
# angreal.toml
project_name = "my-project"
use_docker = false
include_docs = true
include_ci = true

[prompt]
use_docker = "Include Docker support? (y/n)"
include_docs = "Include documentation? (y/n)"
include_ci = "Include CI/CD config? (y/n)"
```

### Pattern: Conditional Config Files

Include config files only when needed:

```python
# .angreal/init.py
import os
import angreal

def init():
    context = angreal.get_context()
    project = context.get('project_name')

    # Create pytest.ini only if using pytest
    if context.get('use_pytest', False):
        with open(f"{project}/pytest.ini", 'w') as f:
            f.write("[pytest]\ntestpaths = tests\n")

    # Create .env.example only if using env vars
    if context.get('use_env_vars', False):
        env_vars = context.get('env_vars', ['DEBUG', 'API_KEY'])
        with open(f"{project}/.env.example", 'w') as f:
            for var in env_vars:
                f.write(f"{var}=\n")
```

### Pattern: Choose Between Alternatives

Let users choose between mutually exclusive options:

```toml
# angreal.toml
project_name = "my-app"
web_framework = "flask"

[prompt]
web_framework = "Choose web framework (flask, fastapi, django)"

[validation]
web_framework.allowed_values = ["flask", "fastapi", "django"]
```

```python
# .angreal/init.py
import shutil
import os
import angreal

def init():
    context = angreal.get_context()
    project = context.get('project_name')
    framework = context.get('web_framework', 'flask')

    # Template includes all framework options
    frameworks = ['flask', 'fastapi', 'django']

    # Remove non-selected framework files
    for fw in frameworks:
        if fw != framework:
            fw_dir = f"{project}/templates/{fw}"
            if os.path.exists(fw_dir):
                shutil.rmtree(fw_dir)

    # Rename selected framework dir
    selected = f"{project}/templates/{framework}"
    if os.path.exists(selected):
        # Move contents up and remove framework dir
        for item in os.listdir(selected):
            shutil.move(f"{selected}/{item}", f"{project}/{item}")
        os.rmdir(selected)
        os.rmdir(f"{project}/templates")
```

## Post-Initialization Script

Create `.angreal/init.py` to run code after template rendering:

```python
"""Post-initialization script."""
import subprocess
import angreal

def init():
    """Run after template is rendered."""
    context = angreal.get_context()
    project_name = context.get('project_name', 'project')

    print(f"Initializing {project_name}...")

    # Initialize git
    subprocess.run(['git', 'init'], cwd=project_name)

    # Install dependencies if requested
    if context.get('install_dependencies', False):
        subprocess.run(
            ['pip', 'install', '-r', 'requirements.txt'],
            cwd=project_name
        )

    print(f"Done! Next steps:")
    print(f"  cd {project_name}")
    print(f"  angreal tree")
```

## Template-Specific Tasks

Add tasks useful for projects created from your template:

```python
# .angreal/task_setup.py
import angreal
import subprocess

@angreal.command(name="setup", about="Set up development environment")
def setup():
    """Initialize the development environment."""
    project_root = angreal.get_root().parent

    print("Creating virtual environment...")
    subprocess.run(['python', '-m', 'venv', '.venv'], cwd=project_root)

    print("Installing dependencies...")
    subprocess.run(
        ['.venv/bin/pip', 'install', '-r', 'requirements.txt'],
        cwd=project_root
    )

    print("Setup complete!")
    return 0
```

## Complete Template Example

### Directory Structure

```
python-template/
├── angreal.toml
├── {{ project_name }}/
│   ├── {{ project_name }}/
│   │   └── __init__.py
│   ├── tests/
│   │   └── test_main.py
│   ├── README.md
│   ├── pyproject.toml
│   └── .gitignore
└── .angreal/
    ├── init.py
    └── task_dev.py
```

### angreal.toml

```toml
# Variables at root level
project_name = "my-project"
author = "Your Name"
email = "you@example.com"
description = "A Python project"
python_version = "3.11"
use_pytest = true

# Custom prompts
[prompt]
project_name = "Enter your project name"
author = "Your full name"
email = "Your email address"

# Validation
[validation]
project_name.not_empty = true
project_name.length_min = 3
email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
```

### {{ project_name }}/README.md

```markdown
# {{ project_name }}

{{ description }}

## Installation

```bash
pip install -e .
```

{% if use_pytest %}
## Testing

```bash
pytest
```
{% endif %}

## Author

{{ author }} <{{ email }}>
```

### {{ project_name }}/pyproject.toml

```toml
[project]
name = "{{ project_name }}"
version = "0.1.0"
description = "{{ description }}"
authors = [
    {name = "{{ author }}", email = "{{ email }}"}
]
requires-python = ">={{ python_version }}"

{% if use_pytest %}
[project.optional-dependencies]
dev = ["pytest"]
{% endif %}
```

## Publishing Templates

### GitHub

1. Create a repository with your template
2. Users consume via: `angreal init https://github.com/user/template`

### Local Sharing

1. Share template directory
2. Users consume via: `angreal init /path/to/template`

### Best Practices

1. **Include a README** - Document variables and what the template creates
2. **Sensible defaults** - All variables should have reasonable defaults
3. **Minimal prompts** - Don't ask for too many variables
4. **Include .gitignore** - Template should create git-ready projects
5. **Add setup task** - Include `.angreal/` with dev setup commands
6. **Test your template** - Run `angreal init` on your own template

## Template API Functions

For advanced use in tasks:

```python
import angreal

# Render a template STRING (not a file!)
# Takes a template string and returns the rendered result
template = "Hello {{ name }}!"
result = angreal.render_template(template, {"name": "World"})
# result == "Hello World!"

# Render entire directory (positional args: src, dst, force, context)
rendered_files = angreal.render_directory(
    "templates/project",
    "my-project",
    False,
    {"project_name": "My Project"}
)
# Returns list of created file paths

# Generate context from angreal.toml
# Parameters: path, take_input
context = angreal.generate_context(
    path="angreal.toml",
    take_input=True  # Prompt user for values
)

# Get current template context (during template initialization)
context = angreal.get_context()
```

### Function Signatures

```python
# Render a template string
angreal.render_template(template: str, context: dict) -> str

# Render a templated directory (context is optional)
angreal.render_directory(src: str, dst: str, force: bool, context: dict | None) -> list[str]

# Generate context from TOML file
angreal.generate_context(path: str, take_input: bool) -> dict
```
