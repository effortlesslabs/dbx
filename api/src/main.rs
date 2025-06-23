use std::net::SocketAddr;
use tracing_subscriber;

use dbx_api::{ config::{ Config, DatabaseType }, server::Server };

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration from environment variables
    let config = Config {
        database_type: DatabaseType::Redis,
        database_url: std::env
            ::var("DATABASE_URL")
            .unwrap_or_else(|_| "redis://default:redispw@host.docker.internal:55000".to_string()),
        host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: std::env
            ::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000),
        pool_size: std::env
            ::var("POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10),
    };

    // Create and run server
    let server = Server::new(config.clone()).await?;
    let addr = format!("{}:{}", config.host, config.port).parse::<SocketAddr>()?;

    server.run(addr).await?;

    Ok(())
}
