use rust_mcp_sdk::schema::{Tool, ToolAnnotations, ToolInputSchema};
use serde_json::json;
use tracing::{debug, warn};

/// Discover angreal commands and convert them to MCP tools
pub fn discover_angreal_commands() -> Vec<Tool> {
    debug!("Starting dynamic discovery of angreal commands");

    // Try to access the ANGREAL_TASKS registry from the angreal crate
    match try_discover_from_registry() {
        Ok(tools) => {
            debug!(
                "Successfully discovered {} tools from ANGREAL_TASKS registry",
                tools.len()
            );
            tools
        }
        Err(e) => {
            warn!("Failed to discover from registry: {}", e);
            // Fallback: Try to discover from filesystem
            discover_from_filesystem()
        }
    }
}

/// Try to discover commands from the ANGREAL_TASKS registry
fn try_discover_from_registry() -> Result<Vec<Tool>, Box<dyn std::error::Error>> {
    // Initialize Python tasks using angreal's function
    crate::initialize_python_tasks()?;

    // Access the ANGREAL_TASKS from the angreal crate
    let tasks = crate::task::ANGREAL_TASKS
        .lock()
        .map_err(|e| format!("Failed to lock ANGREAL_TASKS: {}", e))?;

    let mut tools = Vec::new();

    for (path, command) in tasks.iter() {
        // Use the full command path (e.g., "test.rust") instead of just the name ("rust")
        let tool_name = format!(
            "angreal_{}",
            path.replace(".", "_").replace(" ", "_").replace("-", "_")
        );
        debug!(
            "Converting command '{}' (path: {}) to MCP tool '{}'",
            command.name, path, tool_name
        );

        let tool = Tool {
            name: tool_name.clone(),
            description: Some(generate_enhanced_description(command)),
            input_schema: generate_command_schema(command, path)?,
            annotations: Some(generate_tool_annotations(command)),
            meta: None,
            output_schema: None,
            title: None,
            execution: None,
            icons: vec![],
        };

        debug!("Created MCP tool: {}", tool_name);
        tools.push(tool);
    }

    debug!(
        "Successfully discovered {} tools from ANGREAL_TASKS registry",
        tools.len()
    );
    for tool in &tools {
        debug!(
            "  - {}: {}",
            tool.name,
            tool.description.as_deref().unwrap_or("No description")
        );
    }

    Ok(tools)
}

/// Fallback: discover commands from filesystem scanning
fn discover_from_filesystem() -> Vec<Tool> {
    debug!("Falling back to filesystem discovery");

    // Try to find and parse task files directly
    match crate::utils::is_angreal_project() {
        Ok(project_path) => {
            debug!("Found angreal project at: {}", project_path.display());

            match crate::utils::get_task_files(project_path.join(".angreal")) {
                Ok(task_files) => {
                    debug!("Found {} task files", task_files.len());

                    // For now, create placeholder tools based on task files
                    task_files
                        .iter()
                        .enumerate()
                        .map(|(i, file)| {
                            let fallback_name = format!("task_{}", i);
                            let task_name = file
                                .file_stem()
                                .and_then(|name| name.to_str())
                                .unwrap_or(&fallback_name);

                            Tool {
                                name: format!("angreal_{}", task_name.replace("task_", "")),
                                description: Some(format!("Angreal task from {}", file.display())),
                                input_schema: serde_json::from_value(serde_json::json!({
                                    "type": "object",
                                    "properties": {},
                                    "additionalProperties": false
                                }))
                                .unwrap(),
                                annotations: None,
                                meta: None,
                                output_schema: None,
                                title: None,
                                execution: None,
                                icons: vec![],
                            }
                        })
                        .collect()
                }
                Err(e) => {
                    warn!("Failed to get task files: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            warn!("Not in angreal project: {}", e);
            Vec::new()
        }
    }
}

/// Generate enhanced description using ToolDescription if available, otherwise fall back to about
fn generate_enhanced_description(command: &crate::task::AngrealCommand) -> String {
    // If a ToolDescription is provided, use it as the primary description
    if let Some(tool) = &command.tool {
        let base = command
            .about
            .clone()
            .unwrap_or_else(|| "Angreal command".to_string());

        // Combine the short about with the full tool description
        format!("{}\n\n{}", base, tool.description.trim())
    } else {
        // Fall back to just the about text
        command
            .about
            .clone()
            .unwrap_or_else(|| "Angreal command".to_string())
    }
}

/// Generate MCP tool annotations based on ToolDescription risk_level
fn generate_tool_annotations(command: &crate::task::AngrealCommand) -> ToolAnnotations {
    let (destructive, read_only) = if let Some(tool) = &command.tool {
        match tool.risk_level.as_str() {
            "destructive" => (Some(true), Some(false)),
            "read_only" => (Some(false), Some(true)),
            "safe" => (Some(false), Some(false)),
            _ => (Some(false), Some(false)), // Unknown defaults to safe
        }
    } else {
        // Default to safe if no tool description
        (Some(false), Some(false))
    };

    ToolAnnotations {
        destructive_hint: destructive,
        idempotent_hint: None, // Could be extended later
        open_world_hint: Some(false),
        read_only_hint: read_only,
        title: None,
    }
}

/// Generate schema for a command based on its arguments
fn generate_command_schema(
    _command: &crate::task::AngrealCommand,
    command_path: &str,
) -> Result<ToolInputSchema, Box<dyn std::error::Error>> {
    // Access ANGREAL_ARGS to get the actual arguments for this command
    let args_registry = crate::task::ANGREAL_ARGS
        .lock()
        .map_err(|e| format!("Failed to lock ANGREAL_ARGS: {}", e))?;

    let command_args = args_registry.get(command_path).cloned().unwrap_or_default();

    let mut properties = json!({
        "command_path": {
            "type": "string",
            "description": format!("The command path ({})", command_path),
            "enum": [command_path]
        }
    });

    // Build properties from actual command arguments
    if !command_args.is_empty() {
        let mut args_properties = json!({});

        for arg in command_args.iter() {
            let arg_schema = match arg.python_type.as_deref().unwrap_or("str") {
                "bool" => json!({
                    "type": "boolean",
                    "description": arg.help.as_deref().unwrap_or(&format!("{} argument", arg.name))
                }),
                "int" => json!({
                    "type": "integer",
                    "description": arg.help.as_deref().unwrap_or(&format!("{} argument", arg.name))
                }),
                "float" => json!({
                    "type": "number",
                    "description": arg.help.as_deref().unwrap_or(&format!("{} argument", arg.name))
                }),
                _ => json!({
                    "type": "string",
                    "description": arg.help.as_deref().unwrap_or(&format!("{} argument", arg.name))
                }),
            };

            args_properties[&arg.name] = arg_schema;
        }

        properties["args"] = json!({
            "type": "object",
            "properties": args_properties,
            "additionalProperties": false,
            "description": "Command arguments"
        });
    }

    let schema_value = json!({
        "type": "object",
        "properties": properties,
        "required": ["command_path"],
        "additionalProperties": false
    });

    // Try to convert from serde_json::Value to ToolInputSchema
    serde_json::from_value(schema_value).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
