---
title: "angreal::builder"
description: "The angreal app builder"
editUrl: false
sidebar:
  order: 10
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

The angreal app builder

## Functions

### <span id="fn.select_args"></span><code style="font-size: 0.85em">fn select_​args</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn select_args(command_path: &str) -> Vec<AngrealArg>
```

Get the args for a given command using full command path.

<details>
<summary>Source</summary>

```rust
pub fn select_args(command_path: &str) -> Vec<AngrealArg> {
    ANGREAL_ARGS
        .lock()
        .unwrap()
        .get(command_path)
        .cloned()
        .unwrap_or_default()
}
```

</details>

### <span id="fn.build_app"></span><code style="font-size: 0.85em">fn build_​app</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn build_app(in_angreal_project: bool) -> App<'static>
```

Build the final CLI from the registered tasks

<details>
<summary>Source</summary>

```rust
pub fn build_app(in_angreal_project: bool) -> App<'static> {
    // Build the initial App with angreal sub commands
    let mut app = base_app_setup();

    // Always add completion subcommands (hidden), alias management, and completion management
    app = add_completion_subcommands(app);
    app = add_alias_subcommand(app);
    app = add_completion_subcommand(app);

    if in_angreal_project {
        app = add_tree_subcommand(app);
        app = add_mcp_subcommand(app);
        app = add_project_subcommands(app);
    } else {
        app = add_init_subcommand(app);
    }
    app
}
```

</details>
