//! Validation for maximum value
use toml::Value;

/// Validate that input is less than or equal to the maximum value
///
/// # Arguments
/// * `input` - The user input to validate
/// * `max_value` - The maximum allowed value
///
/// # Returns
/// * `Ok(())` if input is <= max_value
/// * `Err(message)` with descriptive error message if validation fails
pub fn max(input: &str, max_value: &Value) -> Result<(), String> {
    if let Some(max_int) = max_value.as_integer() {
        // Try to parse input as integer
        if let Ok(input_int) = input.parse::<i64>() {
            if input_int <= max_int {
                return Ok(());
            } else {
                return Err(format!("Input must be less than or equal to {}", max_int));
            }
        }
    }

    if let Some(max_float) = max_value.as_float() {
        // Try to parse input as float
        if let Ok(input_float) = input.parse::<f64>() {
            if input_float <= max_float {
                return Ok(());
            } else {
                return Err(format!("Input must be less than or equal to {}", max_float));
            }
        }
    }

    // If we get here, either the max_value wasn't a number or the input couldn't be parsed
    Err(format!(
        "Invalid max validation. Either '{}' is not a number or the maximum value is not specified correctly",
        input
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_max_integers() {
        // Integer validation
        let max_value = Value::Integer(10);

        // Valid inputs
        assert!(max(10.to_string().as_str(), &max_value).is_ok());
        assert!(max(5.to_string().as_str(), &max_value).is_ok());

        // Invalid inputs
        let result = max(11.to_string().as_str(), &max_value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("must be less than or equal to 10"));
    }

    #[test]
    fn test_max_floats() {
        // Float validation
        let max_value = Value::Float(7.5);

        // Valid inputs
        assert!(max(7.5.to_string().as_str(), &max_value).is_ok());
        assert!(max(7.0.to_string().as_str(), &max_value).is_ok());

        // Invalid inputs
        let result = max(8.0.to_string().as_str(), &max_value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("must be less than or equal to 7.5"));
    }
}
