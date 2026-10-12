---
title: "angreal::utils"
description: "Filesystem utilities"
editUrl: false
sidebar:
  order: 40
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Filesystem utilities

## Functions

### <span id="fn.context_to_map"></span><code style="font-size: 0.85em">fn context_​to_​map</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn context_to_map(ctx: Context) -> Map<String, Value>
```

<details>
<summary>Source</summary>

```rust
pub fn context_to_map(ctx: Context) -> Map<String, Value> {
    Map::try_from(ctx.into_json().as_object().unwrap().clone()).unwrap()
}
```

</details>

### <span id="fn.repl_context_from_toml"></span><code style="font-size: 0.85em">fn repl_​context_​from_​toml</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn repl_context_from_toml(
    toml_path: PathBuf,
    take_input: bool,
) -> Context
```

<details>
<summary>Source</summary>

```rust
pub fn repl_context_from_toml(toml_path: PathBuf, take_input: bool) -> Context {
    // Extract the sections using our new functions
    let defaults = extract_key_defaults(toml_path.clone()).unwrap();
    let prompts = extract_prompts(toml_path.clone()).unwrap();
    let validations = extract_validation_rules(toml_path).unwrap();

    let mut context = Context::new();

    // Process each key-value pair from defaults
    for (k, v) in defaults.iter() {
        let value = if v.is_str()
            && v.as_str().unwrap().starts_with("{{")
            && v.as_str().unwrap().contains("}}")
        {
            let temp_value = v.clone();
            let rendered_value =
                Tera::one_off(temp_value.as_str().unwrap(), &context, false).unwrap();
            Value::from(rendered_value)
        } else {
            v.clone()
        };

        let input = if take_input {
            // Use the prompt if available, otherwise use the key and value
            let prompt_text = prompts
                .get(k)
                .and_then(|p| p.as_str())
                .map(|p| format!("{} [{value}]", p))
                .unwrap_or_else(|| format!("{k}? [{value}]"));

            // Loop until we get valid input
            let mut valid_input = String::new();
            let mut is_valid = false;

            while !is_valid {
                print!("{}: ", prompt_text);
                valid_input = read!("{}\n");

                // Skip validation if input is empty (using default)
                if valid_input.trim().is_empty() {
                    break;
                }

                // Validate input if we have validation rules
                match crate::validation::validate_input(valid_input.trim(), k, &validations) {
                    Ok(_) => {
                        is_valid = true;
                    }
                    Err(err_msg) => {
                        println!("Invalid input: {}", err_msg);
                        is_valid = false;
                    }
                }
            }

            valid_input
        } else {
            String::new()
        };

        if input.trim().is_empty() | take_input.not() {
            if value.is_str() {
                context.insert(k, &value.as_str().unwrap());
            }
            if value.is_integer() {
                context.insert(k, &value.as_integer().unwrap());
            }
            if value.is_bool() {
                context.insert(k, &value.as_bool().unwrap());
            }
            if value.is_float() {
                context.insert(k, &value.as_float().unwrap());
            }
        } else {
            if value.is_str() {
                context.insert(k, &input.trim());
            }
            if value.is_integer() {
                context.insert(
                    k,
                    &input.trim().parse::<i32>().unwrap_or_else(|_| {
                        debug!(
                            "Could not parse '{}' as integer for key '{}', using default.",
                            input.trim(),
                            k
                        );
                        let i64_val = value.as_integer().unwrap();
                        i64_val.try_into().unwrap_or_else(|_| {
                            debug!("Integer value too large for i32, truncating: {}", i64_val);
                            i64_val as i32
                        })
                    }),
                );
            }
            if value.is_bool() {
                context.insert(k, &input.trim());
            }
            if value.is_float() {
                context.insert(
                    k,
                    &input.trim().parse::<f64>().unwrap_or_else(|_| {
                        debug!(
                            "Could not parse '{}' as float for key '{}', using default.",
                            input.trim(),
                            k
                        );
                        value.as_float().unwrap()
                    }),
                );
            }
        }
    }

    context
}
```

</details>

