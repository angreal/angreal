---
title: "angreal::python_bindings::integrations::docker"
description: "Docker integration bindings"
editUrl: false
sidebar:
  order: 32
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Docker integration bindings

## Functions

### <span id="fn.docker_integration"></span><code style="font-size: 0.85em">fn docker_​integration</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn docker_integration(
    _py: Python) -> PyResult<(,
    m: &Bound<'_, PyModule>) -> PyResult<(,
) -> PyResult<()>
```

Docker integration module

This will be exposed as angreal.integrations.docker in Python

<details>
<summary>Source</summary>

```rust
pub fn docker_integration(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<docker_pyo3::Pyo3Docker>()?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::image::image))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::container::container))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::network::network))?;
    m.add_wrapped(wrap_pymodule!(docker_pyo3::volume::volume))?;

    // Add Docker Compose functionality
    m.add_class::<compose::PyDockerCompose>()?;
    m.add_class::<compose::PyComposeOutput>()?;
    m.add_function(wrap_pyfunction!(compose::compose, m)?)?;

    Ok(())
}
```

</details>
