# Angreal Project Configuration

## Project Overview

Angreal is a hybrid Rust/Python project that provides task automation and project templating capabilities. It combines Rust's performance and reliability with Python's flexibility for defining custom tasks.

## Tech Stack

### Core Technologies
- **Rust** (2021 edition) - Core binary and performance-critical operations
- **Python** (3.8+) - Task definitions and user-facing API
- **PyO3** (0.18) - Python-Rust bindings

### Key Dependencies
- **Rust Libraries**:
  - Clap 3 - CLI argument parsing
  - Tokio - Async runtime
  - Axum - MCP server framework (angreal-mcp)
  - Git2 - Git operations
  - Tera - Template engine
  - Diesel/SQLite - Local data storage

- **Python Build**:
  - Maturin - Rust/Python packaging
  - PyTest - Testing framework

### Documentation
- **Hugo** - Static site generator
- **Geekdoc Theme** - Documentation theme

## Project Structure

```
angreal/
├── crates/
│   ├── angreal/          # Main Rust library with Python bindings
│   │   ├── src/
│   │   │   ├── python_bindings/  # PyO3 bindings
│   │   │   ├── integrations/     # Git, UV, Docker integrations
│   │   │   ├── builder/          # Command tree builder
│   │   │   ├── completion/       # Shell completions
│   │   │   └── validation/       # Input validation
│   │   └── tests/
│   └── angreal-mcp/      # MCP server for AI integrations
│       └── src/
│           ├── server.rs
│           └── tools/
├── docs/                 # Hugo documentation
│   ├── content/
│   │   ├── tutorials/
│   │   ├── how-to-guides/
│   │   ├── reference/
│   │   └── explanation/
│   └── themes/
├── py_tests/            # Python test suite
└── .angreal/           # Project-specific tasks
```

## Agent Assignments

### By Component

#### Core Rust Development
- `/crates/angreal/src/` → @rust-service-architect
  - Core library architecture
  - Command building and execution
  - Integration implementations

- `/crates/angreal/src/python_bindings/` → @pyo3-bridge-builder
  - Python-Rust interface design
  - Type conversions and error handling
  - API consistency between languages

- `/crates/angreal-mcp/` → @rust-service-architect
  - MCP server implementation
  - Tool discovery and execution
  - Async request handling

#### Python API and Tasks
- `/.angreal/` → @python-cli-developer
  - Task definitions using decorators
  - Command groups and arguments
  - Integration usage examples

- `/py_tests/` → @test-strategy-agent + @python-cli-developer
  - Python test suite
  - Integration testing
  - Functional tests

#### Documentation
- `/docs/` → @documentation-curator
  - User guides and tutorials
  - API reference documentation
  - Architecture explanations

#### Build and CI/CD
- Build configurations → @integration-orchestrator
  - Cargo workspace management
  - Maturin build configuration
  - CI/CD workflows

### By Task Type

#### Feature Development
1. **New Commands/Tasks** → @python-cli-developer
   - Design command interface
   - Implement using angreal decorators
   - Add to appropriate command groups

2. **Core Features** → @technical-lead-orchestrator
   - Coordinate between Rust and Python components
   - Design cross-language interfaces
   - Ensure consistent behavior

3. **Performance Optimization** → @rust-performance-engineer
   - Profile Rust components
   - Optimize PyO3 boundary crossings
   - Improve async operations

#### Architecture and Design
- **System Design** → @design-first-architect
  - New integration planning
  - API design proposals
  - Component interaction patterns

- **Code Analysis** → @project-analyst
  - Understanding existing patterns
  - Finding implementation locations
  - Impact assessment

#### Quality and Testing
- **Test Strategy** → @test-strategy-agent
  - Test coverage planning
  - Integration test design
  - Cross-language testing approach

- **Code Review** → @code-reviewer
  - Rust idiom compliance
  - Python best practices
  - Cross-language consistency

## Routing Rules

### Quick Decision Tree