### <span id="fn.render_dir"></span><code style="font-size: 0.85em">fn render_​dir</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_dir(
    src: &Path,
    context: Context,
    dst: &Path,
    force: bool,
    in_place: bool,
) -> Vec<String>
```

<details>
<summary>Source</summary>

```rust
pub fn render_dir(
    src: &Path,
    context: Context,
    dst: &Path,
    force: bool,
    in_place: bool,
) -> Vec<String> {
    let mut rendered_paths: Vec<String> = Vec::new();
    // we create a Tera instance for an empty directory so we can extend it with our template later
    let mut tmp_dir = env::temp_dir();
    tmp_dir.push(Path::new("angreal_tmp"));

    if tmp_dir.is_dir().not() {
        debug!("Creating tmpdir at {:?}", tmp_dir);
        fs::create_dir(&tmp_dir).unwrap();
    }

    tmp_dir.push(Path::new("*"));
    let mut tera = Tera::new(tmp_dir.to_str().unwrap()).unwrap();

    tmp_dir.pop();
    if tmp_dir.is_dir() {
        debug!("Destroying tmpdir at {:?}", tmp_dir);
        fs::remove_dir_all(&tmp_dir).unwrap();
    }

    // We glob our template directory
    let mut template_src = <&std::path::Path>::clone(&src).to_path_buf();
    template_src.push(Path::new("**/*"));

    // And build our full prefix
    let _template_name = <&std::path::Path>::clone(&src).file_name().unwrap();

    // In-place mode strips the single top-level templated directory. Validate it
    // up front so we fail fast on ambiguous templates before writing anything.
    if in_place {
        let _ = in_place_root(src);
    }

    for file in glob(template_src.to_str().unwrap()).expect("Failed to read glob pattern") {
        let file_path = file.as_ref().unwrap();
        let rel_path = file_path.strip_prefix(src).unwrap().to_str().unwrap();

        if file.as_ref().unwrap().is_file() && rel_path.starts_with("{{") && rel_path.contains("}}")
        {
            debug!(
                "Adding template with relative path {:?} to tera instance.",
                rel_path
            );

            tera.add_template_file(file.as_ref().unwrap().to_str().unwrap(), Some(rel_path))
                .unwrap();
        }
    }

    // In-place rendering has no single root directory whose existence guards
    // collisions, so without --force we check every target up front and refuse
    // to proceed if any already exists in the destination.
    if in_place && force.not() {
        let mut collisions: Vec<String> = Vec::new();
        for entry in WalkDir::new(src)
            .into_iter()
            .filter_entry(|e| e.file_type().is_dir())
        {
            let entry = entry.unwrap();
            let rel = entry.path().strip_prefix(src).unwrap().to_str().unwrap();
            if is_templated_segment(rel) {
                let real_path = Tera::one_off(rel, &context, false).unwrap();
                if let Some(dest_rel) = dest_rel_path(&real_path, true) {
                    if dst.join(&dest_rel).exists() {
                        collisions.push(dest_rel.to_string_lossy().to_string());
                    }
                }
            }
        }
        for template in tera.get_template_names() {
            if template == "angreal.toml" || template.starts_with('.') {
                continue;
            }
            let path = Tera::one_off(template, &context, false).unwrap();
            if let Some(dest_rel) = dest_rel_path(&path, true) {
                if dst.join(&dest_rel).exists() {
                    collisions.push(dest_rel.to_string_lossy().to_string());
                }
            }
        }
        if collisions.is_empty().not() {
            error!(
                "{} already exist(s) in {}. Will not proceed unless `--force`/force=True is used.",
                collisions.join(", "),
                dst.display()
            );
            exit(1);
        }
    }

    // build our directory structure first
    let walker = WalkDir::new(src).into_iter();
    for entry in walker.filter_entry(|e| e.file_type().is_dir()) {
        let path_template = entry.unwrap().clone();
        let path_postfix = path_template.path();
        let path_template = path_postfix.strip_prefix(src).unwrap().to_str().unwrap();

        // we only render directories that start with a templated path, this is usually a single "root" directory that forms the top level directory of a project.
        if is_templated_segment(path_template) {
            let real_path = Tera::one_off(path_template, &context, false).unwrap();

            // The dot-file skip applies to the rendered template path (e.g. a
            // top-level dot directory), so evaluate it before stripping.
            if real_path.starts_with('.') {
                //skip any sort of top level dot files - extend with an exclusion glob in the future
                // todo: exclusion glob
                continue;
            }

            // In-place mode strips the root component; `None` is the root itself.
            let dest_rel = match dest_rel_path(&real_path, in_place) {
                Some(p) => p,
                None => continue,
            };

            if dst.join(&dest_rel).is_dir() & force.not() {
                error!(
                    "{} already exists. Will not proceed unless `--force`/force=True is used.",
                    dest_rel.display()
                )
            }

            let destination = dst.join(&dest_rel);
            debug!("Creating directory {:?}", destination);
            // create_dir_all is idempotent: in-place renders into an existing
            // directory and may re-create parents on a --force overwrite.
            fs::create_dir_all(&destination).unwrap();
            rendered_paths.push(destination.to_string_lossy().to_string());
        }
    }

    // render templates
    for template in tera.get_template_names() {
        if template == "angreal.toml" {
            // never render the angreal.toml
            // todo: exclusion glob
            continue;
        }

        if template.starts_with('.') {
            // we don't render dot files either
            // todo: exclusion glob
            continue;
        }

        let rendered = tera.render(template, &context).unwrap();
        let path = Tera::one_off(template, &context, false).unwrap();

        // In-place mode strips the root component; `None` is the root itself.
        let dest_rel = match dest_rel_path(&path, in_place) {
            Some(p) => p,
            None => continue,
        };

        let destination = dst.join(&dest_rel);
        debug!("Rendering file at {:?}", destination);
        let mut output = File::create(&destination).unwrap();
        write!(output, "{}", rendered.as_str()).unwrap();
        rendered_paths.push(destination.to_string_lossy().to_string());
    }

    rendered_paths
}
```

</details>

### <span id="fn.check_up_to_date"></span><code style="font-size: 0.85em">fn check_​up_​to_​date</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn check_up_to_date() -> Result<()>
```

