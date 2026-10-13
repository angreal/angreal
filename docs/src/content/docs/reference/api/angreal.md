---
title: "angreal"
description: "API reference of the Python module angreal."
editUrl: false
sidebar:
  order: 1
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>

## Classes

### <span id="class.Command"></span><code style="font-size: 0.85em">class Command</code>

> **Rust Implementation**: [`angreal::task::AngrealCommand`](../rust/angreal/task/#struct.AngrealCommand)

A command describes a subcommand to be registered with the CLI

#### Methods

##### <span id="Command.__new__"></span><code>_​_​new_​_​</code>

```python
def __new__(
    name: str,
    func: Py<PyAny>,
    about: Optional[str],
    long_about: Optional[str],
    group: Optional[List[AngrealGroup]],
    tool: Optional[ToolDescription],
) -> Self
```

> **Rust Implementation**: [`angreal::task::AngrealCommand::__new__`](../rust/angreal/task/#AngrealCommand.__new__)

Initialization method for the object. The command is registered to `ANGREAL_TASKS` upon instantiation from the python api

The decorated version is the most ergonmoic way to use this object.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `str` |  |
| `func` | `Py<PyAny>` |  |
| `about` | `Optional[str]` |  |
| `long_about` | `Optional[str]` |  |
| `group` | `Optional[List[AngrealGroup]]` |  |
| `tool` | `Optional[ToolDescription]` |  |

**Examples:**

```python
import angreal

@angreal.command(name='test',about='a short message',
long_about='a much longer message`)
def test-message():
    pass
```

```python
import angreal

def test-message():
    pass

angreal.Command(name='test',about='a short message',
long_about='a much longer message`, func=test-message)
```

<details>
<summary>Source</summary>

```rust
    fn __new__(
        name: &str,
        func: Py<PyAny>,
        about: Option<&str>,
        long_about: Option<&str>,
        group: Option<Vec<AngrealGroup>>,
        tool: Option<ToolDescription>,
    ) -> Self {
        debug!("Creating new AngrealCommand with name: {}", name);

        // Generate a unique registry key to prevent collisions when two commands
        // share the same base name during transient registration (before group
        // decorators run). For example, a top-level "build" and a grouped
        // "docs build" both initially register as "build" — without a unique key,
        // the second silently overwrites the first.
        let id = COMMAND_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path_key = generate_command_path_key_from_parts(group.as_deref(), name);
        let registry_key = format!("{}.__reg_{}", path_key, id);

        let cmd = AngrealCommand {
            name: name.to_string(),
            about: about.map(|i| i.to_string()),
            long_about: long_about.map(|i| i.to_string()),
            group,
            func,
            tool,
            registry_key: Some(registry_key.clone()),
        };

        ANGREAL_TASKS
            .lock()
            .unwrap()
            .insert(registry_key.clone(), cmd.clone());

        // Set current command path for argument registration
        set_current_command_path(registry_key.clone());

        debug!(
            "Registered new command '{}' with registry key: {}",
            name, registry_key
        );
        debug!(
            "Updated ANGREAL_TASKS registry size: {}",
            ANGREAL_TASKS.lock().unwrap().len()
        );
        cmd
    }
```

</details>

##### <span id="Command.add_group"></span><code>add_​group</code>

```python
def add_group(group: AngrealGroup) -> None
```

> **Rust Implementation**: [`angreal::task::AngrealCommand::add_group`](../rust/angreal/task/#AngrealCommand.add_group)

Add a (task::AngrealGroup) to the task::AngrealCommand called on

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `group` | `AngrealGroup` |  |

<details>
<summary>Source</summary>

```rust
    pub fn add_group(&mut self, group: AngrealGroup) -> PyResult<()> {
        debug!("Adding group '{}' to command '{}'", group.name, self.name);

        // Use the unique registry key to find our entry (avoids collisions
        // with other commands that share the same base name).
        let old_registry_key = self
            .registry_key
            .clone()
            .unwrap_or_else(|| generate_command_path_key(self));

        if self.group.is_none() {
            debug!(
                "Initializing empty group vector for command '{}'",
                self.name
            );
            self.group = Some(Vec::new());
        }

        let mut g = self.group.as_mut().unwrap().clone();

        debug!("Adding group '{}' to command '{}'", group.name, self.name);
        g.insert(0, group);
        self.group = Some(g.clone());

        // Generate a new unique registry key for the updated command
        let id = COMMAND_COUNTER.fetch_add(1, Ordering::Relaxed);
        let new_path_key = generate_command_path_key(self);
        let new_registry_key = format!("{}.__reg_{}", new_path_key, id);
        self.registry_key = Some(new_registry_key.clone());

        let mut tasks = ANGREAL_TASKS.lock().unwrap();

        // Remove old entry by its unique registry key and insert with new key
        if let Some(_cmd) = tasks.remove(&old_registry_key) {
            tasks.insert(new_registry_key.clone(), self.clone());
            debug!(
                "Updated command registry key from '{}' to '{}'",
                old_registry_key, new_registry_key
            );
        } else {
            // Fallback: just insert with new key
            tasks.insert(new_registry_key.clone(), self.clone());
            debug!(
                "Inserted command with new registry key: '{}'",
                new_registry_key
            );
        }

        debug!("Current ANGREAL_TASKS registry size: {}", tasks.len());
        drop(tasks);

        // Also update arguments registry with new key
        let mut args_registry = ANGREAL_ARGS.lock().unwrap();
        if let Some(args) = args_registry.remove(&old_registry_key) {
            args_registry.insert(new_registry_key.clone(), args);
            debug!(
                "Moved arguments from '{}' to '{}'",
                old_registry_key, new_registry_key
            );
        }

        Ok(())
    }
```

</details>

### <span id="class.Arg"></span><code style="font-size: 0.85em">class Arg</code>

> **Rust Implementation**: [`angreal::task::AngrealArg`](../rust/angreal/task/#struct.AngrealArg)

An argument to augment the behavior of an angreal command

#### Methods

##### <span id="Arg.__new__"></span><code>_​_​new_​_​</code>

```python
def __new__(
    name: str,
    command_name: str,
    default_value: Optional[str],
    is_flag: Optional[bool],
    require_equals: Optional[bool],
    multiple_values: Optional[bool],
    number_of_values: Optional[int],
    max_values: Optional[int],
    min_values: Optional[int],
    short: Optional[char],
    long: Optional[str],
    long_help: Optional[str],
    help: Optional[str],
    required: Optional[bool],
    takes_value: Optional[bool],
    python_type: Optional[str],
) -> Self
```

> **Rust Implementation**: [`angreal::task::AngrealArg::__new__`](../rust/angreal/task/#AngrealArg.__new__)

Adds an argument to an angreal command.

The decorated version is the most ergonmoic way to use this object.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `str` |  |
| `command_name` | `str` |  |
| `default_value` | `Optional[str]` |  |
| `is_flag` | `Optional[bool]` |  |
| `require_equals` | `Optional[bool]` |  |
| `multiple_values` | `Optional[bool]` |  |
| `number_of_values` | `Optional[int]` |  |
| `max_values` | `Optional[int]` |  |
| `min_values` | `Optional[int]` |  |
| `short` | `Optional[char]` |  |
| `long` | `Optional[str]` |  |
| `long_help` | `Optional[str]` |  |
| `help` | `Optional[str]` |  |
| `required` | `Optional[bool]` |  |
| `takes_value` | `Optional[bool]` |  |
| `python_type` | `Optional[str]` |  |

**Examples:**

```python
import angreal

@angreal.command(name='echo',about='a needless echo replacement',
@angreal.argument(name="phrase", help="the phrase to echo", required=True)
   def task_echo(phrase):
       print(phrase)
```

```python
import angreal

def echo(phrase):
    print(phrase)

angreal.Command(name='echo',about='a needless echo replacement', func=test-message)
angreal.Arg(name="phrase", help="the phrase to echo", required=True, command_name='echo')
```

<details>
<summary>Source</summary>

```rust
    fn __new__(
        name: &str,
        command_name: &str,
        default_value: Option<&str>,
        is_flag: Option<bool>,
        require_equals: Option<bool>,
        multiple_values: Option<bool>,
        number_of_values: Option<u32>,
        max_values: Option<u32>,
        min_values: Option<u32>,
        short: Option<char>,
        long: Option<&str>,
        long_help: Option<&str>,
        help: Option<&str>,
        required: Option<bool>,
        takes_value: Option<bool>,
        python_type: Option<&str>,
    ) -> Self {
        debug!(
            "Creating new AngrealArg '{}' for command '{}'",
            name, command_name
        );

        // Get the current command path or fallback to command_name if not available
        let command_path = get_current_command_path().unwrap_or_else(|| command_name.to_string());

        let arg = AngrealArg {
            name: name.to_string(),
            command_name: command_name.to_string(),
            command_path: command_path.clone(),
            takes_value: Some(takes_value.unwrap_or(true)),
            default_value: default_value.map(|i| i.to_string()),
            is_flag: Some(is_flag.unwrap_or(false)),
            require_equals,
            multiple_values,
            number_of_values,
            max_values,
            min_values,
            python_type: Some(python_type.unwrap_or("str").to_string()),
            short,
            long: long.map(|i| i.to_string()),
            long_help: long_help.map(|i| i.to_string()),
            help: help.map(|i| i.to_string()),
            required,
        };

        // Insert into HashMap using command path as key
        let mut args_registry = ANGREAL_ARGS.lock().unwrap();
        args_registry
            .entry(command_path.clone())
            .or_default()
            .push(arg.clone());

        debug!(
            "Registered new argument '{}' for command path '{}'",
            name, command_path
        );
        debug!(
            "Current ANGREAL_ARGS registry has {} command paths",
            args_registry.len()
        );
        arg
    }
```

</details>

### <span id="class.Group"></span><code style="font-size: 0.85em">class Group</code>

> **Rust Implementation**: [`angreal::task::AngrealGroup`](../rust/angreal/task/#struct.AngrealGroup)

A group is just a special type of sub-command

#### Methods

##### <span id="Group.__new__"></span><code>_​_​new_​_​</code>

```python
def __new__(name: str, about: Optional[str]) -> Self
```

> **Rust Implementation**: [`angreal::task::AngrealGroup::__new__`](../rust/angreal/task/#AngrealGroup.__new__)

<details>
<summary>Source</summary>

```rust
    fn __new__(name: &str, about: Option<&str>) -> Self {
        let group = AngrealGroup {
            name: name.to_string(),
            about: about.map(|i| i.to_string()),
        };

        let mut groups = ANGREAL_GROUPS.lock().unwrap();
        if !groups.iter().any(|g| g.name == group.name) {
            debug!("Adding new group: {}", group.name);
            groups.push(group.clone());
        } else {
            debug!("Group {} already exists, skipping add", group.name);
        }
        drop(groups);
        debug!(
            "Current ANGREAL_GROUPS state: {:#?}",
            ANGREAL_GROUPS.lock().unwrap()
        );
        group
    }
```

</details>

### <span id="class.ToolDescription"></span><code style="font-size: 0.85em">class ToolDescription</code>

> **Rust Implementation**: [`angreal::task::ToolDescription`](../rust/angreal/task/#struct.ToolDescription)

Rich description for exposing a command to AI agents

This class allows task authors to provide detailed, prose-based descriptions
that help AI agents understand when and how to use a command effectively.
The description is essentially a mini-prompt that teaches the agent about the tool.

**Examples:**

```python
import angreal

@angreal.command(
    name="build",
    about="Build the project",
    tool=angreal.ToolDescription("""
Compiles all Rust crates and creates the Python wheel for distribution.

## When to use
- Before releasing a new version
- Testing production builds locally

## When NOT to use
- During iterative development (use `cargo build` directly)

## Examples
```
angreal build
angreal build --release
```rust

# Preconditions
- Rust toolchain installed
- Run `angreal dev check-deps` first if unsure
""")
)
def build():
    pass
```

#### Methods

##### <span id="ToolDescription.__new__"></span><code>_​_​new_​_​</code>

```python
def __new__(description: str, risk_level: Optional[str]) -> Self
```

> **Rust Implementation**: [`angreal::task::ToolDescription::__new__`](../rust/angreal/task/#ToolDescription.__new__)

<details>
<summary>Source</summary>

```rust
    fn __new__(description: &str, risk_level: Option<&str>) -> Self {
        let risk = risk_level.unwrap_or("safe");
        // Validate risk_level
        let validated_risk = match risk {
            "safe" | "read_only" | "destructive" => risk.to_string(),
            _ => {
                log::warn!(
                    "Invalid risk_level '{}', defaulting to 'safe'. Valid values: safe, read_only, destructive",
                    risk
                );
                "safe".to_string()
            }
        };

        ToolDescription {
            description: description.to_string(),
            risk_level: validated_risk,
        }
    }
```

</details>

##### <span id="ToolDescription.__repr__"></span><code>_​_​repr_​_​</code>

```python
def __repr__() -> str
```

> **Rust Implementation**: [`angreal::task::ToolDescription::__repr__`](../rust/angreal/task/#ToolDescription.__repr__)

<details>
<summary>Source</summary>

```rust
    fn __repr__(&self) -> String {
        format!(
            "ToolDescription(description=<{} chars>, risk_level='{}')",
            self.description.len(),
            self.risk_level
        )
    }
```

</details>

### <span id="class.GroupDecorator"></span><code style="font-size: 0.85em">class GroupDecorator</code>

> **Rust Implementation**: [`angreal::python_bindings::decorators::GroupDecorator`](../rust/angreal/python_bindings/decorators/#struct.GroupDecorator)

A Python callable that wraps the group decorator logic

#### Methods

##### <span id="GroupDecorator.__call__"></span><code>_​_​call_​_​</code>

```python
def __call__(func: Optional[Py<PyAny>]) -> Py<PyAny>
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::GroupDecorator::__call__`](../rust/angreal/python_bindings/decorators/#GroupDecorator.__call__)

<details>
<summary>Source</summary>

```rust
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

### <span id="class.CommandDecorator"></span><code style="font-size: 0.85em">class CommandDecorator</code>

> **Rust Implementation**: [`angreal::python_bindings::decorators::CommandDecorator`](../rust/angreal/python_bindings/decorators/#struct.CommandDecorator)

A Python callable that wraps the command decorator logic

#### Methods

##### <span id="CommandDecorator.__call__"></span><code>_​_​call_​_​</code>

```python
def __call__(func: Py<PyAny>) -> Py<PyAny>
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::CommandDecorator::__call__`](../rust/angreal/python_bindings/decorators/#CommandDecorator.__call__)

<details>
<summary>Source</summary>

```rust
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

### <span id="class.ArgumentDecorator"></span><code style="font-size: 0.85em">class ArgumentDecorator</code>

> **Rust Implementation**: [`angreal::python_bindings::decorators::ArgumentDecorator`](../rust/angreal/python_bindings/decorators/#struct.ArgumentDecorator)

A Python callable that wraps the argument decorator logic

#### Methods

##### <span id="ArgumentDecorator.__call__"></span><code>_​_​call_​_​</code>

```python
def __call__(func: Py<PyAny>) -> Py<PyAny>
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::ArgumentDecorator::__call__`](../rust/angreal/python_bindings/decorators/#ArgumentDecorator.__call__)

<details>
<summary>Source</summary>

```rust
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

### <span id="class.ActivationInfo"></span><code style="font-size: 0.85em">class ActivationInfo</code>

> **Rust Implementation**: [`angreal::integrations::uv::ActivationInfo`](../rust/angreal/integrations/uv/#struct.ActivationInfo)

## Functions

### <span id="fn.main"></span><code style="font-size: 0.85em">main</code>

```python
def main() -> None
```

> **Rust Implementation**: [`angreal::main`](../rust/angreal/#fn.main)

The main function is just an entry point to be called from the core angreal library.

<details>
<summary>Source</summary>

```rust
fn main(py: Python<'_>) -> PyResult<()> {
    let handle = logger::init_logger();
    if std::env::var("ANGREAL_DEBUG").unwrap_or_default() == "true" {
        logger::update_verbosity(&handle, 2);
        warn!("Angreal application starting with debug level logging from environment");
    }
    debug!("Angreal application starting...");

    // Pull argv from Python's sys.argv rather than std::env::args(). musl does
    // not pass argc/argv to .init_array constructors in dlopened shared objects,
    // so the Rust runtime never populates its ARGV static and std::env::args()
    // returns empty — the historical split_off(2) then panicked with
    // "2 > 0". sys.argv is owned by the Python interpreter and is always
    // populated, regardless of libc. Drop sys.argv[0] (the script path); the
    // remaining elements are the user args. clap's app has NoBinaryName set,
    // so no synthetic argv[0] is needed.
    let sys = py.import("sys")?;
    let py_argv: Vec<String> = sys.getattr("argv")?.extract()?;
    let argvs: Vec<String> = py_argv.into_iter().skip(1).collect();

    // Auto-install shell completion on first run (before other operations)
    if let Err(e) = completion::auto_install_completion() {
        warn!("Failed to auto-install shell completion: {}", e);
    }

    debug!("Checking if binary is up to date...");
    match utils::check_up_to_date() {
        Ok(()) => (),
        Err(e) => warn!(
            "An error occurred while checking if our binary is up to date. {}",
            e
        ),
    };

    // Load any angreal task assets that are available to us
    let angreal_project_result = utils::is_angreal_project();
    let in_angreal_project = angreal_project_result.is_ok();

    if in_angreal_project {
        debug!("Angreal project detected, loading found tasks.");
        let angreal_path = angreal_project_result.expect("Expected angreal project path");
        // get a list of files
        let angreal_tasks_to_load = utils::get_task_files(angreal_path);

        // Explicitly capture error with exit
        let _angreal_tasks_to_load = match angreal_tasks_to_load {
            Ok(tasks) => tasks,
            Err(_) => {
                error!("Exiting due to unrecoverable error.");
                exit(1);
            }
        };

        // load the files , IF a file has command or task decorators - they'll register themselves now
        for task in _angreal_tasks_to_load.iter() {
            if let Err(e) = utils::load_python(task.clone()) {
                error!("Failed to load Python task: {}", e);
            }
        }
    }

    let app = build_app(in_angreal_project);
    let mut app_copy = app.clone();
    let sub_command = app.get_matches_from(&argvs);

    // Get our asked for verbosity and set the logger up. TODO: find a way to initialize earlier and reset after.
    let verbosity = sub_command.get_count("verbose");

    // If the user hasn't set the ANGREAL_DEBUG environment variable, set the verbosity from CLI settings
    if std::env::var("ANGREAL_DEBUG").is_err() {
        logger::update_verbosity(&handle, verbosity);
        debug!("Log verbosity set to level: {}", verbosity);
    }

    match sub_command.subcommand() {
        Some(("init", _sub_matches)) => init::init(
            _sub_matches.value_of("template").unwrap(),
            _sub_matches.is_present("force"),
            _sub_matches.is_present("defaults").not(),
            if _sub_matches.is_present("values_file") {
                Some(_sub_matches.value_of("values_file").unwrap())
            } else {
                None
            },
            _sub_matches.is_present("in_place"),
            _sub_matches.is_present("insecure"),
        ),
        Some(("_complete", _sub_matches)) => {
            // Hidden command for shell completion
            let args: Vec<String> = _sub_matches
                .values_of("args")
                .unwrap_or_default()
                .map(|s| s.to_string())
                .collect();

            match completion::generate_completions(&args) {
                Ok(completions) => {
                    for completion in completions {
                        println!("{}", completion);
                    }
                }
                Err(e) => {
                    debug!("Completion generation failed: {}", e);
                }
            }
            return Ok(());
        }
        Some(("_completion", _sub_matches)) => {
            // Hidden command for completion script generation
            let shell = _sub_matches.value_of("shell").unwrap_or("bash");
            match shell {
                "bash" => println!("{}", completion::bash::generate_completion_script()),
                "zsh" => println!("{}", completion::zsh::generate_completion_script()),
                _ => {
                    error!("Unsupported shell for completion: {}", shell);
                    exit(1);
                }
            }
            return Ok(());
        }
        Some(("alias", sub_matches)) => {
            // Handle alias subcommands
            match sub_matches.subcommand() {
                Some(("create", create_matches)) => {
                    let name = create_matches.value_of("name").unwrap();
                    Python::attach(|_py| {
                        if let Err(e) = register_entrypoint(name) {
                            error!("Failed to create alias: {}", e);
                            exit(1);
                        }
                    });
                }
                Some(("remove", remove_matches)) => {
                    let name = remove_matches.value_of("name").unwrap();
                    Python::attach(|_py| {
                        if let Err(e) = unregister_entrypoint(name) {
                            error!("Failed to remove alias: {}", e);
                            exit(1);
                        }
                    });
                }
                Some(("list", _)) => {
                    Python::attach(|_py| match list_entrypoints() {
                        Ok(aliases) => {
                            if aliases.is_empty() {
                                println!("No aliases registered.");
                            } else {
                                println!("Registered aliases:");
                                for alias in aliases {
                                    println!("  {}", alias);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to list aliases: {}", e);
                            exit(1);
                        }
                    });
                }
                _ => {
                    error!("Invalid alias subcommand. Use 'create', 'remove', or 'list'.");
                    exit(1);
                }
            }
            return Ok(());
        }
        Some(("completion", sub_matches)) => {
            // Handle completion management subcommands
            match sub_matches.subcommand() {
                Some(("install", install_matches)) => {
                    let shell = install_matches.value_of("shell");
                    match crate::completion::force_install_completion(shell) {
                        Ok(()) => {}
                        Err(e) => {
                            error!("Failed to install completion: {}", e);
                            exit(1);
                        }
                    }
                }
                Some(("uninstall", uninstall_matches)) => {
                    let shell = uninstall_matches.value_of("shell");
                    match crate::completion::uninstall_completion(shell) {
                        Ok(()) => {}
                        Err(e) => {
                            error!("Failed to uninstall completion: {}", e);
                            exit(1);
                        }
                    }
                }
                Some(("status", _)) => match crate::completion::show_completion_status() {
                    Ok(()) => {}
                    Err(e) => {
                        error!("Failed to show completion status: {}", e);
                        exit(1);
                    }
                },
                _ => {
                    error!(
                        "Invalid completion subcommand. Use 'install', 'uninstall', or 'status'."
                    );
                    exit(1);
                }
            }
            return Ok(());
        }
        Some(("tree", sub_matches)) => {
            if !in_angreal_project {
                error!("This doesn't appear to be an angreal project.");
                exit(1);
            }

            // Build command tree from registry
            let mut root = command_tree::CommandNode::new_group("angreal".to_string(), None);
            for (_, cmd) in ANGREAL_TASKS.lock().unwrap().iter() {
                root.add_command(cmd.clone());
            }

            let long = sub_matches.get_flag("long");
            tree_output::print_tree(&root, long);

            return Ok(());
        }
        Some(("mcp", _)) => {
            if !in_angreal_project {
                error!("This doesn't appear to be an angreal project.");
                exit(1);
            }

            mcp::serve();
            return Ok(());
        }
        Some((task, sub_m)) => {
            if !in_angreal_project {
                error!("This doesn't appear to be an angreal project.");
                exit(1)
            }

            let mut command_groups: Vec<String> = Vec::new();
            command_groups.push(task.to_string());

            // iterate matches to get our final command and get our final arg matches
            // object for applying down stream
            let mut next = sub_m.subcommand();
            let mut arg_matches = sub_m.clone();
            while next.is_some() {
                let cmd = next.unwrap();
                command_groups.push(cmd.0.to_string());
                next = cmd.1.subcommand();
                arg_matches = cmd.1.clone();
            }

            let task = command_groups.pop().unwrap();

            // Generate the logical path key for command lookup
            let command_path = generate_path_key_from_parts(&command_groups, &task);
            let tasks_registry = ANGREAL_TASKS.lock().unwrap();

            debug!("Looking up command with path: {}", command_path);
            // Find the command by its logical path (registry keys include a
            // unique suffix to prevent collisions during decoration).
            let (registry_key, command) = match tasks_registry
                .iter()
                .find(|(_, cmd)| generate_command_path_key(cmd) == command_path)
            {
                None => {
                    error!("Command '{}' not found.", task);
                    app_copy.print_help().unwrap_or(());
                    exit(1)
                }
                Some((key, found_command)) => (key.clone(), found_command),
            };

            debug!(
                "Executing command: {} (registry key: {})",
                task, registry_key
            );

            let args = builder::select_args(&registry_key);
            Python::attach(|py| {
                debug!("Starting Python execution for command: {}", task);
                let mut kwargs: Vec<(&str, Py<PyAny>)> = Vec::new();

                for arg in args.into_iter() {
                    let n = Box::leak(Box::new(arg.name));
                    // unable to find the value of the passed arg with sub_m when its been wrapped
                    // in a command group

                    if arg.is_flag.unwrap() {
                        let v = arg_matches.get_flag(&n.clone());
                        kwargs.push((
                            n.as_str(),
                            v.into_bound_py_any(py)
                                .expect("Failed to convert to Python object")
                                .unbind(),
                        ));
                    } else {
                        let v = arg_matches.value_of(n.clone());
                        match v {
                            None => {
                                // We need to handle "boolean flags" that are present w/o a value
                                // should probably test that the name is a "boolean type also"
                                kwargs.push((
                                    n.as_str(),
                                    v.into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                ));
                            }
                            Some(v) => match arg.python_type.unwrap().as_str() {
                                "str" => kwargs.push((
                                    n.as_str(),
                                    v.into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                                "int" => kwargs.push((
                                    n.as_str(),
                                    v.parse::<i32>()
                                        .unwrap()
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                                "float" => kwargs.push((
                                    n.as_str(),
                                    v.parse::<f32>()
                                        .unwrap()
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                                _ => kwargs.push((
                                    n.as_str(),
                                    v.into_bound_py_any(py)
                                        .expect("Failed to convert to Python object")
                                        .unbind(),
                                )),
                            },
                        }
                    }
                }

                let kwargs_dict = match kwargs.into_py_dict(py) {
                    Ok(dict) => dict,
                    Err(err) => {
                        error!("Failed to convert kwargs to dict");
                        let formatter = PythonErrorFormatter::new(err);
                        println!("{}", formatter);
                        exit(1);
                    }
                };
                let r_value = command.func.call(py, (), Some(&kwargs_dict));

                match r_value {
                    Ok(r_value) => {
                        // Check bool before int — in Python, bool is a subtype of int
                        // (True == 1, False == 0), so extract::<i32> would match bools
                        if let Ok(val) = r_value.extract::<bool>(py) {
                            if !val {
                                exit(1);
                            }
                        } else if let Ok(code) = r_value.extract::<i32>(py) {
                            if code != 0 {
                                exit(code);
                            }
                        }
                        // None, True, or other → success
                        debug!("Successfully executed Python command: {}", task);
                    }
                    Err(err) => {
                        // SystemExit → exit with the original code
                        let is_sys_exit = err
                            .value(py)
                            .get_type()
                            .name()
                            .map(|n| n == "SystemExit")
                            .unwrap_or(false);
                        if is_sys_exit {
                            let code = err
                                .value(py)
                                .getattr("code")
                                .and_then(|c| c.extract::<i32>())
                                .unwrap_or(1);
                            exit(code);
                        }
                        error!("Failed to execute Python command: {}", task);
                        let formatter = PythonErrorFormatter::new(err);
                        println!("{}", formatter);
                        exit(56);
                    }
                }
            });
        }
        _ => {
            println!("process for current context")
        }
    }

    debug!("Angreal application completed successfully.");
    Ok(())
}
```

</details>

### <span id="fn.required_version"></span><code style="font-size: 0.85em">required_​version</code>

```python
def required_version(specifier: str) -> None
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::required_version`](../rust/angreal/python_bindings/decorators/#fn.required_version)

Check if the current angreal version meets the specified requirement

This is equivalent to Python's packaging.specifiers.Specifier.contains()
but implemented in Rust using version_compare crate.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `specifier` | `str` |  |

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.group"></span><code style="font-size: 0.85em">group</code>

```python
def group(kwargs: Optional[Bound<'_, PyDict>]) -> GroupDecorator
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::group`](../rust/angreal/python_bindings/decorators/#fn.group)

Create a group decorator that assigns commands to a group

This function returns a Python decorator that can be applied to commands.
It's equivalent to the Python @group decorator.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `kwargs` | `Optional[Bound<'_, PyDict>]` |  |

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.command_group"></span><code style="font-size: 0.85em">command_​group</code>

```python
def command_group(name: str, about: Optional[str]) -> GroupDecorator
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::command_group`](../rust/angreal/python_bindings/decorators/#fn.command_group)

Generate a reusable command group decorator

This function returns a decorator function that can be used multiple times
to assign commands to the same group. It's equivalent to Python's
functools.partial(group, name=name, about=about).

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | `str` |  |
| `about` | `Optional[str]` |  |

<details>
<summary>Source</summary>

```rust
pub fn command_group(name: &str, about: Option<&str>) -> PyResult<GroupDecorator> {
    Ok(GroupDecorator {
        name: name.to_string(),
        about: about.map(|s| s.to_string()),
    })
}
```

</details>

### <span id="fn.command"></span><code style="font-size: 0.85em">command</code>

```python
def command(kwargs: Optional[Bound<'_, PyDict>]) -> CommandDecorator
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::command`](../rust/angreal/python_bindings/decorators/#fn.command)

Create a command decorator that registers functions as commands

This function returns a Python decorator that can be applied to functions.
It's equivalent to the Python @command decorator.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `kwargs` | `Optional[Bound<'_, PyDict>]` |  |

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.argument"></span><code style="font-size: 0.85em">argument</code>

```python
def argument(kwargs: Optional[Bound<'_, PyDict>]) -> ArgumentDecorator
```

> **Rust Implementation**: [`angreal::python_bindings::decorators::argument`](../rust/angreal/python_bindings/decorators/#fn.argument)

Create an argument decorator that adds command-line arguments to commands

This function returns a Python decorator that can be applied to commands
to add command-line arguments. It's equivalent to the Python @argument decorator.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `kwargs` | `Optional[Bound<'_, PyDict>]` |  |

<details>
<summary>Source</summary>

```rust
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

### <span id="fn.ensure_uv_installed"></span><code style="font-size: 0.85em">ensure_​uv_​installed</code>

```python
def ensure_uv_installed() -> None
```

> **Rust Implementation**: [`angreal::ensure_uv_installed`](../rust/angreal/#fn.ensure_uv_installed)

<details>
<summary>Source</summary>

```rust
fn ensure_uv_installed() -> PyResult<()> {
    UvIntegration::ensure_installed()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>

### <span id="fn.uv_version"></span><code style="font-size: 0.85em">uv_​version</code>

```python
def uv_version() -> str
```

> **Rust Implementation**: [`angreal::uv_version`](../rust/angreal/#fn.uv_version)

<details>
<summary>Source</summary>

```rust
fn uv_version() -> PyResult<String> {
    UvIntegration::version()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>

### <span id="fn.create_virtualenv"></span><code style="font-size: 0.85em">create_​virtualenv</code>

```python
def create_virtualenv(path: str, python_version: Optional[str]) -> None
```

> **Rust Implementation**: [`angreal::create_virtualenv`](../rust/angreal/#fn.create_virtualenv)

<details>
<summary>Source</summary>

```rust
fn create_virtualenv(path: &str, python_version: Option<&str>) -> PyResult<()> {
    UvVirtualEnv::create(Path::new(path), python_version)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    Ok(())
}
```

</details>

### <span id="fn.install_packages"></span><code style="font-size: 0.85em">install_​packages</code>

```python
def install_packages(venv_path: str, packages: List[str]) -> None
```

> **Rust Implementation**: [`angreal::install_packages`](../rust/angreal/#fn.install_packages)

<details>
<summary>Source</summary>

```rust
fn install_packages(venv_path: &str, packages: Vec<String>) -> PyResult<()> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.install_packages(&packages)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>

### <span id="fn.install_requirements"></span><code style="font-size: 0.85em">install_​requirements</code>

```python
def install_requirements(venv_path: str, requirements_file: str) -> None
```

> **Rust Implementation**: [`angreal::install_requirements`](../rust/angreal/#fn.install_requirements)

<details>
<summary>Source</summary>

```rust
fn install_requirements(venv_path: &str, requirements_file: &str) -> PyResult<()> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.install_requirements(Path::new(requirements_file))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>

### <span id="fn.discover_pythons"></span><code style="font-size: 0.85em">discover_​pythons</code>

```python
def discover_pythons() -> List[(String, String)]
```

> **Rust Implementation**: [`angreal::discover_pythons`](../rust/angreal/#fn.discover_pythons)

<details>
<summary>Source</summary>

```rust
fn discover_pythons() -> PyResult<Vec<(String, String)>> {
    UvVirtualEnv::discover_pythons()
        .map(|pythons| {
            pythons
                .into_iter()
                .map(|(version, path)| (version, path.display().to_string()))
                .collect()
        })
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>

### <span id="fn.install_python"></span><code style="font-size: 0.85em">install_​python</code>

```python
def install_python(version: str) -> str
```

> **Rust Implementation**: [`angreal::install_python`](../rust/angreal/#fn.install_python)

<details>
<summary>Source</summary>

```rust
fn install_python(version: &str) -> PyResult<String> {
    UvVirtualEnv::install_python(version)
        .map(|path| path.display().to_string())
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>

### <span id="fn.get_venv_activation_info"></span><code style="font-size: 0.85em">get_​venv_​activation_​info</code>

```python
def get_venv_activation_info(venv_path: str) -> integrations::uv::ActivationInfo
```

> **Rust Implementation**: [`angreal::get_venv_activation_info`](../rust/angreal/#fn.get_venv_activation_info)

<details>
<summary>Source</summary>

```rust
fn get_venv_activation_info(venv_path: &str) -> PyResult<integrations::uv::ActivationInfo> {
    let venv = UvVirtualEnv {
        path: PathBuf::from(venv_path),
    };
    venv.get_activation_info()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}
```

</details>

### <span id="fn.register_entrypoint"></span><code style="font-size: 0.85em">register_​entrypoint</code>

```python
def register_entrypoint(name: str) -> None
```

> **Rust Implementation**: [`angreal::register_entrypoint`](../rust/angreal/#fn.register_entrypoint)

<details>
<summary>Source</summary>

```rust
fn register_entrypoint(name: &str) -> PyResult<()> {
    use home::home_dir;
    use serde_json;

    // Get home directory, with fallback to environment variables for testing
    let home = if let Some(home_from_env) = std::env::var_os("HOME") {
        PathBuf::from(home_from_env)
    } else if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        PathBuf::from(userprofile)
    } else {
        home_dir().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cannot find home directory")
        })?
    };

    // Create directories
    let local_bin = home.join(".local").join("bin");
    fs::create_dir_all(&local_bin).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to create bin directory: {}",
            e
        ))
    })?;

    let data_dir = home.join(".angrealrc");
    fs::create_dir_all(&data_dir).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to create data directory: {}",
            e
        ))
    })?;

    // Determine script path based on platform
    #[cfg(unix)]
    let script_path = local_bin.join(name);
    #[cfg(windows)]
    let script_path = local_bin.join(format!("{}.bat", name));

    // Check for conflicts
    if script_path.exists() {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Command '{}' already exists at {}",
            name,
            script_path.display()
        )));
    }

    // Create wrapper script
    #[cfg(unix)]
    {
        let script_content = format!(
            "#!/usr/bin/env python\n# ANGREAL_ALIAS: {}\n# Auto-generated by angreal.register_entrypoint\nimport sys\ntry:\n    import angreal\n    angreal.main()\nexcept ImportError:\n    print(f\"Error: angreal not installed. Remove alias: rm {}\", file=sys.stderr)\n    sys.exit(1)\n",
            name,
            script_path.display()
        );

        fs::write(&script_path, script_content).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write script: {}",
                e
            ))
        })?;

        // Make executable
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to get permissions: {}",
                    e
                ))
            })?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to set permissions: {}",
                e
            ))
        })?;
    }

    #[cfg(windows)]
    {
        let script_content = format!(
            "@echo off\nREM ANGREAL_ALIAS: {}\nREM Auto-generated by angreal.register_entrypoint\npython -m angreal %*\n",
            name
        );
        fs::write(&script_path, script_content).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write script: {}",
                e
            ))
        })?;
    }

    // Update registry
    let registry_path = home.join(".angrealrc").join("aliases.json");
    let mut aliases: Vec<String> = if registry_path.exists() {
        let content = fs::read_to_string(&registry_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to read registry: {}",
                e
            ))
        })?;
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    if !aliases.contains(&name.to_string()) {
        aliases.push(name.to_string());
        let json = serde_json::to_string_pretty(&aliases).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to serialize registry: {}",
                e
            ))
        })?;
        fs::write(&registry_path, json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write registry: {}",
                e
            ))
        })?;
    }

    println!("✅ Registered '{}' as angreal alias", name);
    println!("Make sure ~/.local/bin is in your PATH");
    Ok(())
}
```

</details>

### <span id="fn.list_entrypoints"></span><code style="font-size: 0.85em">list_​entrypoints</code>

```python
def list_entrypoints() -> List[str]
```

> **Rust Implementation**: [`angreal::list_entrypoints`](../rust/angreal/#fn.list_entrypoints)

<details>
<summary>Source</summary>

```rust
fn list_entrypoints() -> PyResult<Vec<String>> {
    use home::home_dir;

    // Get home directory, with fallback to environment variables for testing
    let home = if let Some(home_from_env) = std::env::var_os("HOME") {
        PathBuf::from(home_from_env)
    } else if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        PathBuf::from(userprofile)
    } else {
        home_dir().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cannot find home directory")
        })?
    };

    let registry_path = home.join(".angrealrc").join("aliases.json");

    if !registry_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&registry_path).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to read registry: {}", e))
    })?;

    let aliases: Vec<String> = serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
    Ok(aliases)
}
```

</details>

### <span id="fn.unregister_entrypoint"></span><code style="font-size: 0.85em">unregister_​entrypoint</code>

```python
def unregister_entrypoint(name: str) -> None
```

> **Rust Implementation**: [`angreal::unregister_entrypoint`](../rust/angreal/#fn.unregister_entrypoint)

<details>
<summary>Source</summary>

```rust
fn unregister_entrypoint(name: &str) -> PyResult<()> {
    use home::home_dir;

    // Get home directory, with fallback to environment variables for testing
    let home = if let Some(home_from_env) = std::env::var_os("HOME") {
        PathBuf::from(home_from_env)
    } else if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        PathBuf::from(userprofile)
    } else {
        home_dir().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cannot find home directory")
        })?
    };

    // Remove script
    let local_bin = home.join(".local").join("bin");
    #[cfg(unix)]
    let script_path = local_bin.join(name);
    #[cfg(windows)]
    let script_path = local_bin.join(format!("{}.bat", name));

    if script_path.exists() {
        fs::remove_file(&script_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to remove script: {}",
                e
            ))
        })?;
    }

    // Update registry
    let registry_path = home.join(".angrealrc").join("aliases.json");

    if registry_path.exists() {
        let content = fs::read_to_string(&registry_path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to read registry: {}",
                e
            ))
        })?;

        let mut aliases: Vec<String> =
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new());
        aliases.retain(|alias| alias != name);

        let json = serde_json::to_string_pretty(&aliases).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to serialize registry: {}",
                e
            ))
        })?;
        fs::write(&registry_path, json).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write registry: {}",
                e
            ))
        })?;
    }

    println!("✅ Unregistered '{}' alias", name);
    Ok(())
}
```

</details>

### <span id="fn.cleanup_entrypoints"></span><code style="font-size: 0.85em">cleanup_​entrypoints</code>

```python
def cleanup_entrypoints() -> None
```

> **Rust Implementation**: [`angreal::cleanup_entrypoints`](../rust/angreal/#fn.cleanup_entrypoints)

<details>
<summary>Source</summary>

```rust
fn cleanup_entrypoints() -> PyResult<()> {
    let aliases = list_entrypoints()?;

    for alias in aliases {
        if let Err(e) = unregister_entrypoint(&alias) {
            eprintln!("Warning: Failed to unregister '{}': {}", alias, e);
        }
    }

    println!("✅ Cleaned up all angreal aliases");
    Ok(())
}
```

</details>
