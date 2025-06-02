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

    /// Display the command tree in human-readable format
    pub fn display_tree(&self) -> String {
        let mut output = String::new();
        self.display_tree_recursive(&mut output, "", true);
        output
    }

    /// Recursive helper for tree display
    fn display_tree_recursive(&self, output: &mut String, prefix: &str, is_last: bool) {
        // Don't show the root node
        if self.name != "root" {
            let connector = if is_last { "└── " } else { "├── " };
            output.push_str(&format!("{}{}{}", prefix, connector, self.name));

            // Add description if available
            if let Some(about) = &self.about {
                // Split the about text into lines to handle arguments section
                let lines: Vec<&str> = about.split('\n').collect();
                
                // Add the main description (first line)
                if !lines.is_empty() {
                    output.push_str(&format!(" - {}", lines[0]));
                }
                output.push('\n');

                // Add argument information as sub-directories
                let mut current_section = String::new();
                let mut args_in_section = Vec::new();

                for line in lines.iter().skip(1) {
                    let line = line.trim();
                    if line.starts_with("Arguments:") {
                        // Start new section
                        if !args_in_section.is_empty() {
                            // Print previous section
                            output.push_str(&format!("{}{}    {}\n", 
                                prefix, 
                                if is_last { " " } else { "│" },
                                current_section
                            ));
                            for arg in &args_in_section {
                                output.push_str(&format!("{}{}    {}\n", 
                                    prefix, 
                                    if is_last { " " } else { "│" },
                                    arg
                                ));
                            }
                            args_in_section.clear();
                        }
                        current_section = line.to_string();
                    } else if line.starts_with("  ") {
                        // This is an argument line
                        args_in_section.push(line.to_string());
                    }
                }

                // Print final section if any
                if !args_in_section.is_empty() {
                    output.push_str(&format!("{}{}    {}\n", 
                        prefix, 
                        if is_last { " " } else { "│" },
                        current_section
                    ));
                    for arg in &args_in_section {
                        output.push_str(&format!("{}{}    {}\n", 
                            prefix, 
                            if is_last { " " } else { "│" },
                            arg
                        ));
                    }
                }
            } else if let Some(command) = &self.command {
                if let Some(about) = &command.about {
                    output.push_str(&format!(" - {}", about));
                }
                output.push('\n');
            }
        }

        // Sort children for consistent output
        let mut children: Vec<_> = self.children.iter().collect();
        children.sort_by_key(|(name, _)| *name);

        for (i, (_, child)) in children.iter().enumerate() {
            let is_last_child = i == children.len() - 1;
            let new_prefix = if self.name == "root" {
                String::new()
            } else {
                format!("{}{}    ", prefix, if is_last { " " } else { "│" })
            };
            child.display_tree_recursive(output, &new_prefix, is_last_child);
        }
    }

    /// Convert to JSON format
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
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
        Python::with_gil(|py| {
            let name = "test_cmd".to_string();
            let about = Some("Test command".to_string());
            let func = py.None();

            let command = AngrealCommand {
                name: name.clone(),
                about: about.clone(),
                long_about: None,
                group: None,
                func,
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
        Python::with_gil(|py| {
            let mut root = CommandNode::new_group("root".to_string(), None);
            let command_name = "test".to_string();
            let command_about = Some("Test command".to_string());

            let command = AngrealCommand {
                name: command_name.clone(),
                about: command_about.clone(),
                long_about: None,
                group: None,
                func: py.None(),
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
        Python::with_gil(|py| {
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
    fn test_display_tree_with_arguments() {
        Python::with_gil(|py| {
            let mut root = CommandNode::new_group("root".to_string(), None);
            
            // Create a command with arguments
            let command = AngrealCommand {
                name: "test_cmd".to_string(),
                about: Some("Test command with arguments".to_string()),
                long_about: None,
                group: None,
                func: py.None(),
            };
            
            let mut command_node = CommandNode::new_command("test_cmd".to_string(), command);
            
            // Add required arguments group
            let mut required_group = CommandNode::new_group("required arguments".to_string(), Some("Required arguments for this command".to_string()));
            required_group.add_command(AngrealCommand {
                name: "--target".to_string(),
                about: Some("[str] - Build target to use".to_string()),
                long_about: None,
                group: None,
                func: py.None(),
            });
            command_node.children.insert("required arguments".to_string(), required_group);
            
            // Add optional arguments group
            let mut optional_group = CommandNode::new_group("optional arguments".to_string(), Some("Optional arguments for this command".to_string()));
            optional_group.add_command(AngrealCommand {
                name: "--debug".to_string(),
                about: Some("[bool] - Enable debug mode".to_string()),
                long_about: None,
                group: None,
                func: py.None(),
            });
            command_node.children.insert("optional arguments".to_string(), optional_group);
            
            root.children.insert("test_cmd".to_string(), command_node);
            
            let expected = "└── test_cmd - Test command with arguments\n     ├── optional arguments - Optional arguments for this command\n     │    └── --debug - [bool] - Enable debug mode\n     └── required arguments - Required arguments for this command\n          └── --target - [str] - Build target to use\n";
            
            assert_eq!(root.display_tree(), expected);
        });
    }
}
