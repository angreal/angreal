# task


Core structures for describing tasks and arguments

## Structs

### `class Group`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.task.Group](../../angreal/task.md#class-group)

A group is just a special type of sub-command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name of the command group |
| `about` | `Option < String >` | The about of the command group |

#### Methods

##### `__new__`

```rust
fn __new__ (name : & str , about : Option < & str >) -> Self
```

> **Python API**: [angreal.task.Group.__new__](../../angreal/task.md#__new__)

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





### `class ToolDescription`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.task.ToolDescription](../../angreal/task.md#class-tooldescription)

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
```

## Preconditions
- Rust toolchain installed
- Run `angreal dev check-deps` first if unsure
""")
)
def build():
    pass
```

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `description` | `String` | The full prose description of the tool (markdown supported) |
| `risk_level` | `String` | Risk level: "safe", "read_only", or "destructive" |

#### Methods

##### `__new__`

```rust
fn __new__ (description : & str , risk_level : Option < & str >) -> Self
```

> **Python API**: [angreal.task.ToolDescription.__new__](../../angreal/task.md#__new__)

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



##### `__repr__`

```rust
fn __repr__ (& self) -> String
```

> **Python API**: [angreal.task.ToolDescription.__repr__](../../angreal/task.md#__repr__)

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





### `class Command`

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.task.Command](../../angreal/task.md#class-command)

A command describes a subcommand to be registered with the CLI

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name of the sub command |
| `about` | `Option < String >` | A short description of what the command does |
| `long_about` | `Option < String >` | A longer description of what the command does |
| `func` | `Py < PyAny >` | The actual function that is executed when the command is run |
| `group` | `Option < Vec < AngrealGroup > >` | The group this command belongs to |
| `tool` | `Option < ToolDescription >` | Rich tool description for AI agent integration |

#### Methods

##### `__new__`

```rust
fn __new__ (name : & str , func : Py < PyAny > , about : Option < & str > , long_about : Option < & str > , group : Option < Vec < AngrealGroup > > , tool : Option < ToolDescription > ,) -> Self
```

> **Python API**: [angreal.task.Command.__new__](../../angreal/task.md#__new__)

Initialization method for the object. The command is registered to `ANGREAL_TASKS` upon instantiation from the python api

The decorated version is the most ergonmoic way to use this object.

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

```rust
fn add_group (& mut self , group : AngrealGroup) -> PyResult < () >
```

> **Python API**: [angreal.task.Command.add_group](../../angreal/task.md#add_group)

Add a (task::AngrealGroup) to the task::AngrealCommand called on

<details>
<summary>Source</summary>

```rust
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

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-accent-fg-color); color: white;">Binding</span>


> **Python API**: [angreal.task.Arg](../../angreal/task.md#class-arg)

An argument to augment the behavior of an angreal command

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | The name of the argument, required to match the name in the function being executed by the command |
| `command_name` | `String` | The command name associated with this argument (for backward compatibility) |
| `command_path` | `String` | The full command path (internal use for collision resolution) |
| `takes_value` | `Option < bool >` | Whether or not the argument consumes a value from the command line |
| `default_value` | `Option < String >` | The default value to be applied to the arg. |
| `is_flag` | `Option < bool >` | whether or not the argument is a flag (bool) |
| `require_equals` | `Option < bool >` | Whether or not the argument requires an `=` behind it to set a value |
| `multiple_values` | `Option < bool >` | Whether or not the argument takes multiple values |
| `number_of_values` | `Option < u32 >` | The number of values the argument takes |
| `max_values` | `Option < u32 >` | The maximum number of values the argument takes |
| `min_values` | `Option < u32 >` | The minimum number of values the argument takes |
| `python_type` | `Option < String >` | The python type to apply the the consumed value (int, string, float) |
| `short` | `Option < char >` | the short flag to be used on the command line (i.e. `-s`) |
| `long` | `Option < String >` | the long flag to be used on the command line (i.e. `--long`) |
| `long_help` | `Option < String >` | A verbose help message to be displayed |
| `help` | `Option < String >` | a shorter help message to be displayed |
| `required` | `Option < bool >` | whether or not the argument is required |

#### Methods

##### `__new__`

```rust
fn __new__ (name : & str , command_name : & str , default_value : Option < & str > , is_flag : Option < bool > , require_equals : Option < bool > , multiple_values : Option < bool > , number_of_values : Option < u32 > , max_values : Option < u32 > , min_values : Option < u32 > , short : Option < char > , long : Option < & str > , long_help : Option < & str > , help : Option < & str > , required : Option < bool > , takes_value : Option < bool > , python_type : Option < & str > ,) -> Self
```

> **Python API**: [angreal.task.Arg.__new__](../../angreal/task.md#__new__)

Adds an argument to an angreal command.

The decorated version is the most ergonmoic way to use this object.

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





## Functions

### `fn register`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn register (_py : Python < '_ > , m : & Bound < '_ , PyModule >) -> PyResult < () >
```

Registers the Command and Arg structs to the python api in the `angreal` module

<details>
<summary>Source</summary>

```rust
pub fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    debug!("Registering Angreal types to Python module");
    m.add_class::<AngrealCommand>()?;
    m.add_class::<AngrealArg>()?;
    m.add_class::<AngrealGroup>()?;
    m.add_class::<ToolDescription>()?;
    debug!("Successfully registered all Angreal types");
    Ok(())
}
```

</details>



### `fn set_current_command_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn set_current_command_path (path : String)
```

Set the current command path for argument registration

<details>
<summary>Source</summary>

```rust
pub fn set_current_command_path(path: String) {
    LAST_COMMAND_PATH.with(|p| *p.borrow_mut() = Some(path));
}
```

</details>



### `fn get_current_command_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn get_current_command_path () -> Option < String >
```

Get the current command path for argument registration

<details>
<summary>Source</summary>

```rust
pub fn get_current_command_path() -> Option<String> {
    LAST_COMMAND_PATH.with(|p| p.borrow().clone())
}
```

</details>



### `fn generate_command_path_key`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn generate_command_path_key (command : & AngrealCommand) -> String
```

Generate a full path key for a command based on its group hierarchy

<details>
<summary>Source</summary>

```rust
pub fn generate_command_path_key(command: &AngrealCommand) -> String {
    let path = match &command.group {
        None => command.name.clone(),
        Some(groups) => {
            let group_path = groups
                .iter()
                .map(|g| g.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            format!("{}.{}", group_path, command.name)
        }
    };
    // Strip any leading dots that might have been introduced
    path.strip_prefix('.').unwrap_or(&path).to_string()
}
```

</details>



### `fn generate_path_key_from_parts`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn generate_path_key_from_parts (groups : & [String] , command_name : & str) -> String
```

Generate a full path key for a command based on group names and command name

<details>
<summary>Source</summary>

```rust
pub fn generate_path_key_from_parts(groups: &[String], command_name: &str) -> String {
    let path = if groups.is_empty() {
        command_name.to_string()
    } else {
        format!("{}.{}", groups.join("."), command_name)
    };
    // Strip any leading dots that might have been introduced
    path.strip_prefix('.').unwrap_or(&path).to_string()
}
```

</details>
