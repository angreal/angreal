---
title: Use Shell Completion
weight: 20
---

# Use Shell Completion

Angreal provides shell completion to make working with commands and arguments easier.

## Overview

Shell completion helps you:

- Complete `angreal` commands automatically
- Get template suggestions when using `angreal init`
- Complete task names within Angreal projects
- Complete command arguments

## Installation

Shell completion is automatically installed when you first run Angreal. To manually install:

```bash
# Install for current shell
angreal _completion install

# Generate completion script for specific shell
angreal _completion bash  # For bash
angreal _completion zsh   # For zsh
```

After installation, restart your shell or run:

```bash
# For bash
source ~/.bashrc

# For zsh
source ~/.zshrc
```

## Supported Shells

- Bash
- Zsh

## Usage Examples

### Command Completion

```bash
# Type 'angreal ' and press TAB to see available commands
angreal [TAB]
init    run     test    ...

# Type 'angreal i' and press TAB to complete to 'init'
angreal i[TAB]
```

### Template Completion

```bash
# Type 'angreal init ' and press TAB to see available templates
angreal init [TAB]
python-cli    django-api    rust-project  ...

# Type 'angreal init p' and press TAB to complete to 'python-cli'
angreal init p[TAB]
```

### Task Completion

```bash
# Inside an Angreal project, type 'angreal ' and press TAB to see available tasks
angreal [TAB]
build    test     deploy   ...

# Type 'angreal t' and press TAB to complete to 'test'
angreal t[TAB]
```

## Related Documentation

- [Create Tasks](/angreal/how-to-guides/create-a-task) - How to create tasks with completion support
- [Create Templates](/angreal/how-to-guides/create-templates) - How to create templates with completion support
