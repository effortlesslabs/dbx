use clap::Parser;
use std::net::SocketAddr;
use tracing::{ info, Level };

mod config;
mod constants;
mod handlers;
mod middleware;
mod models;
mod routes;
mod server;

use config::{ Config, DatabaseType };
use constants::{ config::ConfigDefaults, database::DatabaseUrls, errors::ErrorMessages };
use server::Server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Database type (redis, postgres, mongodb, mysql)
    #[arg(short, long, value_enum, default_value = "redis")]
    database_type: DatabaseType,

    /// Database connection URL
    #[arg(short = 'u', long)]
    database_url: Option<String>,

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

    info!("Starting DBX API server for {}...", args.database_type);

    let database_type = args.database_type.clone();
    // Create configuration with environment variables taking precedence
    let config = Config {
        database_type,
        database_url: args.database_url
            .or_else(|| std::env::var("DATABASE_URL").ok())
            .unwrap_or_else(|| {
                match args.database_type {
                    DatabaseType::Redis => DatabaseUrls::redis_url(),
                    // DatabaseType::Postgres => DatabaseUrls::postgres_url(),
                    // DatabaseType::MongoDB => DatabaseUrls::mongodb_url(),
                    // DatabaseType::MySQL => DatabaseUrls::mysql_url(),
                }
            }),
        host: args.host
            .or_else(|| std::env::var("HOST").ok())
            .unwrap_or_else(|| ConfigDefaults::HOST.to_string()),
        port: args.port
            .or_else(||
                std::env
                    ::var("PORT")
                    .ok()
                    .and_then(|s| s.parse().ok())
            )
            .unwrap_or(ConfigDefaults::PORT),
        pool_size: args.pool_size
            .or_else(||
                std::env
                    ::var("POOL_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
            )
            .unwrap_or(ConfigDefaults::POOL_SIZE),
    };

    // Create and start server
    let server = Server::new(config).await?;
    let addr = SocketAddr::new(server.config().host.parse()?, server.config().port);

    info!("Server listening on {}", addr);
    info!("Database type: {}", server.config().database_type);
    info!("Database URL: {}", server.config().database_url);
    info!("Connection pool size: {}", server.config().pool_size);

    server.run(addr).await?;

    Ok(())
}
