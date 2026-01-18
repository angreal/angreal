# docker


Docker integration bindings

## Functions

### `fn docker_integration`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn docker_integration (_py : Python , m : & Bound < '_ , PyModule >) -> PyResult < () >
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
