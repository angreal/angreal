use crate::task::AngrealCommand;
use serde::Serialize;
use std::collections::HashMap;

/// Represents a node in the command tree
#[derive(Debug, Clone, Serialize)]
pub struct CommandNode {
    /// Name of the command or group
    pub name: String,
    /// Command metadata if this is a leaf node (actual command)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<SerializableCommand>,
    /// About text for groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    /// Child nodes (subgroups and commands)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub children: HashMap<String, CommandNode>,
}

/// Serializable version of AngrealCommand for JSON output
#[derive(Debug, Clone, Serialize)]
pub struct SerializableCommand {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<SerializableToolDescription>,
}

/// Serializable version of ToolDescription for JSON output
#[derive(Debug, Clone, Serialize)]
pub struct SerializableToolDescription {
    pub description: String,
    pub risk_level: String,
}

/// Tree schema for MCP consumption
#[derive(Debug, Clone, Serialize)]
pub struct ProjectSchema {
    pub angreal_root: String,
    pub angreal_version: String,
    pub commands: Vec<CommandSchema>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandSchema {
    pub command: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<SerializableToolDescription>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ParameterSchema>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ParameterSchema {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(rename = "type")]
    pub param_type: String,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CommandNode {
    /// Create a new group node
    pub fn new_group(name: String, about: Option<String>) -> Self {
        CommandNode {
            name,
            command: None,
            about,
            children: HashMap::new(),
        }
    }

    /// Create a new command node
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
        };

        CommandNode {
            name,
            command: Some(serializable_command),
            about: command.about,
            children: HashMap::new(),
        }
    }

    /// Add a command to this node or its children
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

    /// Convert to new schema format
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

    /// Recursively collect all commands from the tree
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

    /// Convert to JSON format (legacy)
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Convert to new schema JSON format
    pub fn to_schema_json(
        &self,
        angreal_root: String,
        angreal_version: String,
    ) -> Result<String, serde_json::Error> {
        let schema = self.to_project_schema(angreal_root, angreal_version);
        serde_json::to_string_pretty(&schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{AngrealCommand, AngrealGroup};
    use pyo3::Python;

    #[test]
    fn test_new_group() {
        let name = "test_group".to_string();
        let about = Some("Test group description".to_string());
        let node = CommandNode::new_group(name.clone(), about.clone());

        assert_eq!(node.name, name);
        assert_eq!(node.about, about);
        assert!(node.command.is_none());
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_new_command() {
        Python::attach(|py| {
            let name = "test_cmd".to_string();
            let about = Some("Test command".to_string());
            let func = py.None();

            let command = AngrealCommand {
                name: name.clone(),
                about: about.clone(),
                long_about: None,
                group: None,
                func,
                tool: None,
            };

            let node = CommandNode::new_command(name.clone(), command);

            // Compare individual fields instead of the whole command
            assert_eq!(node.name, name);
            assert_eq!(node.about, about);
            assert!(node.command.is_some());
            assert!(node.children.is_empty());
        });
    }

    #[test]
    fn test_add_command_top_level() {
        Python::attach(|py| {
            let mut root = CommandNode::new_group("root".to_string(), None);
            let command_name = "test".to_string();
            let command_about = Some("Test command".to_string());

            let command = AngrealCommand {
                name: command_name.clone(),
                about: command_about.clone(),
                long_about: None,
                group: None,
                func: py.None(),
                tool: None,
            };

            root.add_command(command);

            // Verify the command was added correctly
            assert!(root.children.contains_key(&command_name));
            let child = root.children.get(&command_name).unwrap();
            assert_eq!(child.name, command_name);
            assert_eq!(child.about, command_about);
            assert!(child.command.is_some());
        });
    }

    #[test]
    fn test_add_command_nested() {
        Python::attach(|py| {
            let mut root = CommandNode::new_group("root".to_string(), None);

            let group1 = AngrealGroup {
                name: "group1".to_string(),
                about: Some("Group 1".to_string()),
            };

            let group2 = AngrealGroup {
                name: "group2".to_string(),
                about: Some("Group 2".to_string()),
            };

            let command = AngrealCommand {
                name: "nested_cmd".to_string(),
                about: Some("Nested command".to_string()),
                long_about: None,
                group: Some(vec![group1.clone(), group2.clone()]),
                func: py.None(),
                tool: None,
            };

            root.add_command(command);

            // Verify the group hierarchy
            let first_group = root.children.get(&group1.name).unwrap();
            assert_eq!(first_group.name, group1.name);
            assert_eq!(first_group.about, group1.about);

            let second_group = first_group.children.get(&group2.name).unwrap();
            assert_eq!(second_group.name, group2.name);
            assert_eq!(second_group.about, group2.about);

            // Verify the command exists and has correct metadata
            let cmd_node = second_group.children.get("nested_cmd").unwrap();
            assert_eq!(cmd_node.name, "nested_cmd");
            assert_eq!(cmd_node.about, Some("Nested command".to_string()));
            assert!(cmd_node.command.is_some());
        });
    }

    #[test]
    fn test_to_project_schema() {
        Python::attach(|py| {
            let mut root = CommandNode::new_group("angreal".to_string(), None);

            // Create a command
            let command = AngrealCommand {
                name: "test_cmd".to_string(),
                about: Some("Test command".to_string()),
                long_about: None,
                group: Some(vec![crate::task::AngrealGroup {
                    name: "test".to_string(),
                    about: Some("Test group".to_string()),
                }]),
                func: py.None(),
                tool: None,
            };

            root.add_command(command);

            let schema = root.to_project_schema("/test/root".to_string(), "2.4.2".to_string());

            assert_eq!(schema.angreal_root, "/test/root");
            assert_eq!(schema.angreal_version, "2.4.2");
            assert_eq!(schema.commands.len(), 1);
            assert_eq!(schema.commands[0].command, "test test_cmd");
            assert_eq!(schema.commands[0].description, "Test command");
        });
    }
}
