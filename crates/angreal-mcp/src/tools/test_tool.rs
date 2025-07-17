use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    schema::{schema_utils::CallToolError, CallToolResult, TextContent},
};
use serde::{Deserialize, Serialize};

#[mcp_tool(
    name = "test",
    description = "A test tool to verify MCP server functionality",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TestTool {
    /// A test message
    pub message: Option<String>,
}

impl TestTool {
    pub async fn call_tool(&self) -> std::result::Result<CallToolResult, CallToolError> {
        let message = self
            .message
            .as_deref()
            .unwrap_or("Hello from angreal MCP server!");

        let response = serde_json::json!({
            "test_response": format!("Test tool received: {}", message),
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        Ok(CallToolResult::text_content(vec![TextContent::from(
            serde_json::to_string_pretty(&response).map_err(CallToolError::new)?,
        )]))
    }
}
