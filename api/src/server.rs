use axum::{ http::StatusCode, response::Json, routing::get, Router };
use std::sync::Arc;
use tower_http::cors::{ Any, CorsLayer };
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    config::Config,
    handlers::redis_handlers,
    models::{ ApiResponse, HealthResponse, ServerInfo },
};

use dbx_crates::adapter::redis::Redis;

/// Main server struct
pub struct Server {
    config: Config,
    redis: Arc<Redis>,
}

impl Server {
    /// Create a new server instance
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        info!("Connecting to Redis at {}", config.redis_url);

        let redis = Redis::from_url(&config.redis_url)?;

        // Test the connection
        let ping_result = redis.ping();
        match ping_result {
            Ok(true) => info!("Successfully connected to Redis"),
            Ok(false) => {
                return Err(anyhow::anyhow!("Redis ping failed"));
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Failed to connect to Redis: {}", e));
            }
        }

        Ok(Self {
            config,
            redis: Arc::new(redis),
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the Redis instance
    pub fn redis(&self) -> &Arc<Redis> {
        &self.redis
    }

    /// Create the application router
    pub fn create_router(&self) -> Router {
        // CORS configuration
        let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

        // Health check handler
        let health_handler = {
            let redis = self.redis.clone();
            move || async move {
                let redis_connected = redis.ping().unwrap_or(false);
                let health = HealthResponse {
                    status: "ok".to_string(),
                    redis_connected,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                Json(ApiResponse::success(health))
            }
        };

        // Server info handler
        let info_handler = {
            let config = self.config.clone();
            move || async move {
                let info = ServerInfo {
                    name: "DBX API".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    redis_url: config.redis_url.clone(),
                    pool_size: config.pool_size,
                };
                Json(ApiResponse::success(info))
            }
        };

        Router::new()
            .route("/health", get(health_handler))
            .route("/info", get(info_handler))
            .nest("/api/v1/redis", redis_handlers::create_redis_routes(self.redis.clone()))
            .layer(cors)
            .layer(TraceLayer::new_for_http())
            .fallback(|| async {
                (StatusCode::NOT_FOUND, Json(ApiResponse::<()>::error("Not found".to_string())))
            })
    }

    /// Run the server
    pub async fn run(self, addr: std::net::SocketAddr) -> anyhow::Result<()> {
        let app = self.create_router();

        info!("Starting server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}
