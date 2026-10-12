---
title: "angreal::validation::length"
description: "Validation for string length"
editUrl: false
sidebar:
  order: 43
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>

Validation for string length

## Functions

### <span id="fn.length_min"></span><code style="font-size: 0.85em">fn length_​min</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn length_min(input: &str, min_length: &Value) -> Result<(), String>
```

Validate that input string has at least the minimum length

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `&str` | The user input to validate |
| `min_length` | `&Value` | The minimum required length |

**Returns:**

* `Ok(())` if input length is >= min_length
* `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn length_min(input: &str, min_length: &Value) -> Result<(), String> {
    if let Some(min) = min_length.as_integer() {
        let min_usize = min as usize;
        if input.chars().count() < min_usize {
            return Err(format!(
                "Input must be at least {} characters long",
                min_usize
            ));
        }
        Ok(())
    } else {
        Err("Invalid length_min validation. Min length must be an integer.".to_string())
    }
}
```

</details>

### <span id="fn.length_max"></span><code style="font-size: 0.85em">fn length_​max</code>

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>

```rust
fn length_max(input: &str, max_length: &Value) -> Result<(), String>
```

Validate that input string does not exceed maximum length

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `&str` | The user input to validate |
| `max_length` | `&Value` | The maximum allowed length |

**Returns:**

* `Ok(())` if input length is <= max_length
* `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn length_max(input: &str, max_length: &Value) -> Result<(), String> {
    if let Some(max) = max_length.as_integer() {
        let max_usize = max as usize;
        if input.chars().count() > max_usize {
            return Err(format!("Input must not exceed {} characters", max_usize));
        }
        Ok(())
    } else {
        Err("Invalid length_max validation. Max length must be an integer.".to_string())
    }
}
```

</details>
