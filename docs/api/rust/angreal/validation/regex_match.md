# regex_match


Validation for regex pattern matching

## Functions

### `fn regex_match`

<span class="plissken-badge plissken-badge-visibility" style="display: inline-block; padding: 0.1em 0.35em; font-size: 0.55em; font-weight: 600; border-radius: 0.2em; vertical-align: middle; background: #4caf50; color: white;">pub</span>


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