```
Is it a new feature?
├─ Yes → @technical-lead-orchestrator (coordinates implementation)
└─ No → Continue

Is it documentation?
├─ Yes → @documentation-curator
└─ No → Continue

Is it a bug fix?
├─ Yes → @project-analyst (find cause) → Language specialist (fix)
└─ No → Continue

Is it Python-Rust integration?
├─ Yes → @pyo3-bridge-builder
└─ No → Continue

Is it performance related?
├─ Yes → @rust-performance-engineer
└─ No → Language specialist based on file location
```

### Specific Patterns

1. **Adding a new angreal command**:
   - @python-cli-developer creates task in `.angreal/`
   - @documentation-curator updates docs
   - @test-strategy-agent designs tests

2. **Improving Python-Rust interface**:
   - @pyo3-bridge-builder designs interface
   - @rust-service-architect implements Rust side
   - @python-cli-developer ensures Python API consistency

3. **MCP server enhancements**:
   - @rust-service-architect implements features
   - @integration-orchestrator updates tool discovery
   - @documentation-curator documents new tools

4. **Performance issues**:
   - @project-analyst identifies bottlenecks
   - @rust-performance-engineer optimizes Rust code
   - @pyo3-bridge-builder optimizes bindings if needed

## Workflows

### Feature Implementation Flow

```mermaid
graph LR
    A[User Request] --> B[@technical-lead-orchestrator]
    B --> C{Component Type}
    C -->|Rust Core| D[@rust-service-architect]
    C -->|Python API| E[@python-cli-developer]
    C -->|Bindings| F[@pyo3-bridge-builder]
    D --> G[@test-strategy-agent]
    E --> G
    F --> G
    G --> H[@documentation-curator]
    H --> I[@code-reviewer]
```

### Bug Fix Flow

```mermaid
graph LR
    A[Bug Report] --> B[@project-analyst]
    B -->|Find Root Cause| C{Language}
    C -->|Rust| D[@rust-service-architect]
    C -->|Python| E[@python-cli-developer]
    C -->|Integration| F[@pyo3-bridge-builder]
    D --> G[@test-strategy-agent]
    E --> G
    F --> G
    G --> H[@code-reviewer]
```

## Tool Integration

### Metis Project Management
- Use @agile-delivery-coach for:
  - Creating work items (Vision → Strategy → Initiative → Task)
  - Tracking feature progress
  - Managing dependencies between Rust and Python work

### Angreal Task Automation
- MCP integration available: @integration-orchestrator
- Dynamic tool discovery from `.angreal/` tasks
- Use `angreal-mcp` for AI-assisted task execution

## Project-Specific Conventions

### Code Style
- Rust: Follow Rust idioms, use clippy
- Python: Follow PEP 8, use type hints where beneficial
- Cross-language: Maintain consistent naming at boundaries

### Testing Requirements
- All Rust code must have unit tests
- Python tasks should have integration tests
- Cross-language features need both Rust and Python tests

### Documentation Standards
- All public APIs must be documented
- Examples required for complex features
- Architecture decisions documented in `/docs/explanation/`

### Git Workflow
- Feature branches for all changes
- Intermediate commits encouraged
- No references to Claude/Anthropic in commits

## Common Tasks

### Running Tests
```bash
# All tests
angreal test all

# Specific test suites
angreal test python
angreal test rust
angreal test completion
```

### Building Documentation
```bash
angreal docs build
angreal docs preview
```

### Development Setup
```bash
angreal dev check-deps
```

## Key Integration Points

1. **Python-Rust Bridge**: All Python functionality goes through PyO3 bindings in `/crates/angreal/src/python_bindings/`

2. **Task Discovery**: Tasks are discovered from `.angreal/task_*.py` files and integrated into the CLI

3. **MCP Tools**: The MCP server dynamically creates tools from discovered angreal tasks

4. **Template System**: Tera templates with Jinja2-like syntax for project scaffolding

5. **Shell Completion**: Dynamic completion based on available tasks and arguments
