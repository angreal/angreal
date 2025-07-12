---
id: convert-core-decorators
level: task
title: "Convert Core Decorators to PyO3"
created_at: 2025-07-11T16:06:00.000000+00:00
updated_at: 2025-07-11T16:06:00.000000+00:00
parent: remove-raw-python-dependencies
blocked_by: ["configure-pyo3-project"]
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
---

# Convert Core Decorators to PyO3

## Acceptance Criteria

- [ ] @required_version decorator converted to PyO3
- [ ] @group decorator converted to PyO3
- [ ] @command decorator converted to PyO3
- [ ] @argument decorator converted to PyO3
- [ ] command_group function converted to PyO3
- [ ] All decorators maintain exact same API
- [ ] All existing tests pass

## Tasks

- Convert required_version decorator with version parsing logic
- Convert group decorator with metadata handling
- Convert command decorator with all kwargs support
- Convert argument decorator with type mapping
- Convert command_group helper function
- Ensure all decorators work with functools.wraps equivalent
- Test decorator chaining and combinations

## Notes

These are the core decorators in angreal/__init__.py that need to be converted to maintain compatibility.