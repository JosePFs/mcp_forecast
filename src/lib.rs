use crate::server::WeatherServer;
use rmcp::{ServiceExt, transport::io::stdio};

pub mod command_runner;
pub mod dto;
pub mod rules;
pub mod server;

pub async fn start_server() -> anyhow::Result<()> {
    let service = WeatherServer::new().serve(stdio()).await?;

    service.waiting().await?;

    Ok(())
}
