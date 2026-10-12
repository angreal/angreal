---
title: "angreal::mcp"
description: "Minimal MCP (Model Context Protocol) stdio server."
editUrl: false
sidebar:
  order: 26
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Minimal MCP (Model Context Protocol) stdio server.

Provides persistent system-level instructions to AI agents by serving
the angreal task tree and usage rules via the MCP protocol. This server
exposes no tools — its sole purpose is context injection.

## Functions

### <span id="fn.serve"></span><code style="font-size: 0.85em">fn serve</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn serve()
```

Run the MCP stdio server.

<details>
<summary>Source</summary>

```rust
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
```

</details>
