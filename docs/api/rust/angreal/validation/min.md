# min


Validation for minimum value

## Functions

### `fn min`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn min (input : & str , min_value : & Value) -> Result < () , String >
```

Validate that input is greater than or equal to the minimum value

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `min_value` | `-` | The minimum allowed value |


**Returns:**

* `Ok(())` if input is >= min_value * `Err(message)` with descriptive error message if validation fails

<details>
<summary>Source</summary>

```rust
pub fn min(input: &str, min_value: &Value) -> Result<(), String> {
    if let Some(min_int) = min_value.as_integer() {
        // Try to parse input as integer
        if let Ok(input_int) = input.parse::<i64>() {
            if input_int >= min_int {
                return Ok(());
            } else {
                return Err(format!(
                    "Input must be greater than or equal to {}",
                    min_int
                ));
            }
        }
    }

    if let Some(min_float) = min_value.as_float() {
        // Try to parse input as float
        if let Ok(input_float) = input.parse::<f64>() {
            if input_float >= min_float {
                return Ok(());
            } else {
                return Err(format!(
                    "Input must be greater than or equal to {}",
                    min_float
                ));
            }
        }
    }

    // If we get here, either the min_value wasn't a number or the input couldn't be parsed
    Err(format!(
        "Invalid min validation. Either '{}' is not a number or the minimum value is not specified correctly",
        input
    ))
}
```

</details>
