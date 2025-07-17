use async_trait::async_trait;
use rust_mcp_sdk::{
    mcp_server::ServerHandler,
    schema::{
        schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
        ListToolsResult, TextContent,
    },
    McpServer,
};
use tracing::{debug, info};

use crate::tools::{angreal_command_tool::AngrealCommandTool, ToolRegistry};

pub struct AngrealMcpHandler {
    tools: ToolRegistry,
    is_angreal_project: bool,
}

impl AngrealMcpHandler {
    pub fn new(is_angreal_project: bool) -> Self {
        let tools = ToolRegistry::new(is_angreal_project);

        Self {
            tools,
            is_angreal_project,
        }
    }
}

#[async_trait]
impl ServerHandler for AngrealMcpHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _server: &dyn McpServer,
    ) -> Result<ListToolsResult, rust_mcp_sdk::schema::RpcError> {
        debug!("Listing available tools");

        let tools = self.tools.list_tools();

        info!("Returning {} tools", tools.len());

        Ok(ListToolsResult {
            tools,
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _server: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        debug!("Tool call requested: {}", request.params.name);

        if !self.is_angreal_project {
            return Ok(CallToolResult::text_content(vec![TextContent::from(
                "Error: Not in an angreal project".to_string(),
            )]));
        }

        let args = serde_json::Value::Object(request.params.arguments.unwrap_or_default());

        match request.params.name.as_str() {
            tool_name if tool_name.starts_with("angreal_") => {
                // Handle dynamically discovered angreal commands
                self.handle_dynamic_angreal_tool(tool_name, args).await
            }
            _ => Ok(CallToolResult::text_content(vec![TextContent::from(
                format!("Unknown tool: {}", request.params.name),
            )])),
        }
    }
}

impl AngrealMcpHandler {
    async fn handle_dynamic_angreal_tool(
        &self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, CallToolError> {
        debug!("Handling dynamic angreal tool: {}", tool_name);

        // Map tool name back to command path
        let command_path = self.map_tool_name_to_command_path(tool_name)?;

        // Extract the args field if it exists, otherwise use empty object
        let command_args = if let Some(args_obj) = args.get("args") {
            args_obj.clone()
        } else {
            serde_json::Value::Object(serde_json::Map::new())
        };

        // Create AngrealCommandTool with the mapped command path and args
        let angreal_tool = AngrealCommandTool {
            command_path,
            args: if command_args.is_object() {
                serde_json::from_value(command_args).ok()
            } else {
                None
            },
        };

        angreal_tool.call_tool().await
    }

    fn map_tool_name_to_command_path(&self, tool_name: &str) -> Result<String, CallToolError> {
        debug!("Mapping tool name '{}' to command path", tool_name);

        // Initialize angreal tasks to ensure registry is populated
        angreal::initialize_python_tasks().map_err(|e| {
            CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to initialize angreal tasks: {}", e),
            ))
        })?;

        // Search through registered commands to find matching tool name
        let tasks = angreal::task::ANGREAL_TASKS.lock().map_err(|e| {
            CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to lock ANGREAL_TASKS: {}", e),
            ))
        })?;

        for (command_path, _command) in tasks.iter() {
            let expected_tool_name = format!(
                "angreal_{}",
                command_path
                    .replace(".", "_")
                    .replace(" ", "_")
                    .replace("-", "_")
            );
            if expected_tool_name == tool_name {
                debug!(
                    "Found matching command path '{}' for tool '{}'",
                    command_path, tool_name
                );
                return Ok(command_path.clone());
            }
        }

        Err(CallToolError::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No command found for tool name: {}", tool_name),
        )))
    }
}
