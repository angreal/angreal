---
title: "API reference"
description: "The API reference, written by plissken from the source."
editUrl: false
sidebar:
  order: 0
  hidden: true
---

## Python

- [`angreal`](angreal/)
- [`angreal._integrations.git_module`](angreal/_integrations/git_module/)
- [`angreal.compose_integration`](angreal/compose_integration/): Compose integration module
- [`angreal.integrations.docker_integration`](angreal/integrations/docker_integration/): Docker integration module
- [`angreal.integrations.flox`](angreal/integrations/flox/): Create the flox submodule
- [`angreal.integrations.git_integration`](angreal/integrations/git_integration/): Git integration module
- [`angreal.integrations.oci`](angreal/integrations/oci/): OCI integration module, exposed as `angreal.integrations.oci` in Python.
- [`angreal.integrations.venv`](angreal/integrations/venv/): Create the venv submodule

## Rust

- [`angreal`](rust/angreal/): Angreal - project templating and task management
- [`angreal::builder`](rust/angreal/builder/): The angreal app builder
- [`angreal::builder::command_tree`](rust/angreal/builder/command_tree/)
- [`angreal::builder::tree_output`](rust/angreal/builder/tree_output/): Human-readable tree output for angreal commands.
- [`angreal::completion`](rust/angreal/completion/): Shell completion support for Angreal
- [`angreal::completion::bash`](rust/angreal/completion/bash/): Bash completion script generation for Angreal
- [`angreal::completion::templates`](rust/angreal/completion/templates/): Template discovery for shell completion
- [`angreal::completion::zsh`](rust/angreal/completion/zsh/): Zsh completion script generation for Angreal
- [`angreal::error_formatter`](rust/angreal/error_formatter/)
- [`angreal::git`](rust/angreal/git/): Git operations for Angreal
- [`angreal::init`](rust/angreal/init/): The angreal `init` command.
- [`angreal::integrations::docker_compose`](rust/angreal/integrations/docker_compose/): Docker Compose integration using subprocess execution for reliability
- [`angreal::integrations::flox`](rust/angreal/integrations/flox/): Flox development environment integration
- [`angreal::integrations::git`](rust/angreal/integrations/git/)
- [`angreal::integrations::oci`](rust/angreal/integrations/oci/): OCI registry integration.
- [`angreal::integrations::uv`](rust/angreal/integrations/uv/)
- [`angreal::logger`](rust/angreal/logger/): Logging for the core application
- [`angreal::mcp`](rust/angreal/mcp/): Minimal MCP (Model Context Protocol) stdio server.
- [`angreal::py_logger`](rust/angreal/py_logger/): A unified logger to bridge python and rust
- [`angreal::python_bindings`](rust/angreal/python_bindings/): Python bindings for angreal
- [`angreal::python_bindings::decorators`](rust/angreal/python_bindings/decorators/): Python decorator functions
- [`angreal::python_bindings::integrations`](rust/angreal/python_bindings/integrations/): Integration modules for external tools and services
- [`angreal::python_bindings::integrations::compose`](rust/angreal/python_bindings/integrations/compose/): PyO3 bindings for Docker Compose integration
- [`angreal::python_bindings::integrations::docker`](rust/angreal/python_bindings/integrations/docker/): Docker integration bindings
- [`angreal::python_bindings::integrations::flox`](rust/angreal/python_bindings/integrations/flox/): Flox environment integration submodule
- [`angreal::python_bindings::integrations::git`](rust/angreal/python_bindings/integrations/git/): Git integration bindings
- [`angreal::python_bindings::integrations::oci`](rust/angreal/python_bindings/integrations/oci/): OCI registry integration bindings.
- [`angreal::python_bindings::integrations::venv`](rust/angreal/python_bindings/integrations/venv/): Virtual environment integration submodule
- [`angreal::python_bindings::utils`](rust/angreal/python_bindings/utils/): Utility functions for Python bindings
- [`angreal::python_bindings::venv`](rust/angreal/python_bindings/venv/): Virtual environment integration
- [`angreal::task`](rust/angreal/task/): Core structures for describing tasks and arguments
- [`angreal::utils`](rust/angreal/utils/): Filesystem utilities
- [`angreal::validation`](rust/angreal/validation/): Validation utilities for template variables
- [`angreal::validation::allowed_values`](rust/angreal/validation/allowed_values/): Validation for allowed values
- [`angreal::validation::length`](rust/angreal/validation/length/): Validation for string length
- [`angreal::validation::max`](rust/angreal/validation/max/): Validation for maximum value
- [`angreal::validation::min`](rust/angreal/validation/min/): Validation for minimum value
- [`angreal::validation::not_empty`](rust/angreal/validation/not_empty/): Validation for non-empty values
- [`angreal::validation::regex_match`](rust/angreal/validation/regex_match/): Validation for regex pattern matching
- [`angreal::validation::type_check`](rust/angreal/validation/type_check/): Validation for type checking
