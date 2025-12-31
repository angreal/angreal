use super::placeholder_tool::PlaceholderTool;
use rust_mcp_sdk::tool_box;

// Generate the combined AngrealTools enum with placeholder to satisfy tool_box requirements
tool_box!(AngrealTools, [PlaceholderTool]);
