//! Validation for minimum value
use toml::Value;

/// Validate that input is greater than or equal to the minimum value
///
/// # Arguments
/// * `input` - The user input to validate
/// * `min_value` - The minimum allowed value
///
/// # Returns
/// * `Ok(())` if input is >= min_value
/// * `Err(message)` with descriptive error message if validation fails
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

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_min_integers() {
        // Integer validation
        let min_value = Value::Integer(5);

        // Valid inputs
        assert!(min(5.to_string().as_str(), &min_value).is_ok());
        assert!(min(10.to_string().as_str(), &min_value).is_ok());

        // Invalid inputs
        let result = min(4.to_string().as_str(), &min_value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("must be greater than or equal to 5"));
    }

    #[test]
    fn test_min_floats() {
        // Float validation
        let min_value = Value::Float(5.5);

        // Valid inputs
        assert!(min(5.5.to_string().as_str(), &min_value).is_ok());
        assert!(min(6.0.to_string().as_str(), &min_value).is_ok());

        // Invalid inputs
        let result = min(5.0.to_string().as_str(), &min_value);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("must be greater than or equal to 5.5"));
    }
}
