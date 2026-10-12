---
title: "angreal.task"
description: "Core structures for describing tasks and arguments"
sidebar:
  order: 10
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-purple-low); color: var(--sl-color-purple-high);">Binding</span>


Core structures for describing tasks and arguments

## Classes

### <span id="class.Group"></span>`class Group`

> **Rust Implementation**: [angreal::task::AngrealGroup](../../rust/angreal/task/#struct.AngrealGroup)

A group is just a special type of sub-command

#### Methods

##### <span id="Group.__new__"></span>`__new__`

```python
def __new__(name:  str, about: Option < & str >) -> Self
```

> **Rust Implementation**: [angreal::task::AngrealGroup::__new__](../../rust/angreal/task/#AngrealGroup.__new__)

<details>
<summary>Source</summary>

```python
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





### <span id="class.ToolDescription"></span>`class ToolDescription`

> **Rust Implementation**: [angreal::task::ToolDescription](../../rust/angreal/task/#struct.ToolDescription)

Rich description for exposing a command to AI agents

This class allows task authors to provide detailed, prose-based descriptions
that help AI agents understand when and how to use a command effectively.
The description is essentially a mini-prompt that teaches the agent about the tool.
# Example
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
```
## Preconditions
- Rust toolchain installed
- Run `angreal dev check-deps` first if unsure
""")
)
def build():
pass
```

#### Methods

##### <span id="ToolDescription.__new__"></span>`__new__`

```python
def __new__(description:  str, risk_level: Option < & str >) -> Self
```

> **Rust Implementation**: [angreal::task::ToolDescription::__new__](../../rust/angreal/task/#ToolDescription.__new__)

<details>
<summary>Source</summary>

```python
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



##### <span id="ToolDescription.__repr__"></span>`__repr__`

```python
def __repr__() -> str
```

> **Rust Implementation**: [angreal::task::ToolDescription::__repr__](../../rust/angreal/task/#ToolDescription.__repr__)

<details>
<summary>Source</summary>

```python
    fn __repr__(&self) -> String {
        format!(
            "ToolDescription(description=<{} chars>, risk_level='{}')",
            self.description.len(),
            self.risk_level
        )
    }
```

</details>





### <span id="class.Command"></span>`class Command`

> **Rust Implementation**: [angreal::task::AngrealCommand](../../rust/angreal/task/#struct.AngrealCommand)

A command describes a subcommand to be registered with the CLI

#### Methods

##### <span id="Command.__new__"></span>`__new__`

```python
def __new__(
    name:  str,
    func: Py < PyAny >,
    about: Option < & str >,
    long_about: Option < & str >,
    group: Option < Vec < AngrealGroup > >,
    tool: Option < ToolDescription >,
) -> Self
```

> **Rust Implementation**: [angreal::task::AngrealCommand::__new__](../../rust/angreal/task/#AngrealCommand.__new__)

Initialization method for the object. The command is registered to `ANGREAL_TASKS` upon instantiation from the python api

The decorated version is the most ergonmoic way to use this object.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | ` str` |  |
| `func` | `Py < PyAny >` |  |
| `about` | `Option < & str >` |  |
| `long_about` | `Option < & str >` |  |
| `group` | `Option < Vec < AngrealGroup > >` |  |
| `tool` | `Option < ToolDescription >` |  |


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

```python
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



##### <span id="Command.add_group"></span>`add_group`

```python
def add_group(group: AngrealGroup) -> PyResult < () >
```

> **Rust Implementation**: [angreal::task::AngrealCommand::add_group](../../rust/angreal/task/#AngrealCommand.add_group)

Add a (task::AngrealGroup) to the task::AngrealCommand called on

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `group` | `AngrealGroup` |  |


<details>
<summary>Source</summary>

```python
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





### <span id="class.Arg"></span>`class Arg`

> **Rust Implementation**: [angreal::task::AngrealArg](../../rust/angreal/task/#struct.AngrealArg)

An argument to augment the behavior of an angreal command

#### Methods

##### <span id="Arg.__new__"></span>`__new__`

```python
def __new__(
    name:  str,
    command_name:  str,
    default_value: Option < & str >,
    is_flag: Option < bool >,
    require_equals: Option < bool >,
    multiple_values: Option < bool >,
    number_of_values: Option < u32 >,
    max_values: Option < u32 >,
    min_values: Option < u32 >,
    short: Option < char >,
    long: Option < & str >,
    long_help: Option < & str >,
    help: Option < & str >,
    required: Option < bool >,
    takes_value: Option < bool >,
    python_type: Option < & str >,
) -> Self
```

> **Rust Implementation**: [angreal::task::AngrealArg::__new__](../../rust/angreal/task/#AngrealArg.__new__)

Adds an argument to an angreal command.

The decorated version is the most ergonmoic way to use this object.

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `name` | ` str` |  |
| `command_name` | ` str` |  |
| `default_value` | `Option < & str >` |  |
| `is_flag` | `Option < bool >` |  |
| `require_equals` | `Option < bool >` |  |
| `multiple_values` | `Option < bool >` |  |
| `number_of_values` | `Option < u32 >` |  |
| `max_values` | `Option < u32 >` |  |
| `min_values` | `Option < u32 >` |  |
| `short` | `Option < char >` |  |
| `long` | `Option < & str >` |  |
| `long_help` | `Option < & str >` |  |
| `help` | `Option < & str >` |  |
| `required` | `Option < bool >` |  |
| `takes_value` | `Option < bool >` |  |
| `python_type` | `Option < & str >` |  |


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

```python
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
