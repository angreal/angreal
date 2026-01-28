---
id: integrate-flox-for-environment-and
level: initiative
title: "Integrate Flox for Environment and Services Management"
short_code: "ANG-I-0008"
created_at: 2026-01-12T22:30:03.167931+00:00
updated_at: 2026-01-13T01:33:13.484821+00:00
parent: ANG-V-0001
blocked_by: []
archived: false

tags:
  - "#initiative"
  - "#phase/active"


exit_criteria_met: false
estimated_complexity: M
strategy_id: NULL
initiative_id: integrate-flox-for-environment-and
---

# Integrate Flox for Environment and Services Management Initiative

## Context

Angreal currently provides integrations for managing development environments and services through:
- **VirtualEnv** (`angreal.integrations.venv`) - Python virtual environment management via uv
- **Docker/Docker Compose** (`angreal.integrations.docker`) - Container-based services orchestration

Flox is a modern development environment manager built on Nix that offers a compelling alternative approach. It provides declarative, reproducible environments with native support for services orchestration, without the overhead of containers or the language-specificity of virtual environments.

Integrating Flox would give angreal users another option for:
- Cross-language development environments (not just Python)
- Lightweight services orchestration (databases, caches, etc.) without Docker
- Reproducible builds with Nix guarantees
- Seamless environment activation that works alongside existing tools

## Goals & Non-Goals

**Goals:**
- Create `angreal.integrations.flox` module following existing integration patterns
- Support Flox environment initialization, activation, and package installation
- Support Flox services management (start, stop, status)
- Provide task decorators/utilities for Flox-based workflows
- Document when to choose Flox vs VirtualEnv vs Docker

**Non-Goals:**
- Replacing existing VirtualEnv or Docker integrations
- Deep Nix/Nixpkgs integration beyond what Flox exposes
- Supporting Flox's remote environment features initially

## Detailed Design

### Integration Approach: CLI Wrapper

Flox is written in Rust (74.5% of codebase) with internal crates (`flox-rust-sdk`, `flox-core`, `flox-activations`), but these are **not published to crates.io**. They use version `0.0.0` and are workspace-internal only.

**We wrap the Flox CLI** - same pattern as our UV integration for VirtualEnv:
- Shell out to `flox` binary for all operations
- Parse CLI output (text and JSON where available)
- Avoids tight coupling to unstable internal APIs

**Key CLI commands we'll use:**
| Command | Purpose | Output |
|---------|---------|--------|
| `flox --version` | Availability check | Version string |
| `flox activate --print-script` | Get env modifications | Shell script |
| `flox activate -- CMD` | Run command in env | Command output |
| `flox services start [svc...]` | Start services | Status |
| `flox services stop` | Stop services | Status |
| `flox services status` | Get service PIDs | `NAME STATUS PID` |
| `flox services logs [--follow]` | View logs | Log output |

**Environment variables set by Flox activation:**
- `$FLOX_ENV` - Path to built environment
- `$FLOX_ENV_PROJECT` - Project directory
- `$_FLOX_ACTIVE_ENVIRONMENTS` - JSON array of active envs
- `$FLOX_ACTIVATE_START_SERVICES` - "true"/"false"
- Modified `$PATH` with Flox packages prepended

### Module Structure

```
crates/angreal/src/python_bindings/integrations/flox.rs
crates/angreal/src/integrations/flox/mod.rs
```

### Core API

**Flox class** (unified environment + services):
- `__init__(path)` - Reference to directory containing manifest.toml
- `exists` - Property: Check if manifest.toml exists
- `activate()` - Apply Flox env to current Python process (modifies os.environ, PATH)
- `deactivate()` - Restore original environment
- `services` - Property: Returns FloxServices instance
- Context manager support (`__enter__`/`__exit__`)

**FloxServices class**:
- `start(*services)` - Start services, returns FloxServiceHandle
- `stop()` - Stop all services
- `status()` - Get status of all services
- `logs(service)` - Get service logs

**FloxServiceHandle class** (for long-running sessions):
- `save(path=".flox-services.json")` - Persist handle for later cleanup
- `stop()` - Stop services using this handle
- `load(path)` - Class method to restore handle from file

**flox_required decorator**:
- `@flox_required(path, services=None)` - Activate env, optionally start services, run function, cleanup

### Activation Pattern (mirroring VirtualEnv)

```rust
// Pseudocode for activate()
fn activate(&mut self) -> PyResult<()> {
    // 1. Run `flox activate --print-script` or parse env
    // 2. Save original os.environ state
    // 3. Apply Flox environment modifications to os.environ
    // 4. Mark as activated
}
```

### Usage Examples

```python
from angreal.integrations.flox import Flox, flox_required

# ── Task-scoped (decorator) ──────────────────────────────
@angreal.command(name="test")
@flox_required(".", services=["postgres"])
def run_tests():
    subprocess.run(["pytest"])  # runs in flox env with postgres

# ── Context manager ──────────────────────────────────────
@angreal.command(name="integration-test")
def integration_test():
    with Flox(".", services=["postgres", "redis"]) as flox:
        subprocess.run(["pytest", "tests/integration"])

# ── Long-running (explicit control) ──────────────────────
@angreal.command(name="dev-up")
def dev_up():
    flox = Flox(".")
    flox.activate()
    handle = flox.services.start()
    handle.save()  # persist for later stop
    print("Dev environment ready")

@angreal.command(name="dev-down")
def dev_down():
    from angreal.integrations.flox import FloxServiceHandle
    handle = FloxServiceHandle.load()
    handle.stop()
```

## Alternatives Considered

1. **Direct Nix integration** - Too complex, Flox provides the right abstraction layer
2. **Devbox integration** - Similar to Flox but less mature services support
3. **Extending Docker integration** - Doesn't address the "lighter than containers" use case
4. **Using Flox's Rust crates directly** - Investigated but not viable; `flox-rust-sdk`, `flox-core` etc. are internal workspace crates not published to crates.io (version 0.0.0, workspace-only deps). CLI wrapper is the supported integration path.

## Implementation Plan

1. ~~**Research Flox CLI**~~ ✓ Complete - CLI commands and env vars documented above
2. **Core Rust integration** (`crates/angreal/src/integrations/flox/mod.rs`)
   - Flox binary detection (`flox --version`)
   - Parse `flox activate --print-script` output to extract env var changes
   - Service management via `flox services start/stop/status`
   - Parse service status output for PIDs
3. **Python bindings** (`crates/angreal/src/python_bindings/integrations/flox.rs`)
   - `Flox` class with activate/deactivate (mirrors VirtualEnv pattern)
   - `FloxServices` class for service lifecycle
   - `FloxServiceHandle` for persistent service references (JSON serialization)
   - `flox_required` decorator
4. **Testing**
   - Unit tests with mocked Flox CLI output
   - Integration tests (skip if Flox not installed)
   - Functional tests through angreal CLI
5. **Documentation** - Usage guide, comparison with VirtualEnv/Docker
