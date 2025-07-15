---
id: rust-native-git-integration
level: initiative
title: "Rust Native Git Integration"
created_at: 2025-07-12T13:36:02.619044+00:00
updated_at: 2025-07-12T13:45:38.516065+00:00
parent: multi-crate-workspace-restructure
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
---

# Rust Native Git Integration Initiative

## Context

Currently, the Python git integration uses subprocess calls to execute git commands. Since we already have git2 in our dependency chain, we should leverage it to replace the Python git class with native Rust implementation. This will improve performance, error handling, and remove the subprocess overhead.

## Goals & Non-Goals

**Goals:**
- Replace subprocess-based git operations with git2 library usage
- Improve error handling and type safety for git operations
- Eliminate Python git class entirely
- Maintain or improve performance
- Provide better integration with angreal's error handling

**Non-Goals:**
- Adding new git functionality beyond current implementation
- Creating a full git client
- Implementing advanced git operations not currently used

## Detailed Design

1. **Leverage Existing git2**: Use the already included git2 dependency
2. **API Design**: Create Rust API that matches current git integration needs
3. **Core Operations**: Implement essential operations using git2:
   - Clone repositories
   - Basic operations: init, add, commit, push, pull
   - Branch operations
   - Status and diff
4. **Error Handling**: Proper Result types with git2 error conversion
5. **Integration**: Replace Python git class usage throughout codebase

## Alternatives Considered

- **Keep subprocess calls**: Maintains external dependency and poor error handling
- **Switch to gitoxide**: We already have git2, no need to add another dependency
- **Minimal wrapper around git CLI**: Still requires git installation

## Implementation Plan

1. **Analysis**: Document current Python git class usage patterns
2. **Design**: Create Rust API using git2 matching current needs
3. **Implementation**: Build core git operations with git2
4. **Migration**: Replace Python git usage
5. **Testing**: Comprehensive testing of all git operations

## Testing Strategy

- All current git operations work identically using git2
- Performance benchmarks show improvement
- No subprocess calls for git operations
- Error messages are clear and actionable
- Integration tests cover all use cases
