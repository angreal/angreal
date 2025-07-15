---
id: remove-raw-python-dependencies
level: initiative
title: "Remove Raw Python Dependencies"
created_at: 2025-07-11T15:44:02.411971+00:00
updated_at: 2025-07-15T19:48:31.722251+00:00
parent: multi-crate-workspace-restructure
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/completed"


exit_criteria_met: false
estimated_complexity: L
---

# Remove Raw Python Dependencies Initiative

## Context

Angreal currently uses raw Python code and decorators in the `/python` directory. This creates build complexity, requires Python runtime dependencies, and limits portability. By converting all raw Python to PyO3 bindings, we can compile Python functionality into the Rust binary while maintaining the same interface.

## Goals & Non-Goals

**Goals:**
- Convert all Python decorators to PyO3 bindings
- Replace raw Python files with Rust implementations using PyO3
- Maintain exact same functionality and API
- Enable compilation of Python code into the Rust binary

**Non-Goals:**
- Changing the existing decorator API
- Adding new functionality
- Performance optimization beyond what naturally comes from compilation
- Workspace restructuring (separate initiative)

## Detailed Design

1. **Audit Phase**: Catalog all Python files and their functionality
2. **PyO3 Setup**: Configure PyO3 in the project with proper Python version targeting
3. **Decorator Conversion**: Convert each Python decorator to PyO3 bindings
   - Maintain exact same interface
   - Ensure all parameters and return types match
4. **Python Module Conversion**: Convert utility modules to Rust+PyO3
   - alternatively early deletion from lack of consumption is also acceptable
5. **Integration**: Update the main application to use PyO3 modules instead of raw Python
6. **Validation**: Comprehensive testing to ensure no functionality loss

## Alternatives Considered

- The current implementation IS the alternative, we don't want to carry the decision debt any longer.

## Implementation Plan

1. **Discovery**: Audit all Python code and create conversion map
2. **Setup**: Configure PyO3 and create initial binding structure
3. **Conversion**: Systematically convert each Python module
4. **Integration**: Update main application to use PyO3 modules
5. **Testing**: Validate all functionality works as before
6. **Cleanup**: Remove raw Python files once confirmed working

## Testing Strategy

- All existing tests must pass without modification
- Manual testing of all decorator functionality
- Build verification on multiple platforms