<details>
<summary>Source</summary>

```rust
pub fn check_up_to_date() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let response_result = client
        .get("https://pypi.org/pypi/angreal/json")
        .timeout(Duration::from_millis(400)) // Set a 400ms timeout
        .send();

    let json = match response_result {
        Ok(response) => {
            let json_result = response.json::<serde_json::Value>();
            result_or_return_err!(json_result)
        }
        Err(e) => {
            if e.is_timeout() {
                warn!("Request timed out. Please check your network connection.");
                return Ok(());
            }
            warn!("Error checking for updates: {}", e);
            return Ok(());
        }
    };

    let upstream = value_or_return_err!(json["info"]["version"].as_str());
    let current = env!("CARGO_PKG_VERSION");
    let current = value_or_return_err!(Version::from(current));
    let upstream = value_or_return_err!(Version::from(upstream));

    if upstream > current {
        println!(
            "A newer version of angreal is available, use pip install --upgrade angreal to upgrade."
        )
    };
    Ok(())
}
```

</details>

### <span id="fn.get_task_files"></span><code style="font-size: 0.85em">fn get_​task_​files</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn get_task_files(path: PathBuf) -> Result<Vec<PathBuf>>
```

Get a list of task files in given a path

**Examples:**

```rust
use angreal::utils::get_task_files;
use std::path::PathBuf;

let task_files = get_task_files(PathBuf::new("."))
```

<details>
<summary>Source</summary>

```rust
pub fn get_task_files(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut tasks = Vec::new();

    let mut pattern = path;
    pattern.push("task_*.py");
    let mut have_tasks = false;

    for entry in glob(pattern.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                info!("Found task {:?}", path.display());
                tasks.push(path);
                have_tasks = true;
            }
            Err(e) => error!("{:?}", e),
        }
    }

    if have_tasks {
        Ok(tasks)
    } else {
        error!("No tasks found for execution.");
        Err(anyhow!("No tasks found for execution."))
    }
}
```

</details>

### <span id="fn.register"></span><code style="font-size: 0.85em">fn register</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()>
```

Registers the Command and Arg structs to the python api in the `angreal` module

<details>
<summary>Source</summary>

```rust
pub fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_root, m)?)?;
    m.add_function(wrap_pyfunction!(render_template, m)?)?;
    m.add_function(wrap_pyfunction!(generate_context, m)?)?;
    m.add_function(wrap_pyfunction!(render_directory, m)?)?;
    m.add_function(wrap_pyfunction!(get_context, m)?)?;
    Ok(())
}
```

</details>

### <span id="fn.render_directory"></span><code style="font-size: 0.85em">fn render_​directory</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

