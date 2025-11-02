use pyo3::types::PyTypeMethods;
use pyo3::{PyErr, Python};
use std::fmt;

/// Formats Python exception information in a more readable way
pub struct PythonErrorFormatter {
    error: PyErr,
}

impl PythonErrorFormatter {
    pub fn new(error: PyErr) -> Self {
        Self { error }
    }

    /// Formats a Python error in a more readable way
    pub fn format(&self) -> String {
        let mut output = String::new();

        let error_message = Python::attach(|py| {
            // Get the exception type and value
            let type_obj = self.error.get_type(py);
            let type_name = type_obj
                .name()
                .map(|n| n.to_string())
                .unwrap_or_else(|_| "Unknown".to_string());

            // Extract the error message
            let value = self.error.value(py).to_string();

            format!("\nError: {}\n{}", type_name, value)
        });

        output.push_str(&error_message);
        output.push('\n');

        // Format traceback in a simplified way
        Python::attach(|py| {
            if let Some(traceback) = self.error.traceback(py) {
                output.push_str("\nTraceback:\n");

                // Just extract the traceback as a string
                let tb_str = format!("  {}", traceback);
                for line in tb_str.lines() {
                    output.push_str(&format!("  {}\n", line));
                }
            }
        });

        output
    }
}

impl fmt::Display for PythonErrorFormatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::prelude::*;

    #[test]
    fn test_error_formatter() {
        Python::attach(|py| {
            // Create a Python error
            let result: PyResult<()> =
                py.run(c"raise ValueError('Test error message')", None, None);
            let error = result.unwrap_err();

            // Format the error
            let formatter = PythonErrorFormatter::new(error);
            let output = formatter.format();

            // Basic verification
            assert!(output.contains("Error: ValueError"));
            assert!(output.contains("Test error message"));
        });
    }
}
