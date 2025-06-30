use axum::http::StatusCode;
use axum::{response::Html, routing::get, Router};
use std::fs;
use std::sync::Arc;
use tracing::info;

use crate::{config::Config, constants::errors::ErrorMessages};

use dbx_adapter::redis::{client::RedisPool, RedisPoolAdapter};

pub struct Server {
    config: Config,
    redis_pool: Option<Arc<RedisPool>>,
}

impl Server {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        info!("Connecting to Redis at {}", config.database_url);

        let pool = RedisPool::new(&config.database_url, config.pool_size)?;
        let pool_adapter = RedisPoolAdapter::new(pool.clone());
        let redis = pool_adapter.get_instance()?;
        let ping_result = redis.ping();

        let redis_pool = match ping_result {
            Ok(true) => {
                info!("Successfully connected to Redis with connection pool");
                Some(Arc::new(pool))
            }
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
        };

        Ok(Self { config, redis_pool })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Create the application router
    pub fn create_router(&self) -> Router {
        let mut router = Router::new()
            .route("/", get(serve_landing_page))
            .route("/redis_ws", get(serve_landing_page));

        // Add Redis admin routes if Redis pool is available
        if let Some(pool) = &self.redis_pool {
            let redis_string_routes =
                crate::routes::redis::string::create_redis_string_routes(pool.clone());
            let redis_hash_routes =
                crate::routes::redis::hash::create_redis_hash_routes(pool.clone());
            let redis_set_routes = crate::routes::redis::set::create_redis_set_routes(pool.clone());
            let redis_admin_routes =
                crate::routes::redis::admin::create_redis_admin_routes(pool.clone());
            let redis_ws_string_routes =
                crate::routes::redis_ws::string::create_redis_ws_string_routes(pool.clone());
            let redis_ws_hash_routes =
                crate::routes::redis_ws::hash::create_redis_ws_hash_routes(pool.clone());
            let redis_ws_set_routes =
                crate::routes::redis_ws::set::create_redis_ws_set_routes(pool.clone());
            let redis_ws_admin_routes =
                crate::routes::redis_ws::admin::create_redis_ws_admin_routes(pool.clone());

            router = router
                .nest("/redis", redis_string_routes)
                .nest("/redis", redis_hash_routes)
                .nest("/redis", redis_set_routes)
                .nest("/redis", redis_admin_routes)
                .nest("/redis_ws", redis_ws_string_routes)
                .nest("/redis_ws", redis_ws_hash_routes)
                .nest("/redis_ws", redis_ws_set_routes)
                .nest("/redis_ws", redis_ws_admin_routes);
        }

        router
    }

    /// Run the server
    pub async fn run(self, addr: std::net::SocketAddr) -> anyhow::Result<()> {
        let app = self.create_router();

        info!("Starting Redis API server on {}", addr);
        info!("HTTP API available at http://{}", addr);
        info!("RedisWs API available at ws://{}/redis_ws", addr);
        info!(
            "Redis Admin HTTP API available at http://{}/redis/admin",
            addr
        );
        info!(
            "Redis Admin WebSocket API available at ws://{}/redis_ws/admin/ws",
            addr
        );
        info!(
            "Redis String HTTP API available at http://{}/redis/string",
            addr
        );
        info!(
            "Redis String WebSocket API available at ws://{}/redis_ws/string/ws",
            addr
        );
        info!(
            "Redis Hash HTTP API available at http://{}/redis/hash",
            addr
        );
        info!(
            "Redis Hash WebSocket API available at ws://{}/redis_ws/hash/ws",
            addr
        );
        info!("Redis Set HTTP API available at http://{}/redis/set", addr);
        info!(
            "Redis Set WebSocket API available at ws://{}/redis_ws/set/ws",
            addr
        );

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
            redis_pool: self.redis_pool.clone(),
        }
    }
}

/// Serve the landing page HTML
async fn serve_landing_page() -> Result<Html<String>, StatusCode> {
    match fs::read_to_string("static/index.html") {
        Ok(content) => Ok(Html(content)),
        Err(_) => {
            // Fallback to a simple HTML if file not found
            let fallback_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DBX - Redis API Gateway</title>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-gray-50">
    <div class="min-h-screen flex items-center justify-center">
        <div class="text-center">
            <h1 class="text-4xl font-bold text-blue-600 mb-4">DBX API</h1>
            <p class="text-xl text-gray-600 mb-8">Redis API Gateway</p>
            <div class="space-y-4">
                <div class="bg-white p-4 rounded-lg shadow">
                    <h2 class="font-semibold text-gray-900">Available Endpoints:</h2>
                    <ul class="text-left mt-2 space-y-1 text-sm text-gray-600">
                        <li>• <code>/redis/admin/ping</code> - Health check</li>
                        <li>• <code>/redis/admin/health</code> - Server status</li>
                        <li>• <code>/redis/string/:key</code> - String operations</li>
                        <li>• <code>ws://localhost:3000/redis_ws/string/ws</code> - WebSocket API</li>
                    </ul>
                </div>
                <p class="text-sm text-gray-500">For full documentation, check the static/index.html file</p>
            </div>
        </div>
    </div>
</body>
</html>
            "#;
            Ok(Html(fallback_html.to_string()))
        }
    }
}
