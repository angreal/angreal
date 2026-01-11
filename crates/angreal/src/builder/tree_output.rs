//! Human-readable tree output for angreal commands.
//!
//! Provides formatted display of the command tree with two modes:
//! - Short: commands with arguments and descriptions
//! - Long: includes full ToolDescription prose

use crate::builder::command_tree::CommandNode;
use crate::task::{AngrealArg, ANGREAL_ARGS};

/// Format argument signature for display: [--flag] [--option=<type>]
fn format_arg_signature(args: &[AngrealArg]) -> String {
    args.iter()
        .filter_map(|arg| {
            // Prefer long flag, fall back to short
            let flag = arg
                .long
                .as_ref()
                .map(|l| format!("--{}", l))
                .or_else(|| arg.short.map(|s| format!("-{}", s)))?;

            if arg.is_flag.unwrap_or(false) {
                Some(format!("[{}]", flag))
            } else {
                let typ = arg.python_type.as_deref().unwrap_or("str");
                Some(format!("[{}=<{}>]", flag, typ))
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Get the command path for looking up arguments
fn get_command_path(node: &CommandNode, parent_path: &[String]) -> String {
    if parent_path.is_empty() {
        node.name.clone()
    } else {
        format!("{}.{}", parent_path.join("."), node.name)
    }
}

/// Print the command tree in human-readable format
///
/// # Arguments
/// * `node` - The root command node to print
/// * `long` - If true, include full ToolDescription prose
pub fn print_tree(node: &CommandNode, long: bool) {
    print_node(node, long, 0, &[]);
}

fn print_node(node: &CommandNode, long: bool, depth: usize, parent_path: &[String]) {
    let indent = "  ".repeat(depth);

    // Skip the root "angreal" node
    if depth == 0 && node.name == "angreal" {
        // Just print children
        let mut children: Vec<_> = node.children.iter().collect();
        children.sort_by_key(|(name, _)| *name);
        for (_, child) in children {
            print_node(child, long, depth, parent_path);
        }
        return;
    }

    if let Some(cmd) = &node.command {
        // This is a command (leaf node)
        let command_path = get_command_path(node, parent_path);

        // Get arguments for this command
        let args = ANGREAL_ARGS
            .lock()
            .unwrap()
            .get(&command_path)
            .cloned()
            .unwrap_or_default();

        let arg_sig = format_arg_signature(&args);
        let about = cmd.about.as_deref().unwrap_or("");

        if arg_sig.is_empty() {
            println!("{}{} - {}", indent, node.name, about);
        } else {
            println!("{}{} {} - {}", indent, node.name, arg_sig, about);
        }

        // In long mode, print ToolDescription
        if long {
            if let Some(tool) = &cmd.tool {
                let tool_indent = "  ".repeat(depth + 1);
                println!();
                // Print each line of the description with proper indentation
                for line in tool.description.lines() {
                    if line.trim().is_empty() {
                        println!();
                    } else {
                        println!("{}{}", tool_indent, line);
                    }
                }
                println!("{}Risk level: {}", tool_indent, tool.risk_level);
                println!();
            }
        }
    } else {
        // This is a group node - print the group header
        let about = node.about.as_deref().unwrap_or("");
        if about.is_empty() {
            println!("{}{}:", indent, node.name);
        } else {
            println!("{}{}: {}", indent, node.name, about);
        }

        // Build new parent path for children
        let mut new_parent_path = parent_path.to_vec();
        new_parent_path.push(node.name.clone());

        // Print children sorted alphabetically
        let mut children: Vec<_> = node.children.iter().collect();
        children.sort_by_key(|(name, _)| *name);
        for (_, child) in children {
            print_node(child, long, depth + 1, &new_parent_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_arg_signature_flag() {
        let args = vec![AngrealArg {
            name: "verbose".to_string(),
            command_name: "test".to_string(),
            command_path: "test".to_string(),
            is_flag: Some(true),
            long: Some("verbose".to_string()),
            short: Some('v'),
            takes_value: Some(false),
            default_value: None,
            require_equals: None,
            multiple_values: None,
            number_of_values: None,
            max_values: None,
            min_values: None,
            python_type: Some("bool".to_string()),
            long_help: None,
            help: Some("Enable verbose output".to_string()),
            required: Some(false),
        }];

        let sig = format_arg_signature(&args);
        assert_eq!(sig, "[--verbose]");
    }

    #[test]
    fn test_format_arg_signature_value() {
        let args = vec![AngrealArg {
            name: "output".to_string(),
            command_name: "test".to_string(),
            command_path: "test".to_string(),
            is_flag: Some(false),
            long: Some("output".to_string()),
            short: Some('o'),
            takes_value: Some(true),
            default_value: None,
            require_equals: None,
            multiple_values: None,
            number_of_values: None,
            max_values: None,
            min_values: None,
            python_type: Some("str".to_string()),
            long_help: None,
            help: Some("Output file path".to_string()),
            required: Some(false),
        }];

        let sig = format_arg_signature(&args);
        assert_eq!(sig, "[--output=<str>]");
    }

    #[test]
    fn test_format_arg_signature_multiple() {
        let args = vec![
            AngrealArg {
                name: "verbose".to_string(),
                command_name: "test".to_string(),
                command_path: "test".to_string(),
                is_flag: Some(true),
                long: Some("verbose".to_string()),
                short: None,
                takes_value: Some(false),
                default_value: None,
                require_equals: None,
                multiple_values: None,
                number_of_values: None,
                max_values: None,
                min_values: None,
                python_type: Some("bool".to_string()),
                long_help: None,
                help: None,
                required: Some(false),
            },
            AngrealArg {
                name: "count".to_string(),
                command_name: "test".to_string(),
                command_path: "test".to_string(),
                is_flag: Some(false),
                long: Some("count".to_string()),
                short: None,
                takes_value: Some(true),
                default_value: None,
                require_equals: None,
                multiple_values: None,
                number_of_values: None,
                max_values: None,
                min_values: None,
                python_type: Some("int".to_string()),
                long_help: None,
                help: None,
                required: Some(false),
            },
        ];

        let sig = format_arg_signature(&args);
        assert_eq!(sig, "[--verbose] [--count=<int>]");
    }
}