```rust
fn render_directory(
    src: &str,
    dst: &str,
    force: bool,
    context: Option<&Bound<'_, PyDict>>,
) -> PyResult<Py<PyAny>>
```

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.generate_context"></span><code style="font-size: 0.85em">fn generate_​context</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

```rust
fn generate_context(path: &str, take_input: bool) -> PyResult<Py<PyAny>>
```

Generate a templating context from a toml file.

**Examples:**

```python
import angreal
angreal_root = angreal.generate_context('path/to/angreal.toml',take_input=False)
```

<details>
<summary>Source</summary>

```rust
fn generate_context(path: &str, take_input: bool) -> PyResult<Py<PyAny>> {
    let toml_path = Path::new(path).to_path_buf();
    let ctx = repl_context_from_toml(toml_path, take_input);
    let map = context_to_map(ctx);
    Ok(pythonize_this!(map))
}
```

</details>

### <span id="fn.get_root"></span><code style="font-size: 0.85em">fn get_​root</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

```rust
fn get_root() -> PyResult<String>
```

Get the root path of a current angreal project.

The root is the actual location of the .angreal file that houses task files

**Examples:**

```python
import angreal
angreal_root = angreal.get_root()
```

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.render_template"></span><code style="font-size: 0.85em">fn render_​template</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

```rust
fn render_template(
    template: &str,
    context: &Bound<'_, PyDict>,
) -> PyResult<String>
```

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.get_context"></span><code style="font-size: 0.85em">fn get_​context</code>

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

```rust
fn get_context() -> PyResult<Py<PyAny>>
```

Read the angreal.toml file from the .angreal folder and return it as a dictionary

**Examples:**

```python
import angreal
config = angreal.get_context()
```

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.is_angreal_project"></span><code style="font-size: 0.85em">fn is_​angreal_​project</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn is_angreal_project() -> Result<PathBuf>
```

Tests whether or not a current path is an angreal project

An angreal project is detected by attempting to find a `.angreal` file
anywhere in the current and parent directories.

**Examples:**

```rust
use angreal::utils::is_angreal_project

let project_path = is_angreal_project()
```

<details>
<summary>Source</summary>

```rust
pub fn is_angreal_project() -> Result<PathBuf> {
    let angreal_path = Path::new(".angreal");

    let mut check_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err(anyhow!("This doesn't appear to be an angreal project.")),
    };
    check_dir.push(angreal_path);

    let found = loop {
        if check_dir.is_dir() {
            break true;
        }

        let mut next_dir = check_dir.clone();
        next_dir.pop();
        next_dir.pop();
        next_dir.push(angreal_path);

        if next_dir == check_dir {
            break false;
        }

        check_dir = next_dir.clone();
    };

    if found {
        Ok(check_dir)
    } else {
        Err(anyhow!("This doesn't appear to be an angreal project."))
    }
}
```

</details>

### <span id="fn.load_python"></span><code style="font-size: 0.85em">fn load_​python</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn load_python(file: PathBuf) -> Result<(), PyErr>
```

Loads a python file as a pyo3 PyModule

**Examples:**

```rust
use angreal::utils::load_python
use std::path::PathBuf;

load_python(PathBuf::new("python_file.py"))?;
```

<details>
<summary>Source</summary>

```rust
pub fn load_python(file: PathBuf) -> Result<(), PyErr> {
    let mut dir = file.clone();
    dir.pop();

    let dir = dir.to_str();
    let contents = fs::read_to_string(file.clone()).unwrap();

    let r_value = Python::attach(|py| -> PyResult<()> {
        // Allow the file to search for modules it might be importing
        let sys = py.import("sys")?;
        let path_attr = sys.getattr("path")?;
        let syspath = path_attr.cast::<PyList>()?;
        syspath.insert(0, dir)?;

        // Import the file.
        use std::ffi::CString;
        let contents_cstr = CString::new(contents.as_str()).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid C string: {}", e))
        })?;
        let result = PyModule::from_code(py, contents_cstr.as_c_str(), c"", c"");

        match result {
            Ok(_result) => {
                debug!("Successfully loaded {:?}", file);
                Ok(())
            }
            Err(err) => {
                error!("{:?} failed to load", file);
                // Use the new error formatter for better error presentation
                let formatter =
                    crate::error_formatter::PythonErrorFormatter::new(err.clone_ref(py));
                println!("{}", formatter);
                Err(err)
            }
        }
    });

    match r_value {
        Ok(_ok) => Ok(()),
        Err(err) => Err(err),
    }
}
```

