---
title: "angreal::validation::max"
description: "Validation for maximum value"
sidebar:
  order: 49
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Validation for maximum value

## Functions

### <span id="fn.max"></span>`fn max`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn max (input : & str , max_value : & Value) -> Result < () , String >
```

Validate that input is less than or equal to the maximum value

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `max_value` | `-` | The maximum allowed value |


**Returns:**

* `Ok(())` if input is <= max_value * `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn max(input: &str, max_value: &Value) -> Result<(), String> {
    if let Some(max_int) = max_value.as_integer() {
        // Try to parse input as integer
        if let Ok(input_int) = input.parse::<i64>() {
            if input_int <= max_int {
                return Ok(());
            } else {
                return Err(format!("Input must be less than or equal to {}", max_int));
            }
        }
    }

    if let Some(max_float) = max_value.as_float() {
        // Try to parse input as float
        if let Ok(input_float) = input.parse::<f64>() {
            if input_float <= max_float {
                return Ok(());
            } else {
                return Err(format!("Input must be less than or equal to {}", max_float));
            }
        }
    }

    // If we get here, either the max_value wasn't a number or the input couldn't be parsed
    Err(format!(
        "Invalid max validation. Either '{}' is not a number or the maximum value is not specified correctly",
        input
    ))
}
```

</details>
