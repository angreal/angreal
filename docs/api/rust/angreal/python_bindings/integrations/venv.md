# venv


Virtual environment integration submodule

This module provides the venv submodule for angreal.integrations.venv

## Functions

### `fn venv`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn venv (_py : Python , m : & Bound < '_ , PyModule >) -> PyResult < () >
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
