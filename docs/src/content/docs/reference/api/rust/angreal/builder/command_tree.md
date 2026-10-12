---
title: "angreal::builder::command_tree"
description: "API reference of the Rust module angreal::builder::command_tree."
sidebar:
  order: 14
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


## Structs

### <span id="struct.CommandNode"></span>`struct CommandNode`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`

Represents a node in the command tree

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` | Name of the command or group |
| `command` | `Option < SerializableCommand >` | Command metadata if this is a leaf node (actual command) |
| `about` | `Option < String >` | About text for groups |
| `children` | `HashMap < String , CommandNode >` | Child nodes (subgroups and commands) |

#### Methods

##### <span id="CommandNode.new_group"></span>`new_group`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn new_group (name : String , about : Option < String >) -> Self
```

Create a new group node

<details>
<summary>Source</summary>

```rust
    pub fn new_group(name: String, about: Option<String>) -> Self {
        CommandNode {
            name,
            command: None,
            about,
            children: HashMap::new(),
        }
    }
```

</details>



##### <span id="CommandNode.new_command"></span>`new_command`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn new_command (name : String , command : AngrealCommand) -> Self
```

Create a new command node

<details>
<summary>Source</summary>

```rust
    pub fn new_command(name: String, command: AngrealCommand) -> Self {
        let serializable_command = SerializableCommand {
            name: command.name.clone(),
            about: command.about.clone(),
            long_about: command.long_about.clone(),
            group: command
                .group
                .as_ref()
                .map(|groups| groups.iter().map(|g| g.name.clone()).collect()),
            tool: command.tool.as_ref().map(|t| SerializableToolDescription {
                description: t.description.clone(),
                risk_level: t.risk_level.clone(),
            }),
            registry_key: command.registry_key.clone(),
        };

        CommandNode {
            name,
            command: Some(serializable_command),
            about: command.about,
            children: HashMap::new(),
        }
    }
```

</details>



##### <span id="CommandNode.add_command"></span>`add_command`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn add_command (& mut self , command : AngrealCommand)
```

Add a command to this node or its children

<details>
<summary>Source</summary>

```rust
    pub fn add_command(&mut self, command: AngrealCommand) {
        match &command.group {
            None => {
                // This is a top-level command
                self.children.insert(
                    command.name.clone(),
                    CommandNode::new_command(command.name.clone(), command),
                );
            }
            Some(groups) => {
                // Navigate/create the group hierarchy
                let mut current = self;
                for group in groups {
                    current = current
                        .children
                        .entry(group.name.clone())
                        .or_insert_with(|| {
                            CommandNode::new_group(group.name.clone(), group.about.clone())
                        });
                }
                // Add the command to the final group
                current.children.insert(
                    command.name.clone(),
                    CommandNode::new_command(command.name.clone(), command),
                );
            }
        }
    }
```

</details>



##### <span id="CommandNode.to_project_schema"></span>`to_project_schema`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn to_project_schema (& self , angreal_root : String , angreal_version : String ,) -> ProjectSchema
```

Convert to new schema format

<details>
<summary>Source</summary>

```rust
    pub fn to_project_schema(
        &self,
        angreal_root: String,
        angreal_version: String,
    ) -> ProjectSchema {
        let mut commands = Vec::new();
        self.collect_commands(&mut commands, vec![]);

        ProjectSchema {
            angreal_root,
            angreal_version,
            commands,
        }
    }
```

</details>



##### <span id="CommandNode.collect_commands"></span>`collect_commands`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-gray-6); color: var(--sl-color-gray-2);">private</span>


```rust
fn collect_commands (& self , commands : & mut Vec < CommandSchema > , path_segments : Vec < String >)
```

Recursively collect all commands from the tree

<details>
<summary>Source</summary>

```rust
    fn collect_commands(&self, commands: &mut Vec<CommandSchema>, path_segments: Vec<String>) {
        // If this node has a command, add it to the list
        if let Some(command) = &self.command {
            let full_command = if path_segments.is_empty() {
                self.name.clone()
            } else {
                format!("{} {}", path_segments.join(" "), self.name)
            };

            commands.push(CommandSchema {
                command: full_command,
                description: command.about.clone().unwrap_or_default(),
                tool: command.tool.clone(),
                parameters: vec![], // Will be populated by caller
            });
        }

        // Recursively process children (skip argument groups)
        for (child_name, child) in &self.children {
            if !child_name.contains("arguments") {
                let mut new_path = path_segments.clone();
                if self.name != "root" && self.name != "angreal" {
                    new_path.push(self.name.clone());
                }
                child.collect_commands(commands, new_path);
            }
        }
    }
```

</details>



##### <span id="CommandNode.to_json"></span>`to_json`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn to_json (& self) -> Result < String , serde_json :: Error >
```

Convert to JSON format (legacy)

<details>
<summary>Source</summary>

```rust
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
```

</details>



##### <span id="CommandNode.to_schema_json"></span>`to_schema_json`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn to_schema_json (& self , angreal_root : String , angreal_version : String ,) -> Result < String , serde_json :: Error >
```

Convert to new schema JSON format

<details>
<summary>Source</summary>

```rust
    pub fn to_schema_json(
        &self,
        angreal_root: String,
        angreal_version: String,
    ) -> Result<String, serde_json::Error> {
        let schema = self.to_project_schema(angreal_root, angreal_version);
        serde_json::to_string_pretty(&schema)
    }
```

</details>





### <span id="struct.SerializableCommand"></span>`struct SerializableCommand`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`

Serializable version of AngrealCommand for JSON output

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `about` | `Option < String >` |  |
| `long_about` | `Option < String >` |  |
| `group` | `Option < Vec < String > >` |  |
| `tool` | `Option < SerializableToolDescription >` |  |
| `registry_key` | `Option < String >` | Internal unique registry key for argument lookup |



### <span id="struct.SerializableToolDescription"></span>`struct SerializableToolDescription`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`

Serializable version of ToolDescription for JSON output

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `description` | `String` |  |
| `risk_level` | `String` |  |



### <span id="struct.ProjectSchema"></span>`struct ProjectSchema`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`

Tree schema for AI agent consumption

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `angreal_root` | `String` |  |
| `angreal_version` | `String` |  |
| `commands` | `Vec < CommandSchema >` |  |



### <span id="struct.CommandSchema"></span>`struct CommandSchema`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `command` | `String` |  |
| `description` | `String` |  |
| `tool` | `Option < SerializableToolDescription >` |  |
| `parameters` | `Vec < ParameterSchema >` |  |



### <span id="struct.ParameterSchema"></span>`struct ParameterSchema`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


**Derives:** `Debug`, `Clone`, `Serialize`

#### Fields

| Name | Type | Description |
|------|------|-------------|
| `name` | `String` |  |
| `flag` | `Option < String >` |  |
| `param_type` | `String` |  |
| `required` | `bool` |  |
| `description` | `Option < String >` |  |
