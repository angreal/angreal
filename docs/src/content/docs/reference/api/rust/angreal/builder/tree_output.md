---
title: "angreal::builder::tree_output"
description: "Human-readable tree output for angreal commands."
editUrl: false
sidebar:
  order: 12
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Human-readable tree output for angreal commands.

Provides formatted display of the command tree with two modes:
- Short: commands with arguments and descriptions
- Long: includes full ToolDescription prose

## Functions

### <span id="fn.format_arg_signature_pub"></span><code style="font-size: 0.85em">fn format_​arg_​signature_​pub</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn format_arg_signature_pub(args: &[AngrealArg]) -> String
```

Format argument signature for display: [--flag] [--option=<type>]

<details>
<summary>Source</summary>

```rust
pub fn format_arg_signature_pub(args: &[AngrealArg]) -> String {
    format_arg_signature(args)
}
```

</details>

### <span id="fn.print_tree"></span><code style="font-size: 0.85em">fn print_​tree</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn print_tree(node: &CommandNode, long: bool)
```

Print the command tree in human-readable format

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `node` | `&CommandNode` | The root command node to print |
| `long` | `bool` | If true, include full ToolDescription prose |

<details>
<summary>Source</summary>

```rust
pub fn print_tree(node: &CommandNode, long: bool) {
    print_node(node, long, 0, &[]);
}
```

</details>
