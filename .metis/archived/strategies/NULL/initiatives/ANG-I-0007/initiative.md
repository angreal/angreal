---
id: mcp-server-and-installation-testing
level: initiative
title: "MCP Server and Installation Testing"
short_code: "ANG-I-0007"
created_at: 2025-12-31T14:48:16.803282+00:00
updated_at: 2025-12-31T14:48:16.803282+00:00
parent: ANG-V-0001
blocked_by: []
archived: true

tags:
  - "#initiative"
  - "#phase/discovery"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: mcp-server-and-installation-testing
---

# MCP Server and Installation Testing Initiative

## Context

The angreal 2.7.1 release integrated the MCP server directly into the angreal binary as a hidden `angreal mcp` subcommand. During testing, we discovered a regression where the MCP server would panic on startup due to attempting to install a tracing subscriber when one was already installed by the PyO3 runtime.

This exposed a gap in our testing: we have no automated tests for:
1. MCP server functionality (startup, tool discovery, command execution)
2. Installation smoke tests (verifying the package installs and launches correctly)

These regressions can silently break critical functionality that users depend on for AI assistant integration.

## Goals & Non-Goals

**Goals:**
- Add MCP server integration tests that verify server startup and basic functionality
- Add installation smoke tests that verify the package installs correctly and core commands work
- Integrate these tests into CI pipeline to catch regressions before release
- Keep tests fast enough to run on every PR

**Non-Goals:**
- Full MCP protocol compliance testing (rely on rust-mcp-sdk for that)
- Performance benchmarking
- Testing against every possible MCP client
- Testing all angreal commands through MCP (just verify the mechanism works)

## Detailed Design

### 1. MCP Server Tests

Add new test commands to `task_tests.py`:

**Test: MCP Server Startup**
- Spawn `angreal mcp` in a subprocess
- Verify it doesn't panic or exit immediately
- Send MCP `initialize` request via stdin
- Verify valid JSON-RPC response
- Terminate gracefully

**Test: MCP Tool Discovery**
- Start MCP server in angreal project directory
- Send `tools/list` request
- Verify tools are returned matching project's task definitions
- Verify tool descriptions are present

**Test: MCP Tool Execution**
- Start MCP server
- Call a simple tool (e.g., `angreal --help` equivalent)
- Verify successful response with expected output

### 2. Installation Smoke Tests

**Test: Fresh Install**
- Build wheel with maturin
- Install in isolated venv
- Run `angreal --version` - verify output
- Run `angreal --help` - verify no errors
- Run `angreal mcp` briefly - verify no immediate crash

**Test: Upgrade Install**
- Install previous version
- Upgrade to new wheel
- Verify commands still work

### 3. Test Integration

Add to existing test groups:
- `angreal test mcp` - Run MCP-specific tests
- `angreal test smoke` - Run installation smoke tests
- Update `angreal test all` to include these

### 4. CI Integration

Add to GitHub Actions workflow:
- Run smoke tests on each platform after wheel build
- Run MCP tests as part of the test suite

## Alternatives Considered

**Alternative 1: External MCP test harness**
- Could use a dedicated MCP testing tool
- Rejected: Adds external dependency, simple subprocess tests are sufficient for our needs

**Alternative 2: Mock the MCP server**
- Unit test individual handlers with mocks
- Rejected: We want integration tests that catch real issues like the tracing subscriber panic

**Alternative 3: Skip MCP testing, rely on manual testing**
- Rejected: We already missed a regression, automated tests are essential

## Implementation Plan

1. **Add MCP test module** - Create helper functions for MCP server subprocess management
2. **Implement MCP startup test** - Basic server lifecycle test
3. **Implement tool discovery test** - Verify tools are exposed correctly
4. **Implement smoke test commands** - Basic install verification
5. **Update CI workflow** - Run new tests in pipeline
6. **Documentation** - Update test documentation
