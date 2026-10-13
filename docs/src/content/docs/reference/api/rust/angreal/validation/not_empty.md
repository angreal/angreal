---
title: "angreal::validation::not_empty"
description: "Validation for non-empty values"
editUrl: false
sidebar:
  order: 46
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Validation for non-empty values

## Functions

### <span id="fn.not_empty"></span><code style="font-size: 0.85em">fn not_​empty</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn not_empty(input: &str, value: &Value) -> Result<(), String>
```

Validate that input is not empty

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `&str` | The user input to validate |
| `value` | `&Value` | The validation value (true/false) |

**Returns:**

* `Ok(())` if validation passes
* `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn not_empty(input: &str, value: &Value) -> Result<(), String> {
    // Only validate if the value is true
    if let Some(required) = value.as_bool() {
        if required && input.trim().is_empty() {
            return Err("Input cannot be empty".to_string());
        }
    } else {
        // If the value isn't a boolean, assume the intent is to require non-empty
        if input.trim().is_empty() {
            return Err("Input cannot be empty".to_string());
        }
    }

    Ok(())
}
```

</details>
