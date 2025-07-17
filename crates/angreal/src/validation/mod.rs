//! Validation utilities for template variables
use log::debug;
use toml::{Table, Value};

// Import validation methods
mod allowed_values;
mod length;
mod max;
mod min;
mod not_empty;
mod regex_match;
mod type_check;

pub use allowed_values::allowed_values;
pub use length::{length_max, length_min};
pub use max::max;
pub use min::min;
pub use not_empty::not_empty;
pub use regex_match::regex_match;
pub use type_check::type_check;

/// Validate input against validation rules defined in TOML
///
/// # Arguments
/// * `input` - The user input to validate
/// * `key` - The key/variable name being validated
/// * `validation_table` - The validation table from TOML containing rules
///
/// # Returns
/// * `Ok(())` if validation passes
/// * `Err(message)` with descriptive error message if validation fails
pub fn validate_input(input: &str, key: &str, validation_table: &Table) -> Result<(), String> {
    debug!("Validating input '{}' for key '{}'", input, key);

    // Check if we have any validation rules for this key using dotted notation
    let key_prefix = format!("{key}.");
    debug!("Looking for validation rules with prefix '{}'", key_prefix);

    // Find all validation rules for this key
    let validation_rules: Vec<(&String, &Value)> = validation_table
        .iter()
        .filter(|(rule_key, _)| rule_key.starts_with(&key_prefix))
        .collect();

    if validation_rules.is_empty() {
        debug!("No validation rules found for key '{}'", key);
        return Ok(());
    }

    debug!(
        "Found {} validation rules for key '{}'",
        validation_rules.len(),
        key
    );

    // Apply each validation rule
    for (rule_key, rule_value) in validation_rules {
        // Extract method name from the dotted key
        let method = rule_key.strip_prefix(&key_prefix).unwrap();
        debug!(
            "Applying validation rule '{}' with value {:?}",
            method, rule_value
        );

        // Apply the validation method
        match method {
            "allowed_values" => {
                debug!("Validating against allowed values");
                allowed_values(input, rule_value)?;
                debug!("Input passed allowed_values validation");
            }
            "min" => {
                debug!("Validating minimum value");
                min(input, rule_value)?;
                debug!("Input passed min validation");
            }
            "max" => {
                debug!("Validating maximum value");
                max(input, rule_value)?;
                debug!("Input passed max validation");
            }
            "regex_match" => {
                debug!("Validating regex pattern match");
                regex_match(input, rule_value)?;
                debug!("Input passed regex_match validation");
            }
            "not_empty" => {
                debug!("Validating non-empty input");
                not_empty(input, rule_value)?;
                debug!("Input passed not_empty validation");
            }
            "type" => {
                debug!("Validating type check");
                type_check(input, rule_value)?;
                debug!("Input passed type validation");
            }
            "length_min" => {
                debug!("Validating minimum length");
                length_min(input, rule_value)?;
                debug!("Input passed length_min validation");
            }
            "length_max" => {
                debug!("Validating maximum length");
                length_max(input, rule_value)?;
                debug!("Input passed length_max validation");
            }
            // Additional validation methods can be added here
            _ => {
                debug!("Unknown validation method: {}", method);
            }
        }
    }

    debug!("All validation rules passed for key '{}'", key);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_validate_input_allowed_values() {
        // Create a test validation table
        let mut validation_table = Table::new();

        // Add allowed_values rule for "role"
        validation_table.insert(
            "role.allowed_values".to_string(),
            Value::Array(vec![
                Value::String("admin".to_string()),
                Value::String("user".to_string()),
                Value::String("guest".to_string()),
            ]),
        );

        // Test valid input
        assert!(validate_input("admin", "role", &validation_table).is_ok());

        // Test invalid input
        let result = validate_input("manager", "role", &validation_table);
        assert!(result.is_err());

        // Test key with no validation rules
        assert!(validate_input("anything", "name", &validation_table).is_ok());
    }

    #[test]
    fn test_validate_input_min_max() {
        // Create a test validation table
        let mut validation_table = Table::new();

        // Add min and max rules for "age"
        validation_table.insert("age.min".to_string(), Value::Integer(18));
        validation_table.insert("age.max".to_string(), Value::Integer(65));

        // Test valid input (in range)
        assert!(validate_input("21", "age", &validation_table).is_ok());
        assert!(validate_input("18", "age", &validation_table).is_ok());
        assert!(validate_input("65", "age", &validation_table).is_ok());

        // Test invalid input (below min)
        let result = validate_input("17", "age", &validation_table);
        assert!(result.is_err());

        // Test invalid input (above max)
        let result = validate_input("66", "age", &validation_table);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_input_regex() {
        // Create a test validation table
        let mut validation_table = Table::new();

        // Add regex_match rule for "email"
        validation_table.insert(
            "email.regex_match".to_string(),
            Value::String(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$".to_string()),
        );

        // Test valid email
        assert!(validate_input("user@example.com", "email", &validation_table).is_ok());

        // Test invalid email
        let result = validate_input("not-an-email", "email", &validation_table);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_input_multiple_rules() {
        // Create a test validation table with multiple rules for one field
        let mut validation_table = Table::new();

        // Add min, max, and allowed_values rules for "score"
        validation_table.insert("score.min".to_string(), Value::Integer(0));
        validation_table.insert("score.max".to_string(), Value::Integer(100));
        validation_table.insert(
            "score.allowed_values".to_string(),
            Value::Array(vec![
                Value::Integer(0),
                Value::Integer(25),
                Value::Integer(50),
                Value::Integer(75),
                Value::Integer(100),
            ]),
        );

        // Test valid input (passes all rules)
        assert!(validate_input("50", "score", &validation_table).is_ok());

        // Test invalid input (passes min/max but not allowed_values)
        let result = validate_input("51", "score", &validation_table);
        assert!(result.is_err());

        // Test invalid input (fails min rule)
        let result = validate_input("-1", "score", &validation_table);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_not_empty() {
        // Create a test validation table
        let mut validation_table = Table::new();

        // Add not_empty rule for "name"
        validation_table.insert("name.not_empty".to_string(), Value::Boolean(true));

        // Test valid input (not empty)
        assert!(validate_input("John", "name", &validation_table).is_ok());

        // Test invalid input (empty)
        let result = validate_input("", "name", &validation_table);
        assert!(result.is_err());
        assert!(result.clone().unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_type() {
        // Create a test validation table
        let mut validation_table = Table::new();

        // Add type rules
        validation_table.insert("age.type".to_string(), Value::String("integer".to_string()));
        validation_table.insert("price.type".to_string(), Value::String("float".to_string()));
        validation_table.insert(
            "active.type".to_string(),
            Value::String("boolean".to_string()),
        );

        // Test valid inputs
        assert!(validate_input("25", "age", &validation_table).is_ok());
        assert!(validate_input("19.99", "price", &validation_table).is_ok());
        assert!(validate_input("true", "active", &validation_table).is_ok());

        // Test invalid inputs
        let result = validate_input("twenty", "age", &validation_table);
        assert!(result.is_err());

        let result = validate_input("nineteen", "price", &validation_table);
        assert!(result.is_err());

        let result = validate_input("yes", "active", &validation_table);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_length() {
        // Create a test validation table
        let mut validation_table = Table::new();

        // Add length rules
        validation_table.insert("username.length_min".to_string(), Value::Integer(3));
        validation_table.insert("username.length_max".to_string(), Value::Integer(20));
        validation_table.insert("code.length_min".to_string(), Value::Integer(6));
        validation_table.insert("code.length_max".to_string(), Value::Integer(6));

        // Test valid inputs
        assert!(validate_input("user123", "username", &validation_table).is_ok());
        assert!(validate_input("123456", "code", &validation_table).is_ok());

        // Test invalid inputs (too short)
        let result = validate_input("us", "username", &validation_table);
        assert!(result.is_err());
        assert!(result
            .clone()
            .unwrap_err()
            .contains("at least 3 characters"));

        // Test invalid inputs (too long)
        let result = validate_input("a_very_long_username_1234", "username", &validation_table);
        assert!(result.is_err());
        assert!(result
            .clone()
            .unwrap_err()
            .contains("not exceed 20 characters"));

        // Test invalid inputs (wrong length for fixed-length code)
        let result = validate_input("12345", "code", &validation_table);
        assert!(result.is_err());

        let result = validate_input("1234567", "code", &validation_table);
        assert!(result.is_err());
    }
}
