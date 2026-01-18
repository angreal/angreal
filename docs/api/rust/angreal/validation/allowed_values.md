# allowed_values


Validation for allowed values

## Functions

### `fn allowed_values`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn allowed_values (input : & str , allowed_values : & Value) -> Result < () , String >
```

Validate that input is one of the allowed values

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `allowed_values` | `-` | A TOML array of allowed values |


**Returns:**

* `Ok(())` if input matches one of the allowed values * `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn allowed_values(input: &str, allowed_values: &Value) -> Result<(), String> {
    if let Some(values) = allowed_values.as_array() {
        // Check if input matches any of the allowed values
        let input_matches = values.iter().any(|value| {
            if let Some(val_str) = value.as_str() {
                val_str == input
            } else if let Some(val_int) = value.as_integer() {
                input.parse::<i64>().map(|i| i == val_int).unwrap_or(false)
            } else if let Some(val_bool) = value.as_bool() {
                input
                    .parse::<bool>()
                    .map(|b| b == val_bool)
                    .unwrap_or(false)
            } else {
                false
            }
        });

        if input_matches {
            Ok(())
        } else {
            // Format the allowed values for error message
            let values_str = values
                .iter()
                .map(|v| match v {
                    Value::String(s) => format!("\"{}\"", s),
                    _ => v.to_string(),
                })
                .collect::<Vec<String>>()
                .join(", ");

            Err(format!("Input must be one of: {}", values_str))
        }
    } else {
        Err("Invalid allowed_values format. Expected array.".to_string())
    }
}
```

</details>
