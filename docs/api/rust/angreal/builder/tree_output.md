# tree_output


Human-readable tree output for angreal commands.

Provides formatted display of the command tree with two modes:
- Short: commands with arguments and descriptions
- Long: includes full ToolDescription prose

## Functions

### `fn format_arg_signature`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn format_arg_signature (args : & [AngrealArg]) -> String
```

Format argument signature for display: [--flag] [--option=<type>]

<details>
<summary>Source</summary>

```rust
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
```

</details>



### `fn get_command_path`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn get_command_path (node : & CommandNode , parent_path : & [String]) -> String
```

Get the command path for looking up arguments

<details>
<summary>Source</summary>

```rust
fn get_command_path(node: &CommandNode, parent_path: &[String]) -> String {
    if parent_path.is_empty() {
        node.name.clone()
    } else {
        format!("{}.{}", parent_path.join("."), node.name)
    }
}
```

</details>



### `fn print_tree`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn print_tree (node : & CommandNode , long : bool)
```

Print the command tree in human-readable format

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `node` | `-` | The root command node to print |
| `long` | `-` | If true, include full ToolDescription prose |


<details>
<summary>Source</summary>

```rust
pub fn print_tree(node: &CommandNode, long: bool) {
    print_node(node, long, 0, &[]);
}
```

</details>



### `fn print_node`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: var(--md-default-fg-color--light); color: white;">private</span>


```rust
fn print_node (node : & CommandNode , long : bool , depth : usize , parent_path : & [String])
```

<details>
<summary>Source</summary>

```rust
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
```

</details>
