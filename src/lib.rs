use crate::server::RiskWeatherServer;
use rmcp::{ServiceExt, transport::io::stdio};

pub mod bootstrap;
pub mod command_runner;
pub mod dto;
pub mod forecasting;
pub mod server;

pub async fn start_server() -> anyhow::Result<()> {
    let service = RiskWeatherServer::new().serve(stdio()).await?;

    service.waiting().await?;

    Ok(())
}
