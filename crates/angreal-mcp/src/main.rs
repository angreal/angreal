use anyhow::Result;
use rust_mcp_sdk::{
    mcp_server::server_runtime,
    schema::{
        Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
        LATEST_PROTOCOL_VERSION,
    },
    McpServer, StdioTransport, ToMcpServerHandler, TransportOptions,
};
use tracing::{info, warn};

mod server;
mod tools;

use server::AngrealMcpHandler;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize basic logging to stderr
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting angreal MCP server");

    // Check if we're in an angreal project
    let is_angreal_project = angreal::utils::is_angreal_project().is_ok();

    if !is_angreal_project {
        warn!("Not in an angreal project, running with zero tools");
    } else {
        info!("Detected angreal project, initializing tools");
    }

    // Create server details
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Angreal MCP Server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            title: Some("Angreal MCP Server".to_string()),
            description: Some(
                "MCP server for angreal project task automation and execution".to_string(),
            ),
            icons: vec![],
            website_url: Some("https://github.com/angreal/angreal".to_string()),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some(
            r#"Angreal MCP Server - Direct access to angreal project commands

This server provides MCP tools for angreal project automation. Use these tools when:

1. Running project-specific tasks (tests, builds, documentation)
2. Executing angreal commands in the current project context
3. Automating development workflows defined in .angreal/ directory

Available tools are dynamically discovered from the project's .angreal/task_*.py files.
Each tool corresponds to an angreal command and will execute in the project context.

Tools accept arguments as defined by each command. Check tool descriptions for specifics."#
                .to_string(),
        ),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    // Create transport with extended timeout for long-running angreal commands
    let transport_options = TransportOptions {
        timeout: std::time::Duration::from_secs(600), // 10 minutes timeout
        ..Default::default()
    };
    let transport = StdioTransport::new(transport_options)
        .map_err(|e| anyhow::anyhow!("Failed to create transport: {}", e))?;

    // Create handler and convert to MCP server handler
    let handler = AngrealMcpHandler::new(is_angreal_project).to_mcp_server_handler();

    // Create and start server
    let server = server_runtime::create_server(server_details, transport, handler);

    info!("MCP server started, listening on stdio");

    // Run the server
    server
        .start()
        .await
        .map_err(|e| anyhow::anyhow!("MCP server failed to start: {}", e))?;

    Ok(())
}
