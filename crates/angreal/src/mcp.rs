//! Minimal MCP (Model Context Protocol) stdio server.
//!
//! Provides persistent system-level instructions to AI agents by serving
//! the angreal task tree and usage rules via the MCP protocol. This server
//! exposes no tools — its sole purpose is context injection.

use crate::builder::command_tree::CommandNode;
use crate::builder::tree_output;
use crate::task::ANGREAL_TASKS;

use std::io::{self, BufRead, Write};

use serde_json::{json, Value};

/// Run the MCP stdio server.
pub fn serve() {
    let instructions = build_instructions();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        let msg: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(response) = handle_message(&msg, &instructions) {
            let _ = writeln!(stdout, "{}", response);
            let _ = stdout.flush();
        }
    }
}

fn handle_message(msg: &Value, instructions: &str) -> Option<String> {
    let method = msg.get("method")?.as_str()?;
    let id = msg.get("id")?;

    // Notifications have no id — ignore them
    if id.is_null() {
        return None;
    }

    let result = match method {
        "initialize" => json!({
            "id": id,
            "jsonrpc": "2.0",
            "result": {
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": "angreal",
                    "version": "1.0.0",
                    "description": "Angreal project task runner context",
                },
                "capabilities": {},
                "instructions": instructions,
            }
        }),
        "tools/list" => json!({
            "id": id,
            "jsonrpc": "2.0",
            "result": { "tools": [] }
        }),
        "resources/list" => json!({
            "id": id,
            "jsonrpc": "2.0",
            "result": { "resources": [] }
        }),
        "prompts/list" => json!({
            "id": id,
            "jsonrpc": "2.0",
            "result": { "prompts": [] }
        }),
        "ping" => json!({
            "id": id,
            "jsonrpc": "2.0",
            "result": {}
        }),
        _ => json!({
            "id": id,
            "jsonrpc": "2.0",
            "error": {
                "code": -32601,
                "message": format!("Method not found: {}", method),
            }
        }),
    };

    Some(result.to_string())
}

fn build_instructions() -> String {
    let tree = capture_tree_output();

    format!(
        r#"# Angreal Project Task Runner

**ALWAYS use `angreal <task>` instead of manual equivalents.** Angreal tasks encode project-specific knowledge — correct flags, paths, environment setup, and proper sequencing.

## Decision Rule

Before running ANY build, test, lint, docs, or deploy command:
1. Check the task list below
2. If an angreal task exists for the operation, **USE IT**
3. Only use manual commands when no angreal task covers the need

## Examples of What NOT To Do

- Do NOT run `pytest` directly → use `angreal test python`
- Do NOT run `cargo test` directly → use `angreal test rust`
- Do NOT run `hugo` directly → use `angreal docs build` or `angreal docs serve`
- Do NOT run `pip install` / `maturin develop` directly → `angreal test python` handles the build

## Available Tasks

```
{tree}
```

## Running Tasks

- `angreal <command>` — run a task
- `angreal <command> --help` — get help for a specific task
- `angreal tree` — list all available tasks
"#,
        tree = tree
    )
}

/// Capture tree output as a string instead of printing to stdout.
fn capture_tree_output() -> String {
    let mut root = CommandNode::new_group("angreal".to_string(), None);
    for (_, cmd) in ANGREAL_TASKS.lock().unwrap().iter() {
        root.add_command(cmd.clone());
    }

    let mut buf = Vec::new();
    tree_to_string(&root, true, 0, &[], &mut buf);
    String::from_utf8(buf).unwrap_or_default()
}

/// Write the tree to a buffer (mirrors tree_output::print_tree logic).
fn tree_to_string(
    node: &CommandNode,
    long: bool,
    depth: usize,
    parent_path: &[String],
    buf: &mut Vec<u8>,
) {
    use crate::task::ANGREAL_ARGS;
    use std::fmt::Write as FmtWrite;

    let indent = "  ".repeat(depth);

    // Skip the root "angreal" node
    if depth == 0 && node.name == "angreal" {
        let mut children: Vec<_> = node.children.iter().collect();
        children.sort_by_key(|(name, _)| *name);
        for (_, child) in children {
            tree_to_string(child, long, depth, parent_path, buf);
        }
        return;
    }

    if let Some(cmd) = &node.command {
        let command_path = if parent_path.is_empty() {
            node.name.clone()
        } else {
            format!("{}.{}", parent_path.join("."), node.name)
        };

        let args = ANGREAL_ARGS
            .lock()
            .unwrap()
            .get(&command_path)
            .cloned()
            .unwrap_or_default();

        let arg_sig = tree_output::format_arg_signature_pub(&args);
        let about = cmd.about.as_deref().unwrap_or("");

        let mut line = String::new();
        if arg_sig.is_empty() {
            let _ = write!(line, "{}{} - {}", indent, node.name, about);
        } else {
            let _ = write!(line, "{}{} {} - {}", indent, node.name, arg_sig, about);
        }
        buf.extend_from_slice(line.as_bytes());
        buf.push(b'\n');

        if long {
            if let Some(tool) = &cmd.tool {
                let tool_indent = "  ".repeat(depth + 1);
                buf.push(b'\n');
                for tline in tool.description.lines() {
                    if tline.trim().is_empty() {
                        buf.push(b'\n');
                    } else {
                        buf.extend_from_slice(format!("{}{}\n", tool_indent, tline).as_bytes());
                    }
                }
                buf.extend_from_slice(
                    format!("{}Risk level: {}\n\n", tool_indent, tool.risk_level).as_bytes(),
                );
            }
        }
    } else {
        let about = node.about.as_deref().unwrap_or("");
        let header = if about.is_empty() {
            format!("{}{}:", indent, node.name)
        } else {
            format!("{}{}: {}", indent, node.name, about)
        };
        buf.extend_from_slice(header.as_bytes());
        buf.push(b'\n');

        let mut new_parent_path = parent_path.to_vec();
        new_parent_path.push(node.name.clone());

        let mut children: Vec<_> = node.children.iter().collect();
        children.sort_by_key(|(name, _)| *name);
        for (_, child) in children {
            tree_to_string(child, long, depth + 1, &new_parent_path, buf);
        }
    }
}
