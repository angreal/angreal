---
id: configure-pyo3-project
level: task
title: "Configure PyO3 in Project"
created_at: 2025-07-11T16:05:00.000000+00:00
updated_at: 2025-07-11T16:05:00.000000+00:00
parent: remove-raw-python-dependencies
blocked_by: []
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
---

# Configure PyO3 in Project

## Acceptance Criteria

- [ ] Create python_bindings module structure
- [ ] Move existing PyO3 code into organized modules
- [ ] Add public initialize() function for library consumers
- [ ] Update lib.rs to use new module structure
- [ ] Verify restructured code compiles successfully
- [ ] Test that Python imports still work

## Tasks

- Create src/python_bindings/ directory structure
- Move PyGit class to python_bindings/integrations/git.rs
- Move Docker setup to python_bindings/integrations/docker.rs
- Create python_bindings/mod.rs with public initialize() function
- Update lib.rs to register modules through new structure
- Create placeholder modules for decorators, venv, utils
- Test compilation and Python import functionality

## Notes

PyO3 is already configured in the project. This task restructures the existing PyO3 code into a modular organization and adds a public API for library consumers.

**See audit findings in**: `audit-python-deps-create-map.md` - Shows existing PyO3 infrastructure that needs reorganizing.

## Proposed Structure

```
src/
├── lib.rs                    # Core Rust API (minimal PyO3)
├── python_bindings/          # Python-specific layer
│   ├── mod.rs               # Public init function + module setup
│   ├── decorators.rs        # Future: @command, @group, @argument bindings
│   ├── venv.rs              # Future: VirtualEnv Python wrapper
│   ├── utils.rs             # Future: required_version, etc.
│   └── integrations/        
│       ├── mod.rs
│       ├── docker.rs        # Move Docker PyO3 bindings here
│       └── git.rs           # Move PyGit class here
```

## Public API Goal

```rust
use angreal::python_bindings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    python_bindings::initialize()?;
    // Now Python can import angreal
    Ok(())
}
```