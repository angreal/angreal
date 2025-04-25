//! Validation for type checking
use toml::Value;

/// Validate that input matches the specified type
///
/// # Arguments
/// * `input` - The user input to validate
/// * `type_value` - The expected type name
///
/// # Returns
/// * `Ok(())` if input can be parsed as the specified type
/// * `Err(message)` with descriptive error message if validation fails
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

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_type_integer() {
        let type_value = Value::String("integer".to_string());

        // Valid integer
        assert!(type_check("123", &type_value).is_ok());
        assert!(type_check("-45", &type_value).is_ok());

        // Invalid integer
        let result = type_check("12.3", &type_value);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be an integer"));

        let result = type_check("abc", &type_value);
        assert!(result.is_err());
    }

    #[test]
    fn test_type_float() {
        let type_value = Value::String("float".to_string());

        // Valid float
        assert!(type_check("123.45", &type_value).is_ok());
        assert!(type_check("-45.67", &type_value).is_ok());
        assert!(type_check("123", &type_value).is_ok()); // Integers are also valid floats

        // Invalid float
        let result = type_check("abc", &type_value);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a number"));
    }

    #[test]
    fn test_type_boolean() {
        let type_value = Value::String("boolean".to_string());

        // Valid boolean
        assert!(type_check("true", &type_value).is_ok());
        assert!(type_check("false", &type_value).is_ok());
        assert!(type_check("TRUE", &type_value).is_ok());
        assert!(type_check("False", &type_value).is_ok());

        // Invalid boolean
        let result = type_check("yes", &type_value);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be a boolean"));
    }

    #[test]
    fn test_type_string() {
        let type_value = Value::String("string".to_string());

        // String validation always passes since all inputs are strings
        assert!(type_check("hello", &type_value).is_ok());
        assert!(type_check("123", &type_value).is_ok());
        assert!(type_check("", &type_value).is_ok());
    }

    #[test]
    fn test_type_unknown() {
        let type_value = Value::String("unknown_type".to_string());

        // Unknown type
        let result = type_check("test", &type_value);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown type"));
    }
}
