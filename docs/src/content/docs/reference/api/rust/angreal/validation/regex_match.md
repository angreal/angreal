---
title: "angreal::validation::regex_match"
description: "Validation for regex pattern matching"
sidebar:
  order: 52
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Validation for regex pattern matching

## Functions

### <span id="fn.regex_match"></span>`fn regex_match`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn regex_match (input : & str , pattern : & Value) -> Result < () , String >
```

Validate that input matches the regex pattern

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `pattern` | `-` | The regex pattern to match against |


**Returns:**

* `Ok(())` if input matches the pattern * `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn regex_match(input: &str, pattern: &Value) -> Result<(), String> {
    if let Some(pattern_str) = pattern.as_str() {
        match Regex::new(pattern_str) {
            Ok(regex) => {
                if regex.is_match(input) {
                    Ok(())
                } else {
                    Err(format!("Input does not match pattern '{}'", pattern_str))
                }
            }
            Err(e) => Err(format!("Invalid regex pattern: {}", e)),
        }
    } else {
        Err("Invalid regex_match format. Expected string.".to_string())
    }
}
```

</details>
