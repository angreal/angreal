---
title: "angreal::validation"
description: "Validation utilities for template variables"
sidebar:
  order: 46
  hidden: true
---

<span class="plissken-badge plissken-badge-source" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-orange-low); color: var(--sl-color-orange-high);">Rust</span>


Validation utilities for template variables

## Functions

### <span id="fn.validate_input"></span>`fn validate_input`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.05em 0.6em; font-family: var(--sl-font-mono); font-size: var(--sl-text-xs); line-height: 1.6; border-radius: var(--radius-pill, 1rem); vertical-align: middle; background: var(--sl-color-green-low); color: var(--sl-color-green-high);">pub</span>


```rust
fn validate_input (input : & str , key : & str , validation_table : & Table) -> Result < () , String >
```

Validate input against validation rules defined in TOML

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `key` | `-` | The key/variable name being validated |
| `validation_table` | `-` | The validation table from TOML containing rules |


**Returns:**

* `Ok(())` if validation passes * `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn validate_input(input: &str, key: &str, validation_table: &Table) -> Result<(), String> {
    debug!("Validating input '{}' for key '{}'", input, key);

    // Check if we have any validation rules for this key using dotted notation
    let key_prefix = format!("{key}.");
    debug!("Looking for validation rules with prefix '{}'", key_prefix);

    // Find all validation rules for this key
    let validation_rules: Vec<(&String, &Value)> = validation_table
        .iter()
        .filter(|(rule_key, _)| rule_key.starts_with(&key_prefix))
        .collect();

    if validation_rules.is_empty() {
        debug!("No validation rules found for key '{}'", key);
        return Ok(());
    }

    debug!(
        "Found {} validation rules for key '{}'",
        validation_rules.len(),
        key
    );

    // Apply each validation rule
    for (rule_key, rule_value) in validation_rules {
        // Extract method name from the dotted key
        let method = rule_key.strip_prefix(&key_prefix).unwrap();
        debug!(
            "Applying validation rule '{}' with value {:?}",
            method, rule_value
        );

        // Apply the validation method
        match method {
            "allowed_values" => {
                debug!("Validating against allowed values");
                allowed_values(input, rule_value)?;
                debug!("Input passed allowed_values validation");
            }
            "min" => {
                debug!("Validating minimum value");
                min(input, rule_value)?;
                debug!("Input passed min validation");
            }
            "max" => {
                debug!("Validating maximum value");
                max(input, rule_value)?;
                debug!("Input passed max validation");
            }
            "regex_match" => {
                debug!("Validating regex pattern match");
                regex_match(input, rule_value)?;
                debug!("Input passed regex_match validation");
            }
            "not_empty" => {
                debug!("Validating non-empty input");
                not_empty(input, rule_value)?;
                debug!("Input passed not_empty validation");
            }
            "type" => {
                debug!("Validating type check");
                type_check(input, rule_value)?;
                debug!("Input passed type validation");
            }
            "length_min" => {
                debug!("Validating minimum length");
                length_min(input, rule_value)?;
                debug!("Input passed length_min validation");
            }
            "length_max" => {
                debug!("Validating maximum length");
                length_max(input, rule_value)?;
                debug!("Input passed length_max validation");
            }
            // Additional validation methods can be added here
            _ => {
                debug!("Unknown validation method: {}", method);
            }
        }
    }

    debug!("All validation rules passed for key '{}'", key);
    Ok(())
}
```

</details>
