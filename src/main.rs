use mcp_forecast::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Pipe(Box::new(
            std::fs::File::create("/tmp/mcp_forecast.log").unwrap(),
        )))
        .init();
    start_server().await?;

    Ok(())
}
