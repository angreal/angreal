---
title: "Create Prompts"
weight: 16
---

# Create Prompts

Prompts provide context and clarity for variables in your template. They help users understand what each variable is for and how it will be used.

## Basic Prompts

Define prompts in the `[prompt]` section of your `angreal.toml` file:

```toml
# Default values
project_slug = "my-project"
author_name = "Anonymous"

# Custom prompts
[prompt]
project_slug = "The project slug is used as the identifier for your project. It should be lowercase with hyphens. What should it be?"
author_name = "Your name will be used in documentation and license files. What is your full name?"
```

## Providing Context

Prompts should explain:
- What the variable is for
- How it will be used
- Any constraints or requirements

```toml
# Default values
project_slug = "my-project"
license = "MIT"
port = 8080

# Custom prompts
[prompt]
project_slug = "The project slug is used in URLs and package names. It should be lowercase with hyphens. What should it be?"
license = "The license determines how others can use your code. Choose from MIT, Apache-2.0, or GPL-3.0"
port = "The port number your application will listen on. Must be between 1024 and 65535"
```

## Combining with Validation

While prompts provide context, validation rules enforce requirements:

```toml
# Default values
project_slug = "my-project"
license = "MIT"
port = 8080

# Custom prompts
[prompt]
project_slug = "The project slug is used in URLs and package names. It should be lowercase with hyphens. What should it be?"
license = "The license determines how others can use your code. Choose from MIT, Apache-2.0, or GPL-3.0"
port = "The port number your application will listen on. Must be between 1024 and 65535"

# Validation rules
[validation]
project_slug.regex_match = "^[a-z0-9-]+$"
license.allowed_values = ["MIT", "Apache-2.0", "GPL-3.0"]
port.min = 1024
port.max = 65535
port.type = "integer"
```

## Example

Here's a complete example showing how prompts provide context for each variable:

```toml
# Project configuration
project_slug = "new-project"
author_name = "Developer"
version = "0.1.0"
description = "A new awesome project"
include_docs = true
include_tests = true
python_version = "3.9"
role = "user"

# Custom prompts
[prompt]
project_slug = "The project slug is used in URLs and package names. It should be lowercase with hyphens. What should it be?"
author_name = "Your name will be used in documentation and license files. What is your full name?"
version = "The initial version number for your project. Use semantic versioning (e.g., 0.1.0)"
description = "A brief description of your project. This will be used in package metadata and documentation"
include_docs = "Documentation helps others use your project. Should we include a docs directory? (true/false)"
include_tests = "Tests help ensure your code works correctly. Should we include a tests directory? (true/false)"
python_version = "The Python version your project will use. Choose from 3.8, 3.9, 3.10, or 3.11"
role = "The role determines what actions a user can perform. Choose from admin, user, or guest"

# Validation rules
[validation]
role.allowed_values = ["admin", "user", "guest"]
python_version.allowed_values = ["3.8", "3.9", "3.10", "3.11"]
```

## Related Documentation

- [Use Validation Rules](/angreal/how-to-guides/use-validation-rules) - Learn about input validation
- [Configuration Reference](/angreal/reference/configuration/) - Complete reference for angreal.toml
