//! Validation for allowed values
use toml::Value;

/// Validate that input is one of the allowed values
///
/// # Arguments
/// * `input` - The user input to validate
/// * `allowed_values` - A TOML array of allowed values
///
/// # Returns
/// * `Ok(())` if input matches one of the allowed values
/// * `Err(message)` with descriptive error message if validation fails
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

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_allowed_values_valid() {
        // Define some allowed values
        let allowed = Value::Array(vec![
            Value::String("admin".to_string()),
            Value::String("user".to_string()),
            Value::String("guest".to_string()),
        ]);

        // Test valid inputs
        assert!(allowed_values("admin", &allowed).is_ok());
        assert!(allowed_values("user", &allowed).is_ok());
        assert!(allowed_values("guest", &allowed).is_ok());
    }

    #[test]
    fn test_allowed_values_invalid() {
        // Define some allowed values
        let allowed = Value::Array(vec![
            Value::String("admin".to_string()),
            Value::String("user".to_string()),
            Value::String("guest".to_string()),
        ]);

        // Test invalid input
        let result = allowed_values("moderator", &allowed);
        assert!(result.is_err());
        assert!(result
            .clone()
            .unwrap_err()
            .contains("Input must be one of:"));
        assert!(result.unwrap_err().contains("admin"));
    }

    #[test]
    fn test_allowed_values_numbers() {
        // Define some allowed values with mixed types
        let allowed = Value::Array(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ]);

        // Test valid inputs
        assert!(allowed_values("1", &allowed).is_ok());
        assert!(allowed_values("2", &allowed).is_ok());

        // Test invalid input
        let result = allowed_values("4", &allowed);
        assert!(result.is_err());
        // Additional checks if needed:
        // assert!(result.clone().unwrap_err().contains("Input must be one of:"));
    }
}
