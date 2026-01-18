---
title: Use Validation Rules
weight: 60
---

# How to Use Template Validation Rules

Angreal templates can now include validation rules for user input, ensuring that template variables meet specific criteria. This guide explains how to add validation rules to your templates.

## Basic Validation Setup

Validation rules are defined in the `[validation]` section of your `angreal.toml` file using dotted notation:

```toml
[validation]
variable_name.validation_method = validation_parameters
```

## Available Validation Methods

### allowed_values

The `allowed_values` method validates that user input is one of the specified values:

```toml
[validation]
role.allowed_values = ["admin", "user", "guest"]
python_version.allowed_values = ["3.8", "3.9", "3.10", "3.11"]
```

### min and max

The `min` and `max` methods validate that numeric inputs are within a specific range:

```toml
[validation]
# Age must be between 18 and 65
age.min = 18
age.max = 65

# Temperature must be positive
temperature.min = 0

# Percentage must be at most 100
percentage.max = 100
```

### regex_match

The `regex_match` method validates that inputs match a regular expression pattern:

```toml
[validation]
# Validate email format
email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"

# Validate phone number format
phone.regex_match = "^\\d{3}-\\d{3}-\\d{4}$"

# Validate alphanumeric input
username.regex_match = "^[a-zA-Z0-9]+$"
```

### not_empty

The `not_empty` method ensures that a field is not left empty:

```toml
[validation]
# Name is required
name.not_empty = true

# Description is optional
description.not_empty = false
```

### type

The `type` method validates that input can be parsed as a specific data type:

```toml
[validation]
# Must be an integer
age.type = "integer"

# Must be a float/number
price.type = "float"

# Must be a boolean (true/false)
active.type = "boolean"

# Must be a string (always passes)
name.type = "string"
```

### length_min and length_max

The `length_min` and `length_max` methods validate the length of string inputs:

```toml
[validation]
# Username between 3 and 20 characters
username.length_min = 3
username.length_max = 20

# Password at least 8 characters
password.length_min = 8

# Code must be exactly 6 characters
code.length_min = 6
code.length_max = 6
```

## Complete Example

Here's an example of an `angreal.toml` file with validation rules:

```toml
# Template variables
project_name = "my_project"
author = "Developer"
role = "user"
python_version = "3.9"
age = 30
email = "developer@example.com"
score = 75
username = "user123"
password = ""

# Custom prompts
[prompt]
project_name = "What is the name of your project?"
author = "Who is the main author/maintainer?"
role = "Select a role (admin, user, guest)"
python_version = "Which Python version will this project use?"
age = "Enter your age (must be between 18 and 65)"
email = "Enter your email address"
score = "Enter a score (0, 25, 50, 75, or 100)"
username = "Choose a username (3-20 characters, alphanumeric)"
password = "Create a password (min 8 characters)"

# Validation rules
[validation]
# Basic value validation
role.allowed_values = ["admin", "user", "guest"]
python_version.allowed_values = ["3.8", "3.9", "3.10", "3.11"]

# Numeric validation
age.min = 18
age.max = 65
age.type = "integer"
score.allowed_values = [0, 25, 50, 75, 100]

# Format validation
email.regex_match = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
email.not_empty = true

# String length validation
username.length_min = 3
username.length_max = 20
username.regex_match = "^[a-zA-Z0-9]+$"
password.length_min = 8
password.not_empty = true
```

## How Validation Works

When a user runs `angreal init` with a template that includes validation rules:

1. The user is prompted for each variable value
2. If they enter a value that fails validation, they are shown an error message
3. They are prompted again until they provide a valid value
4. If they press Enter without typing anything, the default value is used (bypassing validation)

## Combining with Custom Prompts

For the best user experience, combine validation rules with custom prompts:

```toml
# Variable with default
role = "user"

# User-friendly prompt
[prompt]
role = "Select a role (admin, user, guest)"

# Validation rule
[validation]
role.allowed_values = ["admin", "user", "guest"]
```

This approach:
1. Provides a clear prompt showing what values are acceptable
2. Enforces the validation if the user enters an invalid value
3. Makes your templates more robust and user-friendly
