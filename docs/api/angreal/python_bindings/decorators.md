# decorators


Python decorator functions

This module contains PyO3 functions for the decorators:
- @required_version
- @group
- @command
- @argument
- command_group helper function

## Classes

### `class GroupDecorator`

> **Rust Implementation**: [angreal::python_bindings::decorators::GroupDecorator](../../rust/angreal/python_bindings/decorators.md#class-groupdecorator)

A Python callable that wraps the group decorator logic

#### Methods

##### `__call__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__call__</span>(func: Option &lt; Py &lt; PyAny &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::GroupDecorator::__call__](../../rust/angreal/python_bindings/decorators.md#__call__)

<details>
<summary>Source</summary>

```python
    fn __call__(&self, func: Option<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            match func {
                Some(func) => {
                    // Called as a decorator on a function
                    // Check if function has __command attribute
                    let has_command = func.getattr(py, "__command").is_ok();

                    if !has_command {
                        return Err(PyErr::new::<pyo3::exceptions::PySyntaxError, _>(
                            "The group decorator must be applied before a command.",
                        ));
                    }

                    // Create the AngrealGroup using PyO3's class instantiation
                    let group_class = py.get_type::<AngrealGroup>();
                    let group = group_class.call1((&self.name, self.about.as_deref()))?;

                    // Get the __command attribute and call add_group on it
                    let command = func.getattr(py, "__command")?;
                    command.call_method1(py, "add_group", (group,))?;

                    // Return the original function (no wrapping needed in Rust)
                    Ok(func)
                }
                None => {
                    // Called as @test() - return self as the decorator
                    Ok(Py::new(py, self.clone())?.into_any())
                }
            }
        })
    }
```

</details>





### `class CommandDecorator`

> **Rust Implementation**: [angreal::python_bindings::decorators::CommandDecorator](../../rust/angreal/python_bindings/decorators.md#class-commanddecorator)

A Python callable that wraps the command decorator logic

#### Methods

##### `__call__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__call__</span>(func: Py &lt; PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::CommandDecorator::__call__](../../rust/angreal/python_bindings/decorators.md#__call__)

<details>
<summary>Source</summary>

```python
    fn __call__(&self, func: Py<PyAny>) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            // Get or generate command name
            let name = match &self.name {
                Some(name) => name.clone(),
                None => {
                    // Get function name and convert underscores to hyphens
                    func.getattr(py, "__name__")?
                        .extract::<String>(py)?
                        .to_lowercase()
                        .replace("_", "-")
                }
            };

            // Initialize __arguments if not present
            if func.getattr(py, "__arguments").is_err() {
                func.setattr(py, "__arguments", py.None())?;
            }

            // Convert Option<ToolDescription> to Python object
            let tool_py = match &self.tool {
                Some(tool) => {
                    let tool_class = py.get_type::<crate::task::ToolDescription>();
                    // risk_level is keyword-only in Python signature, so use call() with kwargs
                    let kwargs = pyo3::types::PyDict::new(py);
                    kwargs.set_item("risk_level", tool.risk_level.as_str())?;
                    tool_class
                        .call((&tool.description,), Some(&kwargs))?
                        .into_any()
                        .unbind()
                }
                None => py.None(),
            };

            // Create the AngrealCommand using PyO3's class instantiation
            let command_class = py.get_type::<crate::task::AngrealCommand>();
            let command = command_class.call1((
                &name,
                func.clone_ref(py),
                self.about.as_deref(),
                self.long_about.as_deref(),
                py.None(), // group (empty initially)
                tool_py,
            ))?;

            // Set the __command attribute on the function
            func.setattr(py, "__command", command)?;

            // Process any existing arguments stored by @argument decorators
            let arguments = func.getattr(py, "__arguments")?;
            if !arguments.is_none(py) {
                if let Ok(args_list) = arguments.extract::<Vec<Py<PyAny>>>(py) {
                    for arg_kwargs_obj in args_list {
                        // Each item should be the kwargs dict from the @argument decorator
                        let bound_arg = arg_kwargs_obj.bind(py);
                        if let Ok(kwargs_dict) = bound_arg.cast::<pyo3::types::PyDict>() {
                            // Create AngrealArg using PyO3's class instantiation
                            let arg_class = py.get_type::<crate::task::AngrealArg>();

                            // Extract parameters from kwargs
                            let arg_name = kwargs_dict
                                .get_item("name")
                                .ok()
                                .flatten()
                                .map(|v| v.extract::<String>())
                                .transpose()?
                                .unwrap_or_else(|| "default".to_string());

                            // Create a new kwargs dict for AngrealArg constructor
                            let arg_kwargs = pyo3::types::PyDict::new(py);
                            arg_kwargs.set_item("name", &arg_name)?;
                            arg_kwargs.set_item("command_name", &name)?;

                            // Copy over all the argument parameters with proper defaults
                            for (key, value) in kwargs_dict.iter() {
                                let key_str = key.extract::<String>()?;
                                match key_str.as_str() {
                                    "name" => arg_kwargs.set_item("name", value)?,
                                    "short" => {
                                        // Convert string to char if provided
                                        if let Ok(s) = value.extract::<String>() {
                                            if let Some(c) = s.chars().next() {
                                                arg_kwargs.set_item("short", c)?;
                                            } else {
                                                arg_kwargs.set_item("short", py.None())?;
                                            }
                                        } else {
                                            arg_kwargs.set_item("short", py.None())?;
                                        }
                                    }
                                    "long" => arg_kwargs.set_item("long", value)?,
                                    "help" => arg_kwargs.set_item("help", value)?,
                                    "long_help" => arg_kwargs.set_item("long_help", value)?,
                                    "required" => arg_kwargs.set_item("required", value)?,
                                    "takes_value" => arg_kwargs.set_item("takes_value", value)?,
                                    "is_flag" => arg_kwargs.set_item("is_flag", value)?,
                                    "default_value" => {
                                        arg_kwargs.set_item("default_value", value)?
                                    }
                                    "multiple_values" => {
                                        arg_kwargs.set_item("multiple_values", value)?
                                    }
                                    "number_of_values" => {
                                        arg_kwargs.set_item("number_of_values", value)?
                                    }
                                    "max_values" => arg_kwargs.set_item("max_values", value)?,
                                    "min_values" => arg_kwargs.set_item("min_values", value)?,
                                    "require_equals" => {
                                        arg_kwargs.set_item("require_equals", value)?
                                    }
                                    "python_type" => arg_kwargs.set_item("python_type", value)?,
                                    _ => {} // Ignore unknown parameters
                                }
                            }

                            // Set defaults for missing parameters
                            if !arg_kwargs.contains("default_value")? {
                                arg_kwargs.set_item("default_value", py.None())?;
                            }
                            if !arg_kwargs.contains("is_flag")? {
                                arg_kwargs.set_item("is_flag", py.None())?;
                            }
                            if !arg_kwargs.contains("require_equals")? {
                                arg_kwargs.set_item("require_equals", py.None())?;
                            }
                            if !arg_kwargs.contains("multiple_values")? {
                                arg_kwargs.set_item("multiple_values", py.None())?;
                            }
                            if !arg_kwargs.contains("number_of_values")? {
                                arg_kwargs.set_item("number_of_values", py.None())?;
                            }
                            if !arg_kwargs.contains("max_values")? {
                                arg_kwargs.set_item("max_values", py.None())?;
                            }
                            if !arg_kwargs.contains("min_values")? {
                                arg_kwargs.set_item("min_values", py.None())?;
                            }
                            if !arg_kwargs.contains("short")? {
                                arg_kwargs.set_item("short", py.None())?;
                            }
                            if !arg_kwargs.contains("long")? {
                                arg_kwargs.set_item("long", py.None())?;
                            }
                            if !arg_kwargs.contains("long_help")? {
                                arg_kwargs.set_item("long_help", py.None())?;
                            }
                            if !arg_kwargs.contains("help")? {
                                arg_kwargs.set_item("help", py.None())?;
                            }
                            if !arg_kwargs.contains("required")? {
                                arg_kwargs.set_item("required", py.None())?;
                            }
                            if !arg_kwargs.contains("takes_value")? {
                                arg_kwargs.set_item("takes_value", py.None())?;
                            }
                            if !arg_kwargs.contains("python_type")? {
                                arg_kwargs.set_item("python_type", py.None())?;
                            }

                            // Create the AngrealArg instance - this will register it in ANGREAL_ARGS
                            let _arg = arg_class.call((), Some(&arg_kwargs))?;
                        }
                    }
                }
            }

            Ok(func)
        })
    }
```

</details>





### `class ArgumentDecorator`

> **Rust Implementation**: [angreal::python_bindings::decorators::ArgumentDecorator](../../rust/angreal/python_bindings/decorators.md#class-argumentdecorator)

A Python callable that wraps the argument decorator logic

#### Methods

##### `__call__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__call__</span>(func: Py &lt; PyAny &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; Py &lt; PyAny &gt; &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::ArgumentDecorator::__call__](../../rust/angreal/python_bindings/decorators.md#__call__)

<details>
<summary>Source</summary>

```python
    fn __call__(&self, func: Py<PyAny>) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            // Initialize __arguments list if not present
            let mut arguments = if let Ok(args) = func.getattr(py, "__arguments") {
                if args.is_none(py) {
                    Vec::new()
                } else {
                    args.extract::<Vec<Py<PyAny>>>(py)
                        .unwrap_or_else(|_| Vec::new())
                }
            } else {
                Vec::new()
            };

            // Just store the kwargs for later processing by the command decorator
            if let Some(kwargs_obj) = &self.kwargs_dict {
                arguments.push(kwargs_obj.clone_ref(py));
            }

            // Set the updated __arguments list
            use pyo3::types::PyList;
            func.setattr(py, "__arguments", PyList::new(py, &arguments)?)?;

            Ok(func)
        })
    }
```

</details>





## Functions

### `required_version`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">required_version</span>(specifier:  str) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::required_version](../../rust/angreal/python_bindings/decorators.md#fn-required_version)

Check if the current angreal version meets the specified requirement

This is equivalent to Python's packaging.specifiers.Specifier.contains()
but implemented in Rust using version_compare crate.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `specifier` | ` str` |  |


<details>
<summary>Source</summary>

```python
pub fn required_version(specifier: &str) -> PyResult<()> {
    let current_version = env!("CARGO_PKG_VERSION");

    // Parse the specifier (e.g., ">=2.0.0", "==1.2.3", "~=1.0")
    let is_compatible = if let Some(required) = specifier.strip_prefix(">=") {
        compare(current_version, required) != Ok(Cmp::Lt)
    } else if let Some(required) = specifier.strip_prefix("<=") {
        compare(current_version, required) != Ok(Cmp::Gt)
    } else if let Some(required) = specifier.strip_prefix("==") {
        compare(current_version, required) == Ok(Cmp::Eq)
    } else if let Some(required) = specifier.strip_prefix("!=") {
        compare(current_version, required) != Ok(Cmp::Eq)
    } else if let Some(required) = specifier.strip_prefix(">") {
        compare(current_version, required) == Ok(Cmp::Gt)
    } else if let Some(required) = specifier.strip_prefix("<") {
        compare(current_version, required) == Ok(Cmp::Lt)
    } else {
        // Default to exact match if no operator
        compare(current_version, specifier) == Ok(Cmp::Eq)
    };

    if !is_compatible {
        return Err(PyErr::new::<pyo3::exceptions::PyEnvironmentError, _>(
            format!(
                "You require angreal {} but have {} installed.",
                specifier, current_version
            ),
        ));
    }

    Ok(())
}
```

</details>



### `group`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">group</span>(kwargs: Option &lt; &amp; Bound &lt; &#x27;_ , PyDict &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; GroupDecorator &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::group](../../rust/angreal/python_bindings/decorators.md#fn-group)

Create a group decorator that assigns commands to a group

This function returns a Python decorator that can be applied to commands.
It's equivalent to the Python @group decorator.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `kwargs` | `Option < & Bound < '_ , PyDict > >` |  |


<details>
<summary>Source</summary>

```python
pub fn group(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<GroupDecorator> {
    // Extract name and about from kwargs
    let name = kwargs
        .and_then(|d| d.get_item("name").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?
        .unwrap_or_else(|| "default".to_string());

    let about = kwargs
        .and_then(|d| d.get_item("about").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?;

    Ok(GroupDecorator { name, about })
}
```

</details>



### `command_group`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">command_group</span>(name:  str, about: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; GroupDecorator &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::command_group](../../rust/angreal/python_bindings/decorators.md#fn-command_group)

Generate a reusable command group decorator

This function returns a decorator function that can be used multiple times
to assign commands to the same group. It's equivalent to Python's
functools.partial(group, name=name, about=about).

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | ` str` |  |
| `about` | `Option < & str >` |  |


<details>
<summary>Source</summary>

```python
pub fn command_group(name: &str, about: Option<&str>) -> PyResult<GroupDecorator> {
    Ok(GroupDecorator {
        name: name.to_string(),
        about: about.map(|s| s.to_string()),
    })
}
```

</details>



### `command`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">command</span>(kwargs: Option &lt; &amp; Bound &lt; &#x27;_ , PyDict &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; CommandDecorator &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::command](../../rust/angreal/python_bindings/decorators.md#fn-command)

Create a command decorator that registers functions as commands

This function returns a Python decorator that can be applied to functions.
It's equivalent to the Python @command decorator.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `kwargs` | `Option < & Bound < '_ , PyDict > >` |  |


<details>
<summary>Source</summary>

```python
pub fn command(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<CommandDecorator> {
    // Extract parameters from kwargs
    let name = kwargs
        .and_then(|d| d.get_item("name").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?;

    let about = kwargs
        .and_then(|d| d.get_item("about").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?;

    let long_about = kwargs
        .and_then(|d| d.get_item("long_about").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?;

    let tool = kwargs
        .and_then(|d| d.get_item("tool").ok().flatten())
        .map(|v| {
            v.cast::<crate::task::ToolDescription>()
                .map(|bound| bound.borrow().clone())
                .map_err(pyo3::PyErr::from)
        })
        .transpose()?;

    Ok(CommandDecorator {
        name,
        about,
        long_about,
        tool,
    })
}
```

</details>



### `argument`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">argument</span>(kwargs: Option &lt; &amp; Bound &lt; &#x27;_ , PyDict &gt; &gt;) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; ArgumentDecorator &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::python_bindings::decorators::argument](../../rust/angreal/python_bindings/decorators.md#fn-argument)

Create an argument decorator that adds command-line arguments to commands

This function returns a Python decorator that can be applied to commands
to add command-line arguments. It's equivalent to the Python @argument decorator.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `kwargs` | `Option < & Bound < '_ , PyDict > >` |  |


<details>
<summary>Source</summary>

```python
pub fn argument(kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<ArgumentDecorator> {
    // Extract parameters from kwargs - just store them for now
    let name = kwargs
        .and_then(|d| d.get_item("name").ok().flatten())
        .map(|v| v.extract::<String>())
        .transpose()?
        .unwrap_or_else(|| "default".to_string());

    Ok(ArgumentDecorator {
        name,
        kwargs_dict: kwargs.map(|d| d.clone().into_any().unbind()),
    })
}
```

</details>
