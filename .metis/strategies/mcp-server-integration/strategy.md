---
id: mcp-server-integration
level: strategy
title: "MCP Server Integration"
created_at: 2025-07-16T00:56:46.567846+00:00
updated_at: 2025-07-16T01:17:56.996+00:00
parent:
blocked_by: []
archived: false

tags:
  - "#strategy"
  - "#phase/ready"


exit_criteria_met: false
risk_level: medium
stakeholders: []
---

# MCP Server Integration Strategy

## Problem Statement

Angreal currently has a suboptimal JSON interchange system for programmatic access that doesn't work well in practice. This creates friction for AI assistants and other tools trying to interact with angreal projects programmatically.

By implementing an MCP server that directly exposes angreal commands as MCP tools, we can replace the current janky JSON system with a standardized, robust interface. This will enable seamless integration with Claude Code and other MCP-compatible tools, providing direct access to angreal functionality without the current interchange layer limitations.

## Success Metrics

- MCP server successfully exposes angreal project discovery, command execution, and file management capabilities
- AI assistants can automatically detect angreal projects and suggest relevant commands
- Integration with Claude Code allows seamless angreal project management without manual command entry
- MCP server handles at least 95% of common angreal use cases (init, task execution, template management)
- Documentation and examples enable easy adoption by other MCP-compatible tools
- Server maintains backward compatibility with existing angreal CLI functionality

## Solution Approach

1. **MCP Server Foundation**: Use rust-mcp-sdk as the foundation for implementing the MCP protocol, providing a robust and standardized base for the server implementation.

2. **Dynamic Tool Discovery**: Implement dynamic MCP tool creation at server startup based on the angreal project context, automatically exposing available commands as MCP tools without manual configuration.

3. **Direct Command Mapping**: Map each discovered angreal command directly to an MCP tool, eliminating the current JSON interchange layer and providing native MCP access to all angreal functionality.

4. **Project-Aware Startup**: Server initializes within an angreal project context, discovering and exposing only the commands and tools relevant to that specific project.

5. **Resource Exposure**: Provide MCP resources for project metadata, command documentation, and configuration files to give AI assistants full context.

## Scope

**In Scope:**
- Creating a new `angreal-mcp` crate within the workspace using rust-mcp-sdk
- Implementing dynamic tool discovery and creation at server startup
- Direct mapping of angreal commands to MCP tools without intermediate JSON layer
- Project-aware server initialization that exposes context-relevant functionality
- Exposing angreal project metadata and configuration through MCP resources
- Basic error handling and logging for MCP server operations

**Out of Scope:**
- Manual tool configuration or static tool definitions
- Modifying existing angreal CLI behavior or public APIs
- Advanced MCP features like notifications or sampling beyond rust-mcp-sdk capabilities
- Support for non-standard MCP protocol extensions
- Integration with specific IDEs beyond MCP protocol compliance
- Backward compatibility with the existing JSON interchange system

## Risks & Unknowns

- **MCP Protocol Evolution**: MCP specification may change during development, requiring updates to our implementation
- **Angreal Library Coupling**: Existing angreal code may be tightly coupled to CLI patterns, requiring refactoring for library use
- **Performance Impact**: MCP server overhead and JSON-RPC communication may introduce latency for complex operations
- **Security Considerations**: Exposing file system and command execution through MCP requires careful permission and validation design
- **Client Compatibility**: Different MCP clients may interpret the protocol differently, requiring extensive testing
- **Workspace Integration**: New crate structure may require updates to build processes and dependency management

## Implementation Dependencies

1. **rust-mcp-sdk Integration**: Evaluate and integrate rust-mcp-sdk into the angreal workspace, understanding its capabilities and constraints
2. **Dynamic Discovery Architecture**: Design the command discovery mechanism that will scan angreal projects and create MCP tools at runtime
3. **Command-to-Tool Mapping**: Define how angreal commands, their parameters, and outputs map to MCP tool schemas
4. **Security Model Definition**: Establish security boundaries and permission model for MCP tool execution
5. **Integration Testing Framework**: Set up testing infrastructure for MCP protocol compliance and client compatibility

Critical path: rust-mcp-sdk integration → Dynamic discovery design → Command mapping implementation → Security validation → Testing

All prerequisites are completed, so implementation can begin immediately with rust-mcp-sdk evaluation and integration.

## Change Log

###  Initial Strategy
- **Change**: Created initial strategy document
- **Rationale**: Need to establish MCP server integration strategy to enable AI-assisted development workflows with angreal projects
- **Impact**: Baseline established for strategic direction and implementation planning
