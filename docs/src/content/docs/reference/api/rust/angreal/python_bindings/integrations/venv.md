---
title: "angreal::python_bindings::integrations::venv"
description: "Virtual environment integration submodule"
editUrl: false
sidebar:
  order: 36
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Virtual environment integration submodule

This module provides the venv submodule for angreal.integrations.venv

## Functions

### <span id="fn.venv"></span><code style="font-size: 0.85em">fn venv</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn venv(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()>
```

Create the venv submodule

<details>
<summary>Source</summary>

```rust
pub fn venv(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register VirtualEnv and venv_required from the main venv module
    crate::python_bindings::venv::register_venv(_py, m)?;
    Ok(())
}
```

</details>
