# MCP Server Foundation Initiative

## Context

The angreal project needs an MCP server to replace the current ineffective JSON interchange system and provide direct access to angreal commands through the Model Context Protocol. This initiative establishes the foundational infrastructure for the MCP server using rust-mcp-sdk as the base implementation.

The foundation must provide a solid base for dynamic tool discovery and command mapping while integrating cleanly with the existing angreal workspace structure. This work enables AI assistants like Claude Code to natively interact with angreal projects without the limitations of the current JSON system.

## Goals & Non-Goals

**Goals:**
- Create a new `angreal-mcp` crate within the angreal workspace
- Integrate rust-mcp-sdk and establish basic MCP server infrastructure
- Implement core server initialization and project detection capabilities
- Set up proper error handling, logging, and configuration management
- Create foundation for dynamic tool registration and command execution
- Establish testing framework for MCP protocol compliance

**Non-Goals:**
- Implementing specific angreal command mappings (handled by Command-to-Tool Mapping initiative)
- Dynamic tool discovery implementation (handled by Dynamic Tool Discovery initiative)
- Complete MCP server functionality - this is foundation only
- Replacing the JSON interchange system (comes after tool implementation)
- Performance optimization beyond basic functionality

## Detailed Design

1. **Crate Structure**:
   - Create `crates/angreal-mcp/` directory within the workspace
   - Set up `Cargo.toml` with rust-mcp-sdk dependency
   - Establish proper module structure for server, handlers, and utilities
   - ensure that an extensible structure is setup tools in individual modules

2. **MCP Server Infrastructure**:
   - Initialize rust-mcp-sdk server with stdio transport
   - Implement basic MCP protocol handlers (initialize, list_tools, call_tool)
   - Set up JSON-RPC communication layer using rust-mcp-sdk abstractions

3. **Project Detection**:
   - Implement angreal project detection logic (use the library "is angreal project" that exists)
   - Handle cases where server starts outside angreal projects, for now in this event - nothing should happen it should silently run with zero tools.

4. **Configuration & Logging**:
   - Set up structured logging compatible with MCP protocol


## Alternatives Considered

- **Custom MCP Implementation**: Building MCP protocol from scratch would provide complete control but requires significant protocol implementation work and ongoing maintenance. rust-mcp-sdk provides a battle-tested foundation.

- **Python MCP Server**: Using Python with existing MCP libraries would be faster initially but contradicts the goal of removing Python dependencies from angreal and would require maintaining a separate runtime.

- **HTTP Transport**: Using HTTP instead of stdio transport would enable network-based MCP connections but adds complexity and security concerns. Stdio is simpler and matches the expected MCP server pattern for local tools.

- **Monolithic Integration**: Adding MCP functionality directly to the main angreal crate would avoid workspace complexity but violates separation of concerns and makes the MCP server harder to test and maintain independently.

## Implementation Plan

1. **Crate Setup**: Create angreal-mcp crate structure and integrate rust-mcp-sdk dependency
2. **Basic Server**: Implement minimal MCP server that can initialize and respond to basic protocol messages
3. **Project Detection**: Add angreal project detection and validation logic
4. **Error Handling**: Implement comprehensive error types and logging infrastructure
5. **Foundation Interfaces**: Define traits and abstractions for tool registration and execution
6. **Integration Testing**: Set up testing framework and validate MCP protocol compliance
7. **Documentation**: Create developer documentation for foundation interfaces and usage patterns
