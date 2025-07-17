---
id: audit-python-deps-create-map
level: task
title: "Audit Python Deps Create Map"
created_at: 2025-07-11T16:01:53.227572+00:00
updated_at: 2025-07-15T19:48:12.023631+00:00
parent: remove-raw-python-dependencies
blocked_by: []
archived: true

tags:
  - "#task"
  - "#phase/completed"


exit_criteria_met: false
---

# Audit Python Deps Create Map

## Parent Initiative

[[remove-raw-python-dependencies]]

## Objective

Audit all Python files in the /python directory to understand current functionality and create a comprehensive conversion map for PyO3 migration.

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

## Acceptance Criteria

− [x] Document all Python files and their purposes (Documented all 4 Python file groups with their functions)
− [x] List all decorators and their signatures (Listed all 5 decorators with full signatures in findings)
− [x] Map all classes and their methods (Mapped VirtualEnv class with all methods, GitException and Git classes documented)
− [x] Identify import dependencies (All Rust imports documented for each file, including packaging.specifiers dependency)
− [x] Create conversion priority order (Migration strategy section defines order: PyO3 config, decorators, VirtualEnv, Docker, Git removal)
− [x] Note any Rust equivalents already in use (Key observations section documents all existing Rust implementations including PyO3 modules)

## Implementation Notes

Key files to audit:
- `/angreal/__init__.py` - Core decorators
- `/angreal/integrations/venv.py` - Virtual environment management
- `/angreal/integrations/git.py` - Git operations (to be replaced with git2)
- `/angreal/integrations/docker/*.py` - Docker re-exports

This audit will inform the PyO3 conversion strategy and ensure nothing is missed.

## Status Updates

## Findings

### Overview
This document maps all Python code in the `/python` directory and their Rust equivalents.

### Python Files and Their Functions

#### 1. `/python/angreal/__init__.py`
Core decorators and functions:
- `required_version(specifier: str)` - Validates angreal version using packaging.specifiers
- `@group(**kwargs)` - Assigns commands to groups
- `@command(**kwargs)` - Registers functions as angreal commands
- `@argument(**kwargs)` - Registers command arguments
- `command_group(name, about='')` - Creates reusable group decorators
- `main()` - Calls angreal.main() from Rust

**Rust dependencies imported:**
- `angreal.*` (all exports from Rust module)
- Specifically uses: `angreal.Group`, `angreal.Command`, `angreal.Arg`

#### 2. `/python/angreal/integrations/venv.py`
Virtual environment management:
- `@venv_required(path, requirements=None)` - Decorator to wrap functions in venv
- `VirtualEnv` class with methods:
  - `__init__()`, `_create()`, `install_requirements()`, `install()`
  - Properties: `exists`, `python_executable`
  - Static methods: `discover_available_pythons()`, `ensure_python()`, `version()`
  - Context manager: `activate()`, `deactivate()`, `__enter__()`, `__exit__()`

**Rust functions imported:**
- `ensure_uv_installed`, `uv_version`, `create_virtualenv`
- `install_packages`, `install_requirements`
- `discover_pythons`, `install_python`
- `get_venv_activation_info`

#### 3. `/python/angreal/integrations/git.py`
Git operations (TO BE REMOVED - replaced by git2):
- `GitException` class
- `Git` class with dynamic method generation
- `clone()` and `git_clone()` functions

**Rust imports:**
- `from angreal._integrations.git_module import PyGit as _PyGit, git_clone as _git_clone`

#### 4. Docker re-exports in `/python/angreal/integrations/docker/`:
- `__init__.py`: `from angreal._integrations.docker import *`
- `container.py`: `from angreal._integrations.docker.container import Container, Containers`
- `image.py`: `from angreal._integrations.docker.image import Images, Image`
- `network.py`: `from angreal._integrations.docker.network import Network, Networks`
- `volume.py`: `from angreal._integrations.docker.volume import Volumes, Volume`

### Existing Rust Infrastructure

#### PyO3 Modules Already Defined:
1. Main `angreal` module with:
   - UV integration functions (all the venv functions)
   - Task registration system
   - Utils registration
   - Version info

2. `_integrations` submodule containing:
   - `docker` module with all Docker classes
   - `git_module` with PyGit class

#### Key Observations:
1. Most functionality is already implemented in Rust and exposed via PyO3
2. Python decorators are thin wrappers that use Rust objects (Group, Command, Arg)
3. VirtualEnv class uses Rust UV integration functions
4. Docker is already fully in Rust, Python just re-exports
5. Git is already in Rust but wrapped differently in Python

### Migration Strategy:
1. **Configure PyO3** - Already configured, need to add decorator functions
2. **Core Decorators** - Add PyO3 functions for decorators that create/manipulate Rust objects
3. **VirtualEnv** - Convert the class wrapper to PyO3 while keeping same API
4. **Docker** - Simply update import paths, remove Python re-export files
5. **Git** - Remove Python wrapper entirely (separate initiative for git2)

### Dependencies to Note:
- `packaging.specifiers.Specifier` - Need Rust equivalent for version checking
- `functools.wraps` - Need PyO3 equivalent for decorator preservation
- Path handling between Python pathlib and Rust paths

## Status Updates

*Audit completed on 2025-07-12*