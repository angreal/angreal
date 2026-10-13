---
title: "angreal::completion::templates"
description: "Template discovery for shell completion"
editUrl: false
sidebar:
  order: 15
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Template discovery for shell completion

Provides template suggestions for `angreal init` command from:
- Local cache (~/.angrealrc/)
- GitHub angreal organization repositories

## Functions

### <span id="fn.get_template_suggestions"></span><code style="font-size: 0.85em">fn get_​template_​suggestions</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn get_template_suggestions() -> Result<Vec<String>>
```

Get template suggestions for completion

<details>
<summary>Source</summary>

```rust
pub fn get_template_suggestions() -> Result<Vec<String>> {
    let mut suggestions = HashSet::new();

    // Add local cached templates
    if let Ok(local_templates) = get_local_templates() {
        suggestions.extend(local_templates);
    }

    // Add GitHub organization templates (with timeout)
    if let Ok(github_templates) = get_github_templates() {
        suggestions.extend(github_templates);
    }

    // Convert to sorted vector
    let mut result: Vec<String> = suggestions.into_iter().collect();
    result.sort();

    Ok(result)
}
```

</details>
