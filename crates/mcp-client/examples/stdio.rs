use anyhow::Result;
use mcp_client::client::{
    ClientCapabilities, ClientInfo, Error as ClientError, McpClient, McpClientImpl,
};
use mcp_client::{
    service::TransportService,
    transport::{StdioTransport, Transport},
};
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("mcp_client=debug".parse().unwrap())
                .add_directive("eventsource_client=debug".parse().unwrap()),
        )
        .init();

    // 1) Create the transport
    let transport = StdioTransport::new("uvx", vec!["mcp-server-git".to_string()]);

    // 2) Start the transport to get a handle
    let transport_handle = transport.start().await.unwrap();

    // 3) Build service using the handle
    let service = ServiceBuilder::new().service(TransportService::new(transport_handle));

    // 4) Create client
    let client = McpClientImpl::new(service);

    // Initialize
    let server_info = client
        .initialize(
            ClientInfo {
                name: "test-client".into(),
                version: "1.0.0".into(),
            },
            ClientCapabilities::default(),
        )
        .await?;
    println!("Connected to server: {server_info:?}\n");

    // List tools
    let tools = client.list_tools().await?;
    println!("Available tools: {tools:?}\n");

    // Call tool 'git_status' with arguments = {"repo_path": "."}
    let tool_result = client
        .call_tool("git_status", serde_json::json!({ "repo_path": "." }))
        .await?;
    println!("Tool result: {tool_result:?}\n");

    Ok(())
}
