---
title: "angreal::python_bindings"
description: "Python bindings for angreal"
editUrl: false
sidebar:
  order: 28
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Python bindings for angreal

This module provides Python bindings for angreal functionality, organized
into logical submodules. It also provides a public API for other Rust
projects to initialize angreal's Python bindings.

## Functions

### <span id="fn.initialize"></span><code style="font-size: 0.85em">fn initialize</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn initialize() -> PyResult<()>
```

Initialize angreal's Python bindings

This function can be called by other Rust projects to set up angreal's
Python bindings in an embedded Python interpreter.

**Examples:**

```rust
use angreal::python_bindings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    python_bindings::initialize()?;
    // Now Python can import angreal
    Ok(())
}
```

<details>
<summary>Source</summary>

```rust
pub fn initialize() -> PyResult<()> {
    Python::attach(|py| {
        let sys = py.import("sys")?;
        let modules_attr = sys.getattr("modules")?;
        let modules = modules_attr.cast::<PyDict>()?;

        // Create and register the main angreal module
        let angreal_module = create_angreal_module(py)?;
        modules.set_item("angreal", &angreal_module)?;

        Ok(())
    })
}
```

</details>
