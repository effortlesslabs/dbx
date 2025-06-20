use axum::{ routing::{ get, post, delete }, Router };
use std::sync::Arc;
use crate::handlers::redis::RedisHandler;

/// Create Redis-specific routes
pub fn create_routes(redis_handler: Arc<RedisHandler>) -> Router<Arc<RedisHandler>> {
    Router::new().nest("/api/v1/redis", create_redis_api_routes())
}

/// Create Redis API routes
fn create_redis_api_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
        .nest("/strings", create_string_routes())
        .nest("/sets", create_set_routes())
        .nest("/keys", create_key_routes())
}

/// Create string operation routes
fn create_string_routes() -> Router<Arc<RedisHandler>> {
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

/// Create key operation routes
fn create_key_routes() -> Router<Arc<RedisHandler>> {
    Router::new()
        .route("/", get(RedisHandler::list_keys))
        .route("/:key/exists", get(RedisHandler::key_exists))
        .route("/:key/ttl", get(RedisHandler::key_ttl))
        .route("/:key", delete(RedisHandler::delete_key))
}
