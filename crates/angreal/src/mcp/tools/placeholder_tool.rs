use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    schema::{CallToolResult, TextContent},
};
use serde::{Deserialize, Serialize};

#[mcp_tool(
    name = "_placeholder",
    description = "Internal placeholder tool - not exposed to users",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PlaceholderTool {}

impl PlaceholderTool {
    #[allow(dead_code)]
    pub async fn call_tool(
        &self,
    ) -> Result<CallToolResult, rust_mcp_sdk::schema::schema_utils::CallToolError> {
        Ok(CallToolResult::text_content(vec![TextContent::from(
            "This is a placeholder tool and should not be called".to_string(),
        )]))
    }
}
