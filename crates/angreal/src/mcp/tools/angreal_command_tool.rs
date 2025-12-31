use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::IntoPyObjectExt;
use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    schema::{schema_utils::CallToolError, CallToolResult, TextContent},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error};

#[mcp_tool(
    name = "angreal_command",
    description = "Execute an angreal command with arguments",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = true,
    read_only_hint = false
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AngrealCommandTool {
    /// The command path (e.g., "docs.build", "test.rust")
    pub command_path: String,
    /// Command arguments as key-value pairs
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl AngrealCommandTool {
    pub async fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        debug!("Executing angreal command: {}", self.command_path);

        // Ensure Python tasks are initialized
        crate::initialize_python_tasks().map_err(|e| {
            CallToolError::new(std::io::Error::other(format!(
                "Failed to initialize angreal tasks: {}",
                e
            )))
        })?;

        // Get the command from the registry
        let tasks = crate::task::ANGREAL_TASKS.lock().map_err(|e| {
            CallToolError::new(std::io::Error::other(format!(
                "Failed to lock ANGREAL_TASKS: {}",
                e
            )))
        })?;

        let command = tasks.get(&self.command_path).ok_or_else(|| {
            CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Command '{}' not found", self.command_path),
            ))
        })?;

        debug!(
            "Found command '{}', executing with args: {:?}",
            command.name, self.args
        );

        // Get command arguments definition
        let args_registry = crate::task::ANGREAL_ARGS.lock().map_err(|e| {
            CallToolError::new(std::io::Error::other(format!(
                "Failed to lock ANGREAL_ARGS: {}",
                e
            )))
        })?;

        let command_args = args_registry
            .get(&self.command_path)
            .cloned()
            .unwrap_or_default();

        // Execute the command with Python, capturing stdout/stderr
        let result = Python::attach(|py| -> PyResult<(String, String, String)> {
            debug!("Starting Python execution for command: {}", command.name);

            // Import necessary modules for capturing output
            let sys = py.import("sys")?;
            let io = py.import("io")?;

            // Create StringIO objects to capture stdout and stderr
            let stdout_capture = io.call_method0("StringIO")?;
            let stderr_capture = io.call_method0("StringIO")?;

            // Save original stdout/stderr
            let original_stdout = sys.getattr("stdout")?;
            let original_stderr = sys.getattr("stderr")?;

            // Redirect stdout/stderr to our captures
            sys.setattr("stdout", &stdout_capture)?;
            sys.setattr("stderr", &stderr_capture)?;

            let mut kwargs: Vec<(&str, Py<PyAny>)> = Vec::new();

            // Process provided arguments
            if let Some(provided_args) = &self.args {
                for arg in command_args.iter() {
                    let arg_name = &arg.name;

                    if let Some(value) = provided_args.get(arg_name) {
                        // Convert based on the argument's python_type
                        let python_type = arg.python_type.as_deref().unwrap_or("str");
                        let py_value = match python_type {
                            "str" => {
                                if let Some(s) = value.as_str() {
                                    s.into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                } else {
                                    value
                                        .to_string()
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                }
                            }
                            "int" => {
                                if let Some(i) = value.as_i64() {
                                    i.into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                } else if let Some(s) = value.as_str() {
                                    s.parse::<i64>()
                                        .unwrap_or(0)
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                } else {
                                    0.into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                }
                            }
                            "float" => {
                                if let Some(f) = value.as_f64() {
                                    f.into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                } else if let Some(s) = value.as_str() {
                                    s.parse::<f64>()
                                        .unwrap_or(0.0)
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                } else {
                                    0.0.into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                }
                            }
                            "bool" => {
                                if let Some(b) = value.as_bool() {
                                    b.into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                } else if let Some(s) = value.as_str() {
                                    s.parse::<bool>()
                                        .unwrap_or(false)
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                } else {
                                    false
                                        .into_bound_py_any(py)
                                        .expect("Failed to convert to Python")
                                        .unbind()
                                }
                            }
                            _ => value
                                .to_string()
                                .into_bound_py_any(py)
                                .expect("Failed to convert to Python")
                                .unbind(),
                        };

                        kwargs.push((Box::leak(Box::new(arg_name.clone())).as_str(), py_value));
                    } else if arg.is_flag.unwrap_or(false) {
                        // Default false for missing flags
                        kwargs.push((
                            Box::leak(Box::new(arg_name.clone())).as_str(),
                            false
                                .into_bound_py_any(py)
                                .expect("Failed to convert to Python")
                                .unbind(),
                        ));
                    }
                }
            }

            debug!("Calling Python function with {} arguments", kwargs.len());

            // Call the command function
            let kwargs_dict = kwargs.into_py_dict(py)?;
            let result = command.func.call(py, (), Some(&kwargs_dict));

            // Restore original stdout/stderr
            sys.setattr("stdout", original_stdout)?;
            sys.setattr("stderr", original_stderr)?;

            // Get captured output
            let stdout_output = stdout_capture.call_method0("getvalue")?.to_string();
            let stderr_output = stderr_capture.call_method0("getvalue")?.to_string();

            // Handle the command result
            let result_str = match result {
                Ok(result_obj) => {
                    if result_obj.is_none(py) {
                        format!("Command '{}' executed successfully", command.name)
                    } else {
                        result_obj.to_string()
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            };

            Ok((result_str, stdout_output, stderr_output))
        });

        match result {
            Ok((return_value, stdout, stderr)) => {
                debug!("Successfully executed command: {}", self.command_path);

                let response = serde_json::json!({
                    "command": self.command_path,
                    "result": "success",
                    "return_value": return_value,
                    "stdout": stdout,
                    "stderr": stderr,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });

                Ok(CallToolResult::text_content(vec![TextContent::from(
                    serde_json::to_string_pretty(&response).map_err(CallToolError::new)?,
                )]))
            }
            Err(err) => {
                error!("Failed to execute command '{}': {}", self.command_path, err);

                let error_response = serde_json::json!({
                    "command": self.command_path,
                    "result": "error",
                    "error": err.to_string(),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });

                Ok(CallToolResult::text_content(vec![TextContent::from(
                    serde_json::to_string_pretty(&error_response).map_err(CallToolError::new)?,
                )]))
            }
        }
    }
}
