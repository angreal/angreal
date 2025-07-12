---
id: audit-python-deps-create-map
level: task
title: "Audit Python Deps Create Map"
created_at: 2025-07-11T16:01:53.227572+00:00
updated_at: 2025-07-11T16:01:53.227572+00:00
parent: remove-raw-python-dependencies
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"


exit_criteria_met: false
---

# Audit Python Deps Create Map

## Parent Initiative

[[remove-raw-python-dependencies]]

## Objective

Audit all Python files in the /python directory to understand current functionality and create a comprehensive conversion map for PyO3 migration.

## Acceptance Criteria

- [ ] Document all Python files and their purposes
- [ ] List all decorators and their signatures
- [ ] Map all classes and their methods
- [ ] Identify import dependencies
- [ ] Create conversion priority order
- [ ] Note any Rust equivalents already in use

## Implementation Notes

Key files to audit:
- `/angreal/__init__.py` - Core decorators
- `/angreal/integrations/venv.py` - Virtual environment management
- `/angreal/integrations/git.py` - Git operations (to be replaced with git2)
- `/angreal/integrations/docker/*.py` - Docker re-exports

This audit will inform the PyO3 conversion strategy and ensure nothing is missed.

## Status Updates

*To be added during implementation*