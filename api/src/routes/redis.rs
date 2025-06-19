use axum::{ routing::{ get, post, delete }, Router };
use crate::handlers::redis::RedisHandler;

/// Create Redis-specific routes
pub fn create_routes() -> Router<RedisHandler> {
    Router::new().nest("/api/v1/redis", create_redis_api_routes())
}

/// Create Redis API routes
fn create_redis_api_routes() -> Router<RedisHandler> {
    Router::new()
        .nest("/strings", create_string_routes())
        .nest("/scripts", create_script_routes())
        .nest("/keys", create_key_routes())
}

/// Create string operation routes
fn create_string_routes() -> Router<RedisHandler> {
    Router::new()
        .route("/:key", get(RedisHandler::get_string))
        .route("/:key", post(RedisHandler::set_string))
        .route("/:key", delete(RedisHandler::delete_string))
        .route("/:key/exists", get(RedisHandler::exists))
        .route("/:key/ttl", get(RedisHandler::get_ttl))
        .route("/:key/incr", post(RedisHandler::incr))
        .route("/:key/incrby", post(RedisHandler::incr_by))
        .route("/:key/setnx", post(RedisHandler::set_nx))
        .route("/:key/cas", post(RedisHandler::compare_and_set))
        .route("/batch/set", post(RedisHandler::batch_set))
        .route("/batch/get", post(RedisHandler::batch_get))
        .route("/batch/delete", post(RedisHandler::batch_delete))
        .route("/batch/incr", post(RedisHandler::batch_incr))
        .route("/batch/incrby", post(RedisHandler::batch_incr_by))
}

/// Create script operation routes
fn create_script_routes() -> Router<RedisHandler> {
    Router::new()
        .route("/rate-limiter", post(RedisHandler::rate_limiter_script))
        .route("/multi-counter", post(RedisHandler::multi_counter_script))
        .route("/multi-set-ttl", post(RedisHandler::multi_set_ttl_script))
}

/// Create key operation routes
fn create_key_routes() -> Router<RedisHandler> {
    Router::new()
        .route("/", get(RedisHandler::list_keys))
        .route("/:key/exists", get(RedisHandler::key_exists))
        .route("/:key/ttl", get(RedisHandler::key_ttl))
        .route("/:key", delete(RedisHandler::delete_key))
}
