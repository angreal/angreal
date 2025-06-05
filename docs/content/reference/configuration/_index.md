---
title: "Configuration"
weight: 3
---

# Configuration Reference

Complete reference for Angreal configuration files and options.

## Template Configuration (angreal.toml)

The `angreal.toml` file defines template variables, user prompts, and validation rules for Angreal templates.

### File Location

- **Templates**: `angreal.toml` in the template root directory
- **Projects**: `.angreal/angreal.toml` in initialized projects

### Basic Structure

```toml
# Variable defaults (root level)
project_name = "my-project"
author_name = "Anonymous"
license = "MIT"
use_docker = false
port = 8080

# Custom prompts for variables
[prompt]
project_name = "Enter your project name"
author_name = "Enter the author name"
license = "Choose a license (MIT, Apache-2.0, GPL-3.0)"

# Validation rules for user input
[validation]
project_name.not_empty = true
project_name.length_min = 3
project_name.length_max = 50
author_name.not_empty = true
license.allowed_values = ["MIT", "Apache-2.0", "GPL-3.0"]
port.type = "integer"
port.min = 1024
port.max = 65535
use_docker.type = "boolean"
```

## Variable Types

Angreal supports these variable types:

### String Variables
```toml
project_name = "default-name"
description = "Default description"
```

### Boolean Variables
```toml
use_docker = false
include_tests = true
```

### Numeric Variables
```toml
port = 8080
timeout = 30
version_major = 1
```

### List Variables
```toml
dependencies = ["requests", "click", "pydantic"]
authors = ["John Doe", "Jane Smith"]
```

### Nested Objects
```toml
[database]
type = "postgresql"
host = "localhost"
port = 5432

[api]
version = "v1"
base_url = "/api/v1"
```

## Custom Prompts

The `[prompt]` section customizes the text shown to users when they provide values:

```toml
[prompt]
project_name = "Enter your project name"
use_docker = "Include Docker support? (y/n)"
license = "Choose a license (MIT, Apache-2.0, GPL-3.0)"
```

**Without custom prompts:**
```
project_name:
```

**With custom prompts:**
```
Enter your project name:
```

## Validation Rules

The `[validation]` section defines rules to validate user input:

### String Validation

```toml
[validation]
# Must not be empty
project_name.not_empty = true

# Length constraints
username.length_min = 3
username.length_max = 20

# Regular expression matching
email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
```

### Numeric Validation

```toml
[validation]
# Type checking
age.type = "integer"
price.type = "float"

# Range constraints
age.min = 18
age.max = 65
port.min = 1024
port.max = 65535
```

### Choice Validation

```toml
[validation]
# Restrict to specific values
license.allowed_values = ["MIT", "Apache-2.0", "GPL-3.0"]
environment.allowed_values = ["development", "staging", "production"]
```

### Boolean Validation

```toml
[validation]
use_docker.type = "boolean"
include_tests.type = "boolean"
```

## Validation Methods

| Method | Description | Example |
|--------|-------------|---------|
| `not_empty` | Field cannot be empty | `field.not_empty = true` |
| `length_min` | Minimum string length | `field.length_min = 3` |
| `length_max` | Maximum string length | `field.length_max = 50` |
| `min` | Minimum numeric value | `field.min = 0` |
| `max` | Maximum numeric value | `field.max = 100` |
| `type` | Type validation | `field.type = "integer"` |
| `allowed_values` | Restrict to specific values | `field.allowed_values = ["a", "b"]` |
| `regex_match` | Regular expression validation | `field.regex_match = "^[a-z]+$"` |

## Error Messages

When validation fails, Angreal shows descriptive error messages:

- **not_empty**: "Field cannot be empty"
- **length_min**: "Must be at least 3 characters"
- **length_max**: "Must be at most 50 characters"
- **min/max**: "Value must be between 18 and 65"
- **type**: "Value must be an integer"
- **allowed_values**: "Input must be one of: MIT, Apache-2.0, GPL-3.0"
- **regex_match**: "Input must match the pattern"

## Environment Variables

Angreal supports these environment variables:

### ANGREAL_DEBUG

Enable debug logging:

```bash
export ANGREAL_DEBUG=true
angreal init template/ project/
```

**Values:**
- `true` - Enable debug logging
- Any other value - Normal logging

### UV Configuration

UV-related environment variables for virtual environment management:

```bash
# Custom UV cache directory
export UV_CACHE_DIR="/path/to/cache"

# Custom UV installation directory
export UV_INSTALL_DIR="/path/to/uv"

# Disable UV progress bars
export UV_NO_PROGRESS=1
```

See [UV Installation and Management](/angreal/reference/configuration/uv-installation) for complete details.

## Global Cache Directory

Angreal caches Git templates in:

- **Location**: `~/.angrealrc/`
- **Purpose**: Stores cloned Git repositories to avoid re-downloading
- **Automatic**: Created and managed automatically

## Project Detection

Angreal detects projects by walking up the directory tree looking for `.angreal/` directories.

**Example directory structure:**
```
my-project/
├── .angreal/
│   ├── angreal.toml
│   └── task_*.py
├── src/
└── README.md
```

When you run `angreal` commands from any subdirectory of `my-project/`, Angreal finds the `.angreal/` directory and loads project-specific tasks.

## Complete Example

Here's a comprehensive `angreal.toml` for a Python web application template:

```toml
# Project variables with defaults
project_name = "my-webapp"
project_description = "A Python web application"
author_name = "Your Name"
author_email = "you@example.com"
license = "MIT"
python_version = "3.11"
use_docker = false
use_database = true
database_type = "postgresql"
include_tests = true
include_docs = false

# Database configuration
[database]
host = "localhost"
port = 5432
name = "webapp_db"

# API configuration
[api]
version = "v1"
port = 8000
base_url = "/api/v1"

# Dependencies
dependencies = ["fastapi", "uvicorn", "sqlalchemy"]
dev_dependencies = ["pytest", "black", "flake8"]

# Custom prompts
[prompt]
project_name = "Enter your project name (lowercase, no spaces)"
project_description = "Brief description of your web application"
author_name = "Your full name"
author_email = "Your email address"
license = "Choose a license (MIT, Apache-2.0, GPL-3.0)"
python_version = "Python version (3.8, 3.9, 3.10, 3.11)"
use_docker = "Include Docker support? (y/n)"
use_database = "Include database support? (y/n)"
database_type = "Database type (postgresql, mysql, sqlite)"

# Validation rules
[validation]
project_name.not_empty = true
project_name.length_min = 3
project_name.length_max = 30
project_name.regex_match = "^[a-z][a-z0-9_-]*$"

author_name.not_empty = true
author_name.length_min = 2

author_email.not_empty = true
author_email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"

license.allowed_values = ["MIT", "Apache-2.0", "GPL-3.0"]

python_version.allowed_values = ["3.8", "3.9", "3.10", "3.11"]

database_type.allowed_values = ["postgresql", "mysql", "sqlite"]

database.port.type = "integer"
database.port.min = 1
database.port.max = 65535

api.port.type = "integer"
api.port.min = 1024
api.port.max = 65535

use_docker.type = "boolean"
use_database.type = "boolean"
include_tests.type = "boolean"
include_docs.type = "boolean"
```

## See Also

- [Template Development](/angreal/how-to-guides/create-templates) - Creating templates with configuration
- [CLI Reference](/angreal/reference/cli) - Command-line options and usage
- [Tera Templating](https://keats.github.io/tera/docs/) - Template syntax reference
