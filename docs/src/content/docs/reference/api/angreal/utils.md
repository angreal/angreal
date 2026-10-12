---
title: "angreal.utils"
description: "Filesystem utilities"
sidebar:
  order: 11
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


Filesystem utilities

## Functions

### <span id="fn.render_directory"></span>`render_directory`

```python
def render_directory(
    src:  str,
    dst:  str,
    force: bool,
    context: Option < & Bound < '_ , PyDict > >,
) -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::utils::render_directory](../../rust/angreal/utils/#fn.render_directory)

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

    let x = render_dir(src, ctx, dst, force, false);
    Ok(pythonize_this!(x))
    // src: &Path, context: Context, dst: &Path, force: bool
}
```

</details>



### <span id="fn.generate_context"></span>`generate_context`

```python
def generate_context(path:  str, take_input: bool) -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::utils::generate_context](../../rust/angreal/utils/#fn.generate_context)

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



### <span id="fn.get_root"></span>`get_root`

```python
def get_root() -> PyResult < String >
```

> **Rust Implementation**: [angreal::utils::get_root](../../rust/angreal/utils/#fn.get_root)

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



### <span id="fn.render_template"></span>`render_template`

```python
def render_template(
    template:  str,
    context:  Bound < '_ , PyDict >,
) -> PyResult < String >
```

> **Rust Implementation**: [angreal::utils::render_template](../../rust/angreal/utils/#fn.render_template)

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



### <span id="fn.get_context"></span>`get_context`

```python
def get_context() -> PyResult < Py < PyAny > >
```

> **Rust Implementation**: [angreal::utils::get_context](../../rust/angreal/utils/#fn.get_context)

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
