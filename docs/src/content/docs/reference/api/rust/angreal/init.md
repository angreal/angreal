---
title: "angreal::init"
description: "The angreal `init` command."
editUrl: false
sidebar:
  order: 19
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

The angreal `init` command.

## Functions

### <span id="fn.init"></span><code style="font-size: 0.85em">fn init</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn init(
    template: &str,
    force: bool,
    take_inputs: bool,
    values_file: Option<&str>,
    in_place: bool,
    insecure: bool,
)
```

Initialize a new project by rendering a template.

<details>
<summary>Source</summary>

```rust
pub fn init(
    template: &str,
    force: bool,
    take_inputs: bool,
    values_file: Option<&str>,
    in_place: bool,
    insecure: bool,
) {
    let angreal_home = create_home_dot_angreal();
    let template_type = get_scheme(template).unwrap();

    debug!("Got template type {:?} for {:?}.", template_type, template);

    debug!("Template is of type {:?}", template_type.as_str());
    let template = match template_type.as_str() {
        "https" | "gitssh" | "ssh" | "git" => {
            // If we get a git url , go get it either by a clone if it doesn't
            // already exist, or as a ff pull if it does
            handle_git_template(template, angreal_home)
        }
        "file" => PathBuf::from(handle_file_template(template, &angreal_home)),
        "oci" => handle_oci_template(template, &angreal_home, insecure),
        &_ => {
            error!(
                "Unhandled template type {} from {}, exiting.",
                template_type.as_str(),
                template
            );
            exit(1);
        }
    };

    let rendered_dot_angreal_path = render_template(
        Path::new(&template),
        take_inputs,
        force,
        values_file,
        in_place,
    );

    let mut rendered_angreal_init = Path::new(&rendered_dot_angreal_path).to_path_buf();
    rendered_angreal_init.push("init.py");

    if rendered_angreal_init.is_file() {
        let init_contents = fs::read_to_string(rendered_angreal_init).unwrap();
        // Get our init function
        Python::attach(|py| {
            // Change to the rendered directory before executing Python code
            let current_dir = env::current_dir().unwrap();
            if let Err(e) = env::set_current_dir(&rendered_dot_angreal_path) {
                error!("Failed to change to rendered directory: {}", e);
                exit(1);
            }

            use std::ffi::CString;
            let init_cstr = CString::new(init_contents).unwrap();
            let function: Py<PyAny> = PyModule::from_code(py, init_cstr.as_c_str(), c"", c"")
                .unwrap()
                .getattr("init")
                .unwrap()
                .unbind();

            match function.call0(py) {
                Ok(_) => debug!("Successfully executed init.py"),
                Err(err) => {
                    use crate::error_formatter::PythonErrorFormatter;
                    error!("Failed to execute init.py");
                    let formatter = PythonErrorFormatter::new(err);
                    println!("{}", formatter);
                    // Change back to original directory before exiting
                    let _ = env::set_current_dir(current_dir);
                    std::process::exit(1);
                }
            };

            // Change back to original directory after successful execution
            if let Err(e) = env::set_current_dir(current_dir) {
                error!("Failed to change back to original directory: {}", e);
                exit(1);
            }
        });
    }

    println!(
        "Angreal template ({}) successfully rendered !",
        template.to_string_lossy()
    );
}
```

</details>

### <span id="fn.create_home_dot_angreal"></span><code style="font-size: 0.85em">fn create_​home_​dot_​angreal</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn create_home_dot_angreal() -> PathBuf
```

create the angreal caching directory for storing cloned templates

<details>
<summary>Source</summary>

```rust
pub fn create_home_dot_angreal() -> PathBuf {
    let mut home_dir = home_dir().unwrap();
    home_dir.push(".angrealrc");

    if home_dir.exists().not() {
        fs::create_dir(&home_dir).unwrap();
    }
    debug!("Angreal home directory location is {:?}", home_dir);
    home_dir
}
```

</details>

### <span id="fn.render_template"></span><code style="font-size: 0.85em">fn render_​template</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn render_template(
    path: &Path,
    take_input: bool,
    force: bool,
    values_file: Option<&str>,
    in_place: bool,
) -> String
```

render the provided angreal template path

<details>
<summary>Source</summary>

```rust
pub fn render_template(
    path: &Path,
    take_input: bool,
    force: bool,
    values_file: Option<&str>,
    in_place: bool,
) -> String {
    // Verify the provided template path is minimially compliant.
    let mut toml = path.to_path_buf();
    toml.push(Path::new("angreal.toml"));
    debug!("angreal.toml should be at {:?}", toml);
    if toml.is_file().not() {
        error!(
            "`angreal.toml` not found where expected {:}",
            toml.display()
        );
    }

    // This is a replacement for a defensive check and is the closest thing to a ternary I've seen so far.
    // Evaluates to : if values file is None, do the first closure, other wise do the second closure
    let context = values_file.map_or_else(
        || repl_context_from_toml(toml.to_path_buf(), take_input),
        |file| repl_context_from_toml(Path::new(&file).to_path_buf(), false),
    );

    // create a tera context from the toml file interactively.

    let ctx = context.clone();

    // render the provided template directory
    let rendered_files = render_dir(path, context, &env::current_dir().unwrap(), force, in_place);

    let toml_values = context_to_map(ctx);
    let toml_string = toml::to_string(&Value::Table(toml_values)).unwrap();

    for f in rendered_files {
        if f.ends_with(".angreal") {
            let mut value_path = PathBuf::new();
            value_path.push(f.as_str());
            value_path.push("angreal.toml");
            let mut output = File::create(&value_path).unwrap();
            write!(output, "{}", toml_string.as_str()).unwrap();
            debug!("Storing initialization values to {}", &value_path.display());
            return f;
        }
    }

    // let mut output = File::create(&value_path).unwrap();
    // write!(output, "{}", toml_string.as_str()).unwrap();
    // debug!("Storing initialization values to {}", &value_path.display());
    // angreal_path
    String::new()
    // return path to .angreal
}
```

</details>
