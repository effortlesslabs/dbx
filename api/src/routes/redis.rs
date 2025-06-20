use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{ delete, get, post },
    Router,
};
use std::sync::Arc;
use crate::handlers::redis::RedisHandler;

/// Create Redis-specific routes
pub fn create_routes(redis_handler: Arc<RedisHandler>) -> Router<Arc<RedisHandler>> {
    Router::new().nest("/api/v1/redis", create_redis_routes())
}

/// Create Redis API routes
pub fn create_redis_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
        // .route("/health", get(RedisHandler::health))
        .route("/dbsize", get(RedisHandler::dbsize))
        .route("/flushall", post(RedisHandler::flushall))
        .route("/flushdb", post(RedisHandler::flushdb))
        .nest("/strings", create_string_routes())
        .nest("/sets", create_set_routes())
        .nest("/hashes", create_hash_routes())
        .nest("/keys", create_key_routes())
    // .nest("/scripts", create_script_routes()) // Temporarily disabled due to handler trait issues
}

/// Create string operation routes
fn create_string_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
        .route("/:key", get(RedisHandler::get_string))
        .route("/:key", post(RedisHandler::set_string))
        .route("/:key", delete(RedisHandler::delete_string))
        .route("/:key/ttl", get(RedisHandler::get_ttl))
        .route("/:key/incr", post(RedisHandler::incr))
        .route("/:key/incrby", post(RedisHandler::incr_by))
        .route("/:key/setnx", post(RedisHandler::set_nx))
        .route("/:key/cas", post(RedisHandler::compare_and_set))
        .route("/batch/get", post(RedisHandler::batch_get))
        .route("/batch/set", post(RedisHandler::batch_set))
        .route("/batch/delete", post(RedisHandler::batch_delete))
        .route("/batch/incr", post(RedisHandler::batch_incr))
        .route("/batch/incrby", post(RedisHandler::batch_incr_by))
}

/// Create set operation routes
fn create_set_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
        .route("/:key", get(RedisHandler::get_set_members))
        .route("/:key", post(RedisHandler::add_set_members))
        .route("/:key", delete(RedisHandler::delete_set))
        .route("/:key/members", get(RedisHandler::get_set_members))
        .route("/:key/exists", get(RedisHandler::set_member_exists))
        .route("/:key/cardinality", get(RedisHandler::get_set_cardinality))
        .route("/:key/random", get(RedisHandler::get_random_member))
        .route("/:key/pop", post(RedisHandler::pop_random_member))
        .route("/:key/move", post(RedisHandler::move_set_member))
        .route("/:key/union", post(RedisHandler::set_union))
        .route("/:key/intersection", post(RedisHandler::set_intersection))
        .route("/:key/difference", post(RedisHandler::set_difference))
        .route("/batch/add", post(RedisHandler::batch_add_set_members))
        .route("/batch/remove", post(RedisHandler::batch_remove_set_members))
        .route("/batch/members", post(RedisHandler::batch_get_set_members))
        .route("/batch/delete", post(RedisHandler::batch_delete_sets))
}

/// Create hash operation routes
fn create_hash_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
        .route("/:key", get(RedisHandler::get_hash_all))
        .route("/:key", post(RedisHandler::set_hash_multiple))
        .route("/:key", delete(RedisHandler::delete_hash))
        .route("/:key/:field", get(RedisHandler::get_hash_field))
        .route("/:key/:field", post(RedisHandler::set_hash_field))
        .route("/:key/:field", delete(RedisHandler::delete_hash_field))
        .route("/:key/:field/exists", get(RedisHandler::hash_field_exists))
        .route("/:key/:field/incr", post(RedisHandler::increment_hash_field))
        .route("/:key/:field/setnx", post(RedisHandler::set_hash_field_nx))
        .route("/:key/length", get(RedisHandler::get_hash_length))
        .route("/:key/keys", get(RedisHandler::get_hash_keys))
        .route("/:key/values", get(RedisHandler::get_hash_values))
        .route("/:key/random", get(RedisHandler::get_random_hash_field))
        .route("/:key/mget", post(RedisHandler::get_multiple_hash_fields))
        .route("/batch/set", post(RedisHandler::batch_set_hash_fields))
        .route("/batch/get", post(RedisHandler::batch_get_hash_fields))
        .route("/batch/delete", post(RedisHandler::batch_delete_hash_fields))
        .route("/batch/all", post(RedisHandler::batch_get_hash_all))
        .route("/batch/exists", post(RedisHandler::batch_check_hash_fields))
        .route("/batch/lengths", post(RedisHandler::batch_get_hash_lengths))
}

/// Create key operation routes
fn create_key_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
        .route("/", get(RedisHandler::list_keys))
        .route("/:key/exists", get(RedisHandler::key_exists))
        .route("/:key/ttl", get(RedisHandler::key_ttl))
        .route("/:key", delete(RedisHandler::delete_key))
}

/// Create script operation routes (temporarily disabled)
fn create_script_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
    // .route("/rate-limiter", post(crate::handlers::redis::scripts::rate_limiter_script))
    // .route("/multi-counter", post(crate::handlers::redis::scripts::multi_counter_script))
    // .route("/multi-set-ttl", post(crate::handlers::redis::scripts::multi_set_ttl_script))
}
