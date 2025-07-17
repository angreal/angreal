---
id: multi-crate-workspace-restructure
level: strategy
title: "Multi-Crate Workspace Restructure"
created_at: 2025-07-11T15:29:43.762964+00:00
updated_at: 2025-07-16T01:01:22.844171+00:00
parent:
blocked_by: []
archived: false

tags:
  - "#strategy"
  - "#phase/completed"


exit_criteria_met: false
risk_level: medium
stakeholders: []
---

# Multi-Crate Workspace Restructure Strategy

## Problem Statement

Angreal currently exists as a monolithic Rust project with Python dependencies that complicate the build process and limit portability. The tight coupling between components makes it difficult to extend functionality and reuse parts of the system independently.

By restructuring as a multi-crate workspace, we can achieve better separation of concerns, enable the development of an MCP server for angreal projects, and eliminate Python dependencies to create a pure Rust solution. This will improve maintainability, performance, and enable new integration possibilities.

## Success Metrics

- Successfully convert angreal to a Cargo workspace structure with 1 initial crate
- Complete removal of the /Users/dstorey/Desktop/colliery/angreal/python directory
- Maintain 100% functionality - all existing angreal commands and features continue to work
- All tests pass without modification
- No breaking changes to the user interface or angreal project format

## Solution Approach

1. **Workspace Setup**: Create a Cargo workspace configuration with the existing crate as the first member
2. **Python Dependency Analysis**: Identify all Python functionality currently used and create Rust replacements
3. **Rust Implementation**: Port Python utilities to pure Rust implementations
4. **Testing**: Ensure all existing tests pass with the new Rust-only implementation
5. **Cleanup**: Remove the Python directory once all functionality is migrated

## Scope

**In Scope:**
- Converting the project to a Cargo workspace structure
- Creating the initial workspace with the existing angreal crate
- Identifying and replacing all Python dependencies with Rust equivalents
- Removing the python directory entirely
- Ensuring all existing functionality continues to work

**Out of Scope:**
- Adding new features or functionality
- Creating additional crates beyond the initial workspace setup
- Implementing the MCP server (future phase)
- Changing the public API or CLI interface
- Performance optimizations beyond what naturally comes from removing Python

## Risks & Unknowns

- **Python Feature Discovery**: May discover undocumented Python dependencies during migration
- **Workspace Configuration**: Initial workspace setup might affect build processes
- **Rust Equivalents**: Some Python functionality may require complex Rust implementations
- **Testing Coverage**: Need to ensure comprehensive test coverage for replaced functionality
- **Build System Changes**: Workspace structure may require updates to CI/CD pipelines

## Implementation Dependencies

1. **Analysis Phase**: Audit all Python code to understand functionality that needs replacement
2. **Workspace Creation**: Set up Cargo workspace with proper configuration
3. **Rust Migration**: Implement Rust replacements for Python utilities
4. **Integration Testing**: Verify all commands work without Python
5. **Python Removal**: Delete python directory after confirming no functionality loss

Critical path: Python analysis → Rust implementation → Testing → Removal

## Change Log

###  Initial Strategy
- **Change**: Created initial strategy document
- **Rationale**: Need to establish workspace structure for future MCP server and eliminate Python dependencies
- **Impact**: Baseline established for strategic direction
