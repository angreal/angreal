# not_empty


Validation for non-empty values

## Functions

### `fn not_empty`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn not_empty (input : & str , value : & Value) -> Result < () , String >
```

Validate that input is not empty

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `value` | `-` | The validation value (true/false) |


**Returns:**

* `Ok(())` if validation passes * `Err(message)` with descriptive error message if validation fails

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