</details>

### <span id="fn.extract_key_defaults"></span><code style="font-size: 0.85em">fn extract_​key_​defaults</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn extract_key_defaults(
    toml_path: PathBuf,
) -> Result<Map<String, Value>>
```

Extract key/default value pairs from a TOML file

**Result Structure**

For a TOML entry like:
```toml
age = 25
```

The resulting Map will contain:
```rust
{
    "age": 25
}
```

**Examples:**

```rust
use angreal::utils::extract_key_defaults;
use std::path::PathBuf;

let defaults = extract_key_defaults(PathBuf::new("angreal.toml")).unwrap();
```

<details>
<summary>Source</summary>

```rust
pub fn extract_key_defaults(toml_path: PathBuf) -> Result<Map<String, Value>> {
    let file_contents = fs::read_to_string(&toml_path)
        .unwrap_or_else(|_| panic!("Unable to open {:?}", &toml_path));
    let extract = file_contents.parse::<Table>().unwrap();

    let mut defaults = Map::new();

    // Process each key-value pair in the root level (skipping prompt and validation sections)
    for (k, v) in extract
        .iter()
        .filter(|(key, _)| *key != "prompt" && *key != "validation")
    {
        defaults.insert(k.clone(), v.clone());
    }

    Ok(defaults)
}
```

</details>

### <span id="fn.extract_validation_rules"></span><code style="font-size: 0.85em">fn extract_​validation_​rules</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn extract_validation_rules(
    toml_path: PathBuf,
) -> Result<Map<String, Value>>
```

Extract validation rules from a TOML file

**Result Structure**

For a TOML entry like:
```toml
[validation]
age.min = 18
age.max = 65
age.type = "integer"
```

The resulting Map will contain:
```rust
{
    "age.min": 18,
    "age.max": 65,
    "age.type": "integer"
}
```

**Examples:**

```rust
use angreal::utils::extract_validation_rules;
use std::path::PathBuf;

let validations = extract_validation_rules(PathBuf::new("angreal.toml")).unwrap();
```

<details>
<summary>Source</summary>

```rust
pub fn extract_validation_rules(toml_path: PathBuf) -> Result<Map<String, Value>> {
    let file_contents = fs::read_to_string(&toml_path)
        .unwrap_or_else(|_| panic!("Unable to open {:?}", &toml_path));
    let extract = file_contents.parse::<Table>().unwrap();

    let binding_validation = Table::new();
    let validations = extract
        .get("validation")
        .and_then(|v| v.as_table())
        .unwrap_or(&binding_validation);

    let mut flattened_validations = Map::new();

    // Flatten the nested validation structure
    for (field, rules) in validations.iter() {
        if let Some(rules_table) = rules.as_table() {
            for (rule, value) in rules_table.iter() {
                let key = format!("{}.{}", field, rule);
                flattened_validations.insert(key, value.clone());
            }
        }
    }

    Ok(flattened_validations)
}
```

</details>

### <span id="fn.extract_prompts"></span><code style="font-size: 0.85em">fn extract_​prompts</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn extract_prompts(toml_path: PathBuf) -> Result<Map<String, Value>>
```

Extract prompts from a TOML file

**Result Structure**

For a TOML entry like:
```toml
[prompt]
age = "Enter your age (must be between 18 and 65)"
```

The resulting Map will contain:
```rust
{
    "age": "Enter your age (must be between 18 and 65)"
}
```

**Examples:**

```rust
use angreal::utils::extract_prompts;
use std::path::PathBuf;

let prompts = extract_prompts(PathBuf::new("angreal.toml")).unwrap();
```

<details>
<summary>Source</summary>

```rust
pub fn extract_prompts(toml_path: PathBuf) -> Result<Map<String, Value>> {
    let file_contents = fs::read_to_string(&toml_path)
        .unwrap_or_else(|_| panic!("Unable to open {:?}", &toml_path));
    let extract = file_contents.parse::<Table>().unwrap();

    let binding_prompt = Table::new();
    let prompts = extract
        .get("prompt")
        .and_then(|v| v.as_table())
        .unwrap_or(&binding_prompt);

    Ok(prompts.clone())
}
```

</details>
