---
title: "angreal::python_bindings"
description: "Python bindings for angreal"
sidebar:
  order: 33
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Python bindings for angreal

This module provides Python bindings for angreal functionality, organized
into logical submodules. It also provides a public API for other Rust
projects to initialize angreal's Python bindings.

## Functions

### <span id="fn.initialize"></span>`fn initialize`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn initialize () -> PyResult < () >
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



### <span id="fn.create_angreal_module"></span>`fn create_angreal_module`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">pub(crate)</span>


```rust
fn create_angreal_module (py : Python < '_ >) -> PyResult < Bound < '_ , PyModule > >
```

Create the main angreal Python module

This assembles all the submodules and functions into the main angreal module
that Python will import.

<details>
<summary>Source</summary>

```rust
pub(crate) fn create_angreal_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let m = PyModule::new(py, "angreal")?;

    // Add version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Register core functions (these will be moved from lib.rs)
    // TODO: Move these from lib.rs
    // m.add_function(wrap_pyfunction!(main, m)?)?;
    // m.add_function(wrap_pyfunction!(ensure_uv_installed, m)?)?;
    // ... etc

    // Register decorator functions
    decorators::register_decorators(py, &m)?;

    // Register submodules
    m.add_wrapped(wrap_pymodule!(integrations::integrations))?;

    Ok(m)
}
```

</details>
