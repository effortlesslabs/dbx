use clap::Parser;
use std::net::SocketAddr;
use tracing::{ info, Level };

mod config;
mod handlers;
mod middleware;
mod models;
mod routes;
mod server;

use config::Config;
use server::Server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Redis connection URL
    #[arg(short, long)]
    redis_url: Option<String>,

    /// Server host
    #[arg(long)]
    host: Option<String>,

    /// Server port
    #[arg(short, long)]
    port: Option<u16>,

    /// Log level
    #[arg(long)]
    log_level: Option<Level>,

    /// Connection pool size
    #[arg(long)]
    pool_size: Option<u32>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    let args = Args::parse();

    // Initialize tracing
    let log_level = args.log_level
        .or_else(||
            std::env
                ::var("LOG_LEVEL")
                .ok()
                .and_then(|s| s.parse().ok())
        )
        .unwrap_or(Level::INFO);

    tracing_subscriber::fmt().with_max_level(log_level).init();

    info!("Starting DBX API server...");

    // Create configuration with environment variables taking precedence
    let config = Config {
        redis_url: args.redis_url
            .or_else(|| std::env::var("REDIS_URL").ok())
            .unwrap_or_else(|| "redis://127.0.0.1:6379".to_string()),
        host: args.host
            .or_else(|| std::env::var("HOST").ok())
            .unwrap_or_else(|| "127.0.0.1".to_string()),
        port: args.port
            .or_else(||
                std::env
                    ::var("PORT")
                    .ok()
                    .and_then(|s| s.parse().ok())
            )
            .unwrap_or(3000),
        pool_size: args.pool_size
            .or_else(||
                std::env
                    ::var("POOL_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
            )
            .unwrap_or(10),
    };

    // Create and start server
    let server = Server::new(config).await?;
    let addr = SocketAddr::new(server.config().host.parse()?, server.config().port);

    info!("Server listening on {}", addr);
    info!("Redis URL: {}", server.config().redis_url);
    info!("Connection pool size: {}", server.config().pool_size);

    server.run(addr).await?;

    Ok(())
}
