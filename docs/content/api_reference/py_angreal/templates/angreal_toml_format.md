---
title: angreal.toml Format
weight: 15
---

# angreal.toml Format

The `angreal.toml` file is a key component of any Angreal template. It defines the template variables and their default values, and now supports enhanced prompting for template users.

## Basic Structure

The `angreal.toml` file uses TOML format with a simple key-value structure:

```toml
# Basic variables with default values
variable_name = "default_value"
numeric_variable = 42
boolean_flag = true
```

## Enhanced Prompts

Starting with this version, you can add a separate `[prompt]` section to provide more user-friendly prompts during template rendering:

```toml
# Default values
project_name = "my_project"
author = "Your Name"
version = "0.1.0"
include_tests = true

# Custom prompts for each variable
[prompt]
project_name = "What should we name your project?"
author = "Enter your full name"
version = "Enter the initial version number"
include_tests = "Do you want to include a test directory? (true/false)"
```

### Benefits of Using Prompts

- Clearer instructions for users filling out template variables
- Separation of default values from prompt text
- Better user experience when initializing projects
- Backward compatible with existing templates

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

Here's a complete example of an `angreal.toml` file:

```toml
# Project configuration
project_name = "new_project"
author = "Developer"
version = "0.1.0"
description = "A new awesome project"
include_docs = true
include_tests = true
python_version = "3.9"

# Custom prompts for better user experience
[prompt]
project_name = "What is the name of your project?"
author = "Who is the main author/maintainer?"
version = "What version would you like to start with?"
description = "Provide a brief description of your project:"
include_docs = "Include documentation directory? (true/false)"
include_tests = "Include test directory? (true/false)"
python_version = "Which Python version will this project use?"
```

When users run `angreal init` with this template, they will see the custom prompts rather than just variable names with default values.
