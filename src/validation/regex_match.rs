//! Validation for regex pattern matching
use regex::Regex;
use toml::Value;

/// Validate that input matches the regex pattern
///
/// # Arguments
/// * `input` - The user input to validate
/// * `pattern` - The regex pattern to match against
///
/// # Returns
/// * `Ok(())` if input matches the pattern
/// * `Err(message)` with descriptive error message if validation fails
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

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_regex_match_email() {
        // Email pattern
        let pattern =
            Value::String(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$".to_string());

        // Valid inputs
        assert!(regex_match("user@example.com", &pattern).is_ok());
        assert!(regex_match("another.user123@test-domain.co.uk", &pattern).is_ok());

        // Invalid inputs
        let result = regex_match("not_an_email", &pattern);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not match pattern"));
    }

    #[test]
    fn test_regex_match_numeric() {
        // Numbers only pattern
        let pattern = Value::String(r"^\d+$".to_string());

        // Valid inputs
        assert!(regex_match("12345", &pattern).is_ok());

        // Invalid inputs
        let result = regex_match("12a45", &pattern);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_regex() {
        // Invalid regex pattern
        let pattern = Value::String(r"[unclosed".to_string());

        let result = regex_match("test", &pattern);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid regex pattern"));
    }
}
