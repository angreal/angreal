//! Validation for string length
use toml::Value;

/// Validate that input string has at least the minimum length
///
/// # Arguments
/// * `input` - The user input to validate
/// * `min_length` - The minimum required length
///
/// # Returns
/// * `Ok(())` if input length is >= min_length
/// * `Err(message)` with descriptive error message if validation fails
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

/// Validate that input string does not exceed maximum length
///
/// # Arguments
/// * `input` - The user input to validate
/// * `max_length` - The maximum allowed length
///
/// # Returns
/// * `Ok(())` if input length is <= max_length
/// * `Err(message)` with descriptive error message if validation fails
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

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_length_min_valid() {
        // Minimum length 3
        let min_length = Value::Integer(3);

        // Valid inputs (length >= 3)
        assert!(length_min("abc", &min_length).is_ok());
        assert!(length_min("abcdef", &min_length).is_ok());

        // Invalid input (length < 3)
        let result = length_min("ab", &min_length);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least 3 characters"));
    }

    #[test]
    fn test_length_min_invalid_type() {
        // Invalid min_length type
        let min_length = Value::String("3".to_string());

        let result = length_min("test", &min_length);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be an integer"));
    }

    #[test]
    fn test_length_max_valid() {
        // Maximum length 5
        let max_length = Value::Integer(5);

        // Valid inputs (length <= 5)
        assert!(length_max("", &max_length).is_ok());
        assert!(length_max("abc", &max_length).is_ok());
        assert!(length_max("abcde", &max_length).is_ok());

        // Invalid input (length > 5)
        let result = length_max("abcdef", &max_length);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not exceed 5 characters"));
    }

    #[test]
    fn test_length_max_invalid_type() {
        // Invalid max_length type
        let max_length = Value::String("5".to_string());

        let result = length_max("test", &max_length);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be an integer"));
    }

    #[test]
    fn test_length_unicode() {
        // Unicode characters (like emojis) should be counted correctly
        let min_length = Value::Integer(3);
        let max_length = Value::Integer(5);

        // String with 3 characters (including emoji)
        let unicode_string = "a💻c";

        assert!(length_min(unicode_string, &min_length).is_ok());
        assert!(length_max(unicode_string, &max_length).is_ok());
    }
}
