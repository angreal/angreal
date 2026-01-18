# utils


Filesystem utilities

## Functions

### `render_directory`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">render_directory</span>(src:  str, dst:  str, force: bool, context: Option &lt; &amp; Bound &lt; &#x27;_ , PyDict &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::utils::render_directory](../rust/angreal/utils.md#fn-render_directory)

<details>
<summary>Source</summary>

```python
pub fn render_directory(
    src: &str,
    dst: &str,
    force: bool,
    context: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>> {
    let mut ctx = Context::new();
    let src = Path::new(src);
    let dst = Path::new(dst);

    if let Some(context) = context {
        for key in context.keys() {
            if let Ok(Some(value)) = context.get_item(&key) {
                let v = value.to_string();
                let k = key.to_string();
                ctx.insert(&k, &v);
            }
        }
    }

    let x = render_dir(src, ctx, dst, force);
    Ok(pythonize_this!(x))
    // src: &Path, context: Context, dst: &Path, force: bool
}
```

</details>



### `generate_context`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">generate_context</span>(path:  str, take_input: bool) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::utils::generate_context](../rust/angreal/utils.md#fn-generate_context)

Generate a templating context from a toml file.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `path` | ` str` |  |
| `take_input` | `bool` |  |


**Examples:**

```python
import angreal
angreal_root = angreal.generate_context('path/to/angreal.toml',take_input=False)
```

<details>
<summary>Source</summary>

```python
fn generate_context(path: &str, take_input: bool) -> PyResult<Py<PyAny>> {
    let toml_path = Path::new(path).to_path_buf();
    let ctx = repl_context_from_toml(toml_path, take_input);
    let map = context_to_map(ctx);
    Ok(pythonize_this!(map))
}
```

</details>



### `get_root`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">get_root</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::utils::get_root](../rust/angreal/utils.md#fn-get_root)

Get the root path of a current angreal project.

The root is the actual location of the .angreal file that houses task files

**Examples:**

```python
import angreal
angreal_root = angreal.get_root()
```

<details>
<summary>Source</summary>

```python
fn get_root() -> PyResult<String> {
    match is_angreal_project() {
        Ok(angreal_root) => Ok(String::from(angreal_root.to_string_lossy())),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            e.to_string(),
        )),
    }
}
```

</details>



### `render_template`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">render_template</span>(template:  str, context:  Bound &lt; &#x27;_ , PyDict &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; String &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::utils::render_template](../rust/angreal/utils.md#fn-render_template)

<details>
<summary>Source</summary>

```python
fn render_template(template: &str, context: &Bound<'_, PyDict>) -> PyResult<String> {
    let mut tera = Tera::default();
    let mut ctx = tera::Context::new();
    tera.add_raw_template("template", template).unwrap();

    for (key, val) in context.iter() {
        ctx.insert(key.to_string(), &val.to_string());
    }

    Ok(tera.render("template", &ctx).unwrap())
}
```

</details>



### `get_context`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">get_context</span>() -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::utils::get_context](../rust/angreal/utils.md#fn-get_context)

Read the angreal.toml file from the .angreal folder and return it as a dictionary

**Examples:**

```python
import angreal
config = angreal.get_context()
```

<details>
<summary>Source</summary>

```python
fn get_context() -> PyResult<Py<PyAny>> {
    let angreal_root = match is_angreal_project() {
        Ok(root) => root,
        Err(_) => {
            let empty = toml::Table::new();
            return Ok(pythonize_this!(empty));
        }
    };

    let toml_path = angreal_root.join("angreal.toml");

    let file_contents = match fs::read_to_string(&toml_path) {
        Ok(contents) => contents,
        Err(_) => {
            let empty = toml::Table::new();
            return Ok(pythonize_this!(empty));
        }
    };

    let toml_value = match file_contents.parse::<Table>() {
        Ok(value) => value,
        Err(_) => {
            let empty = toml::Table::new();
            return Ok(pythonize_this!(empty));
        }
    };

    Ok(pythonize_this!(toml_value))
}
```

</details>
