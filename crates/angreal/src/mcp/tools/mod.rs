pub mod all_tools;
pub mod angreal_command_tool;
pub mod dynamic_tools;
pub mod placeholder_tool;

pub use all_tools::AngrealTools;
pub use dynamic_tools::*;

use rust_mcp_sdk::schema::Tool;
use tracing::debug;

pub struct ToolRegistry {
    is_angreal_project: bool,
    dynamic_tools: Vec<Tool>,
}

impl ToolRegistry {
    pub fn new(is_angreal_project: bool) -> Self {
        debug!(
            "Initialized tool registry for angreal project: {}",
            is_angreal_project
        );

        let dynamic_tools = if is_angreal_project {
            discover_angreal_commands()
        } else {
            Vec::new()
        };

        debug!("Discovered {} dynamic tools", dynamic_tools.len());

        Self {
            is_angreal_project,
            dynamic_tools,
        }
    }

    pub fn list_tools(&self) -> Vec<Tool> {
        let mut tools = Vec::new();

        if self.is_angreal_project {
            // Add static tools (filtering out placeholder)
            tools.extend(
                AngrealTools::tools()
                    .into_iter()
                    .filter(|tool| tool.name != "_placeholder"),
            );

            // Add dynamically discovered tools
            tools.extend(self.dynamic_tools.clone());
        }

        tools
    }
}
