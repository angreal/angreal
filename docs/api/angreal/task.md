# task


Core structures for describing tasks and arguments

## Classes

### `class Group`

> **Rust Implementation**: [angreal::task::AngrealGroup](../rust/angreal/task.md#class-group)

A group is just a special type of sub-command

#### Methods

##### `__new__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__new__</span>(name:  str, about: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">Self</span></code>
</div>

> **Rust Implementation**: [angreal::task::AngrealGroup::__new__](../rust/angreal/task.md#__new__)

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





### `class ToolDescription`

> **Rust Implementation**: [angreal::task::ToolDescription](../rust/angreal/task.md#class-tooldescription)

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

##### `__new__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__new__</span>(description:  str, risk_level: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">Self</span></code>
</div>

> **Rust Implementation**: [angreal::task::ToolDescription::__new__](../rust/angreal/task.md#__new__)

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



##### `__repr__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__repr__</span>() -> <span style="color: var(--md-default-fg-color--light);">str</span></code>
</div>

> **Rust Implementation**: [angreal::task::ToolDescription::__repr__](../rust/angreal/task.md#__repr__)

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





### `class Command`

> **Rust Implementation**: [angreal::task::AngrealCommand](../rust/angreal/task.md#class-command)

A command describes a subcommand to be registered with the CLI

#### Methods

##### `__new__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__new__</span>(name:  str, func: Py &lt; PyAny &gt;, about: Option &lt; &amp; str &gt;, long_about: Option &lt; &amp; str &gt;, group: Option &lt; Vec &lt; AngrealGroup &gt; &gt;, tool: Option &lt; ToolDescription &gt;) -> <span style="color: var(--md-default-fg-color--light);">Self</span></code>
</div>

> **Rust Implementation**: [angreal::task::AngrealCommand::__new__](../rust/angreal/task.md#__new__)

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
        let cmd = AngrealCommand {
            name: name.to_string(),
            about: about.map(|i| i.to_string()),
            long_about: long_about.map(|i| i.to_string()),
            group,
            func,
            tool,
        };

        let path_key = generate_command_path_key(&cmd);
        ANGREAL_TASKS
            .lock()
            .unwrap()
            .insert(path_key.clone(), cmd.clone());

        // Set current command path for argument registration
        set_current_command_path(path_key.clone());

        debug!(
            "Registered new command '{}' with path key: {}",
            name, path_key
        );
        debug!(
            "Updated ANGREAL_TASKS registry size: {}",
            ANGREAL_TASKS.lock().unwrap().len()
        );
        cmd
    }
```

</details>



##### `add_group`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">add_group</span>(group: AngrealGroup) -> <span style="color: var(--md-default-fg-color--light);">PyResult &lt; () &gt;</span></code>
</div>

> **Rust Implementation**: [angreal::task::AngrealCommand::add_group](../rust/angreal/task.md#add_group)

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

        // Get the current path key for this command
        let old_path_key = generate_command_path_key(self);

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

        // Generate new path key and update registry
        let new_path_key = generate_command_path_key(self);
        let mut tasks = ANGREAL_TASKS.lock().unwrap();

        // Remove old entry and insert with new key
        if let Some(_cmd) = tasks.remove(&old_path_key) {
            tasks.insert(new_path_key.clone(), self.clone());
            debug!(
                "Updated command path from '{}' to '{}'",
                old_path_key, new_path_key
            );
        } else {
            // Fallback: just insert with new key
            tasks.insert(new_path_key.clone(), self.clone());
            debug!("Inserted command with new path: '{}'", new_path_key);
        }

        debug!("Current ANGREAL_TASKS registry size: {}", tasks.len());
        drop(tasks);

        // Also update arguments registry with new path key
        let mut args_registry = ANGREAL_ARGS.lock().unwrap();
        if let Some(args) = args_registry.remove(&old_path_key) {
            args_registry.insert(new_path_key.clone(), args);
            debug!(
                "Moved arguments from '{}' to '{}'",
                old_path_key, new_path_key
            );
        }

        Ok(())
    }
```

</details>





### `class Arg`

> **Rust Implementation**: [angreal::task::AngrealArg](../rust/angreal/task.md#class-arg)

An argument to augment the behavior of an angreal command

#### Methods

##### `__new__`

<div style="background: var(--md-code-bg-color); padding: 0.75em 1em; border-radius: 0.375em; border-left: 3px solid var(--md-primary-fg-color); margin-bottom: 1em;">
<code style="color: var(--md-code-fg-color); font-family: monospace;"><span style="color: var(--md-primary-fg-color); font-weight: 600;">__new__</span>(name:  str, command_name:  str, default_value: Option &lt; &amp; str &gt;, is_flag: Option &lt; bool &gt;, require_equals: Option &lt; bool &gt;, multiple_values: Option &lt; bool &gt;, number_of_values: Option &lt; u32 &gt;, max_values: Option &lt; u32 &gt;, min_values: Option &lt; u32 &gt;, short: Option &lt; char &gt;, long: Option &lt; &amp; str &gt;, long_help: Option &lt; &amp; str &gt;, help: Option &lt; &amp; str &gt;, required: Option &lt; bool &gt;, takes_value: Option &lt; bool &gt;, python_type: Option &lt; &amp; str &gt;) -> <span style="color: var(--md-default-fg-color--light);">Self</span></code>
</div>

> **Rust Implementation**: [angreal::task::AngrealArg::__new__](../rust/angreal/task.md#__new__)

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
