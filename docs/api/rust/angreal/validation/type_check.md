# type_check


Validation for type checking

## Functions

### `fn type_check`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn type_check (input : & str , type_value : & Value) -> Result < () , String >
```

Validate that input matches the specified type

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `type_value` | `-` | The expected type name |


**Returns:**

* `Ok(())` if input can be parsed as the specified type * `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn type_check(input: &str, type_value: &Value) -> Result<(), String> {
    if let Some(type_name) = type_value.as_str() {
        match type_name {
            "integer" | "int" => {
                if input.parse::<i64>().is_err() {
                    return Err(format!("Input '{}' must be an integer", input));
                }
            }
            "float" | "number" => {
                if input.parse::<f64>().is_err() {
                    return Err(format!("Input '{}' must be a number", input));
                }
            }
            "boolean" | "bool" => {
                let lower = input.to_lowercase();
                if lower != "true" && lower != "false" {
                    return Err(format!("Input '{}' must be a boolean (true/false)", input));
                }
            }
            "string" => {
                // All inputs are strings, so this always passes
            }
            _ => {
                return Err(format!(
                    "Unknown type '{}' specified for validation",
                    type_name
                ));
            }
        }

        Ok(())
    } else {
        Err("Invalid type validation. Type must be a string.".to_string())
    }
}
```

</details>
