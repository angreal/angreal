use crate::task::AngrealCommand;
use std::collections::HashMap;

/// Represents a node in the command tree
#[derive(Debug, Clone)]
pub struct CommandNode {
    /// Name of the command or group
    pub name: String,
    /// Command metadata if this is a leaf node (actual command)
    pub command: Option<AngrealCommand>,
    /// About text for groups
    pub about: Option<String>,
    /// Child nodes (subgroups and commands)
    pub children: HashMap<String, CommandNode>,
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
        CommandNode {
            name,
            command: Some(command.clone()),
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
}
