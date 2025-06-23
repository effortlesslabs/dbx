use axum::{ Router, routing::get };
use tracing::info;
use std::sync::Arc;

use crate::{ config::{ Config, DatabaseType }, constants::errors::ErrorMessages };

use dbx_crates::adapter::redis::{ RedisPoolAdapter, client::RedisPool };

pub struct Server {
    config: Config,
    redis_pool: Option<Arc<RedisPool>>,
}

impl Server {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        info!("Connecting to {} at {}", config.database_type, config.database_url);

        let redis_pool = match config.database_type {
            DatabaseType::Redis => {
                let pool = RedisPool::new(&config.database_url, config.pool_size)?;
                let pool_adapter = RedisPoolAdapter::new(pool.clone());
                let redis = pool_adapter.get_instance()?;
                let ping_result = redis.ping();
                match ping_result {
                    Ok(true) => {
                        info!("Successfully connected to Redis with connection pool");
                        Some(Arc::new(pool))
                    }
                    Ok(false) => {
                        return Err(anyhow::anyhow!(ErrorMessages::REDIS_PING_FAILED));
                    }
                    Err(e) => {
                        return Err(
                            anyhow::anyhow!("{}{}", ErrorMessages::REDIS_CONNECTION_FAILED, e)
                        );
                    }
                }
            } // Future database types
            // DatabaseType::Postgres => { /* PostgreSQL connection test */ }
            // DatabaseType::MongoDB => { /* MongoDB connection test */ }
            // DatabaseType::MySQL => { /* MySQL connection test */ }
        };

        Ok(Self {
            config,
            redis_pool,
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Create the application router
    pub fn create_router(&self) -> Router {
        let mut router = Router::new()
            .route(
                "/",
                get(|| async { "Hello, World!" })
            )
            .route(
                "/redis_ws",
                get(|| async { "RedisWs API" })
            );

        // Add Redis admin routes if Redis pool is available
        if let Some(pool) = &self.redis_pool {
            let redis_string_routes = crate::routes::redis::string::create_redis_string_routes(
                pool.clone()
            );
            let redis_ws_string_routes =
                crate::routes::redis_ws::string::create_redis_ws_string_routes(pool.clone());

            router = router
                .nest("/redis", redis_string_routes)
                .nest("/redis_ws", redis_ws_string_routes);
        }

        router
    }

    /// Run the server
    pub async fn run(self, addr: std::net::SocketAddr) -> anyhow::Result<()> {
        let app = self.create_router();

        info!("Starting {} API server on {}", self.config.database_type, addr);
        info!("HTTP API available at http://{}", addr);
        info!("RedisWs API available at ws://{}/redis_ws", addr);
        info!("Redis Admin HTTP API available at http://{}/redis/admin", addr);
        info!("Redis Admin WebSocket API available at ws://{}/redis_ws/admin/ws", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>()
        ).await?;

        Ok(())
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            redis_pool: self.redis_pool.clone(),
        }
    }
}
