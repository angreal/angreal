//! Validation for non-empty values
use toml::Value;

/// Validate that input is not empty
///
/// # Arguments
/// * `input` - The user input to validate
/// * `value` - The validation value (true/false)
///
/// # Returns
/// * `Ok(())` if validation passes
/// * `Err(message)` with descriptive error message if validation fails
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

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_not_empty_valid() {
        // Valid input (not empty) with boolean true
        assert!(not_empty("some value", &Value::Boolean(true)).is_ok());

        // Empty input with boolean false (not required)
        assert!(not_empty("", &Value::Boolean(false)).is_ok());
    }

    #[test]
    fn test_not_empty_invalid() {
        // Empty input with boolean true (required)
        let result = not_empty("", &Value::Boolean(true));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));

        // Empty input with non-boolean value (assumed required)
        let result = not_empty("", &Value::String("yes".to_string()));
        assert!(result.is_err());
    }
}
