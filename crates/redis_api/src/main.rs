use std::net::SocketAddr;
use tracing_subscriber;

use dbx_api::{ config::{ Config, DatabaseType }, server::Server, constants::defaults::Defaults };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration from environment variables
    let config = Config {
        database_type: DatabaseType::Redis,
        database_url: std::env
            ::var("DATABASE_URL")
            .unwrap_or_else(|_| Defaults::DATABASE_URL.to_string()),
        host: std::env::var("HOST").unwrap_or_else(|_| Defaults::HOST.to_string()),
        port: std::env
            ::var("PORT")
            .unwrap_or_else(|_| Defaults::PORT.to_string())
            .parse()
            .unwrap_or(Defaults::PORT),
        pool_size: std::env
            ::var("POOL_SIZE")
            .unwrap_or_else(|_| Defaults::POOL_SIZE.to_string())
            .parse()
            .unwrap_or(Defaults::POOL_SIZE),
    };

    // Create and run server
    let server = Server::new(config.clone()).await?;
    let addr = format!("{}:{}", config.host, config.port).parse::<SocketAddr>()?;

    server.run(addr).await?;

    Ok(())
}
