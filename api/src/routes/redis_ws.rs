use axum::{ routing::get, Router };

use crate::handlers::redis_ws::handler::RedisWsHandler;

/// RedisWs routes
pub fn redis_ws_routes() -> Router<RedisWsHandler> {
    Router::new().route("/redis_ws", get(RedisWsHandler::handle_redis_ws))
}
