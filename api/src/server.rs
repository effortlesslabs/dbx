use axum::{http::StatusCode, response::Json, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    config::{Config, DatabaseType},
    constants::errors::ErrorMessages,
    handlers::{redis::RedisHandler, websocket::WebSocketHandler},
    models::ApiResponse,
    routes,
};

use dbx_crates::adapter::redis::client::RedisClient;

/// Main server struct
pub struct Server {
    config: Config,
}

impl Server {
    /// Create a new server instance
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        info!(
            "Connecting to {} at {}",
            config.database_type, config.database_url
        );

        // Test the database connection based on type
        match config.database_type {
            DatabaseType::Redis => {
                let redis_client = RedisClient::from_url(&config.database_url)?;
                let ping_result = redis_client.ping();
                match ping_result {
                    Ok(true) => info!("Successfully connected to Redis"),
                    Ok(false) => {
                        return Err(anyhow::anyhow!(ErrorMessages::REDIS_PING_FAILED));
                    }
                    Err(e) => {
                        return Err(anyhow::anyhow!(
                            "{}{}",
                            ErrorMessages::REDIS_CONNECTION_FAILED,
                            e
                        ));
                    }
                }
            } // Future database types
              // DatabaseType::Postgres => { /* PostgreSQL connection test */ }
              // DatabaseType::MongoDB => { /* MongoDB connection test */ }
              // DatabaseType::MySQL => { /* MySQL connection test */ }
        }

        Ok(Self { config })
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Create the application router
    pub fn create_router(&self) -> Router {
        // CORS configuration
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        // Create database-specific routes
        let (database_routes, websocket_routes, redis_handler) = match self.config.database_type {
            DatabaseType::Redis => {
                let redis_client = RedisClient::from_url(&self.config.database_url)
                    .expect(ErrorMessages::REDIS_CLIENT_CREATION_FAILED);
                let redis_handler = Arc::new(RedisHandler::new(redis_client));
                let websocket_handler = WebSocketHandler::new(Arc::new(tokio::sync::Mutex::new(
                    (*redis_handler).clone(),
                )));

                let http_routes = crate::routes::redis::create_routes(redis_handler.clone());
                let ws_routes = routes::websocket::create_routes().with_state(websocket_handler);

                (http_routes, ws_routes, redis_handler)
            } // Future database types
              // DatabaseType::Postgres => { /* PostgreSQL routes */ }
              // DatabaseType::MongoDB => { /* MongoDB routes */ }
              // DatabaseType::MySQL => { /* MySQL routes */ }
        };

        Router::new()
            .merge(routes::common::create_routes())
            .merge(database_routes)
            .merge(websocket_routes)
            .layer(cors)
            .layer(TraceLayer::new_for_http())
            .fallback(|| async {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::<()>::error(
                        ErrorMessages::NOT_FOUND.to_string(),
                    )),
                )
            })
            .with_state(redis_handler)
    }

    /// Run the server
    pub async fn run(self, addr: std::net::SocketAddr) -> anyhow::Result<()> {
        let app = self.create_router();

        info!(
            "Starting {} API server on {}",
            self.config.database_type, addr
        );
        info!("HTTP API available at http://{}", addr);
        info!("WebSocket API available at ws://{}/ws", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await?;

        Ok(())
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
}
