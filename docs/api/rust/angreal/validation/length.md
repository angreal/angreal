# length


Validation for string length

## Functions

### `fn length_min`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn length_min (input : & str , min_length : & Value) -> Result < () , String >
```

Validate that input string has at least the minimum length

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `min_length` | `-` | The minimum required length |


**Returns:**

* `Ok(())` if input length is >= min_length * `Err(message)` with descriptive error message if validation fails

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



### `fn length_max`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


```rust
fn length_max (input : & str , max_length : & Value) -> Result < () , String >
```

Validate that input string does not exceed maximum length

**Parameters:**

| Name | Type | Description |
|------|------|-------------|
| `input` | `-` | The user input to validate |
| `max_length` | `-` | The maximum allowed length |


**Returns:**

* `Ok(())` if input length is <= max_length * `Err(message)` with descriptive error message if validation fails

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
