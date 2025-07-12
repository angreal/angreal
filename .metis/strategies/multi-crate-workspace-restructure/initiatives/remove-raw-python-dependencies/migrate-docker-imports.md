---
id: migrate-docker-imports
level: task
title: "Migrate Docker Import Structure"
created_at: 2025-07-11T16:09:00.000000+00:00
updated_at: 2025-07-11T16:09:00.000000+00:00
parent: remove-raw-python-dependencies
blocked_by: ["configure-pyo3-project"]
archived: false

tags:
  - "#task"
  - "#phase/todo"

exit_criteria_met: false
---

# Migrate Docker Import Structure

## Acceptance Criteria

- [ ] Identify where Docker imports are re-exported from
- [ ] Update imports to use Rust-based paths
- [ ] Remove Python re-export files
- [ ] Ensure Docker functionality remains accessible
- [ ] Update any code that uses these imports

## Tasks

- Trace angreal._integrations.docker imports
- Update container.py, image.py, network.py, volume.py imports
- Modify import paths throughout the codebase
- Remove Python stub files after migration
- Test that Docker integration still works

## Notes

The Docker files appear to be re-exporting from angreal._integrations which suggests the actual implementation is already in Rust.

**See audit findings in**: `audit-python-deps-create-map.md` - Confirms Docker is already fully implemented in Rust, just needs import path updates.