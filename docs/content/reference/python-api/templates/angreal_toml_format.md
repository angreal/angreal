---
title: angreal.toml Format
weight: 15
---

# angreal.toml Format

The `angreal.toml` file is a key component of any Angreal template. It defines the template variables and their default values, and now supports enhanced prompting and validation for template users.

## Basic Structure

The `angreal.toml` file uses TOML format with a simple key-value structure:

```toml
# Basic variables with default values
variable_name = "default_value"
numeric_variable = 42
boolean_flag = true
```

## Enhanced Prompts and Validation

Starting with this version, you can add separate `[prompt]` and `[validation]` sections to provide more user-friendly prompts and input validation during template rendering:

```toml
# Default values
project_name = "my_project"
author = "Your Name"
version = "0.1.0"
include_tests = true
role = "user"

# Custom prompts for each variable
[prompt]
project_name = "What should we name your project?"
author = "Enter your full name"
version = "Enter the initial version number"
include_tests = "Do you want to include a test directory? (true/false)"
role = "Select a role (admin, user, guest)"

# Validation rules for inputs
[validation]
role.allowed_values = ["admin", "user", "guest"]
```

### Benefits of Enhanced Inputs

- Clearer instructions for users filling out template variables
- Input validation to prevent errors
- Separation of default values from prompt text
- Better user experience when initializing projects
- Backward compatible with existing templates

### Validation Methods

The validation system uses a dotted notation to apply validation methods to specific variables:

```toml
[validation]
variable_name.validation_method = validation_parameters
```

Currently supported validation methods:

| Method | Description | Example |
|--------|-------------|---------|
| `allowed_values` | Validates that input is one of the specified values | `role.allowed_values = ["admin", "user", "guest"]` |
| `min` | Validates that a numeric input is greater than or equal to a value | `age.min = 18` |
| `max` | Validates that a numeric input is less than or equal to a value | `age.max = 65` |
| `regex_match` | Validates that input matches a regular expression pattern | `email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"` |
| `not_empty` | Validates that input is not empty | `name.not_empty = true` |
| `type` | Validates that input can be parsed as a specific type | `age.type = "integer"` |
| `length_min` | Validates that input has at least the specified length | `password.length_min = 8` |
| `length_max` | Validates that input does not exceed the specified length | `username.length_max = 20` |

You can apply multiple validation rules to the same variable:

```toml
[validation]
# Validate age is between 18 and 65 and an integer
age.min = 18
age.max = 65
age.type = "integer"

# Validate email format using regex and not empty
email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
email.not_empty = true

# Validate score is one of the valid values and in range
score.min = 0
score.max = 100
score.allowed_values = [0, 25, 50, 75, 100]

# Validate username length and type
username.length_min = 3
username.length_max = 20
username.type = "string"
username.not_empty = true
```

## Template Variable Usage

Variables defined in `angreal.toml` become available in your templates and can be referenced using the Tera templating syntax:

```
Hello {{ project_name }}!

This project was created by {{ author }}.
```

They can also be used in template filenames and directory names:

```
/template_dir/
  angreal.toml
  /{{ project_name }}/
    README.md
```

## Example

Here's a complete example of an `angreal.toml` file with both prompts and validation:

```toml
# Project configuration
project_name = "new_project"
author = "Developer"
version = "0.1.0"
description = "A new awesome project"
include_docs = true
include_tests = true
python_version = "3.9"
role = "user"

# Custom prompts for better user experience
[prompt]
project_name = "What is the name of your project?"
author = "Who is the main author/maintainer?"
version = "What version would you like to start with?"
description = "Provide a brief description of your project:"
include_docs = "Include documentation directory? (true/false)"
include_tests = "Include test directory? (true/false)"
python_version = "Which Python version will this project use?"
role = "Select a role (admin, user, guest)"

# Validation rules
[validation]
role.allowed_values = ["admin", "user", "guest"]
python_version.allowed_values = ["3.8", "3.9", "3.10", "3.11"]
```

When users run `angreal init` with this template, they will see the custom prompts rather than just variable names with default values. If they enter invalid input for fields with validation rules, they'll be prompted to enter valid values.
