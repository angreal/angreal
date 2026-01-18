# integrations


Integration modules for external tools and services

## Functions

### `fn integrations`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn integrations (py : Python , m : & Bound < '_ , PyModule >) -> PyResult < () >
```

Create the integrations submodule

This will be exposed as angreal.integrations in Python

<details>
<summary>Source</summary>

```rust
pub fn integrations(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(docker::docker_integration))?;

    // Create and register the git submodule
    let git_module = wrap_pymodule!(git::git_integration)(py);
    m.add_submodule(git_module.bind(py))?;

    // Create and register the venv submodule
    let venv_module = wrap_pymodule!(venv::venv)(py);
    m.add_submodule(venv_module.bind(py))?;

    // Create and register the flox submodule
    let flox_module = wrap_pymodule!(flox::flox)(py);
    m.add_submodule(flox_module.bind(py))?;

    // Also register all modules in sys.modules for proper import support
    let sys = py.import("sys")?;
    let modules = sys.getattr("modules")?;
    modules.set_item("angreal.integrations.git", git_module)?;
    modules.set_item("angreal.integrations.venv", venv_module)?;
    modules.set_item("angreal.integrations.flox", flox_module)?;

    Ok(())
}
```

</details>
