use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;
use tracing::debug;

use crate::{
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{ApiResponse, BooleanValue, DeleteResponse, IntegerValue, KeyValues, StringValue},
};

impl RedisHandler {
    // Set operation handlers

    pub async fn get_set_members(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}", key);

        match handler.redis.set().smembers(&key) {
            Ok(members) => {
                let mut map = std::collections::HashMap::new();
                map.insert(key, members.join(","));
                Ok(Json(ApiResponse::success(KeyValues { key_values: map })))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn add_set_members(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<crate::models::SetRequest>,
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}", key);

        match handler.redis.set().sadd(&key, &[&request.value]) {
            Ok(_) => Ok(Json(ApiResponse::success(BooleanValue { value: true }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_set(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /sets/{}", key);

        match handler.redis.set().del(&key) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse {
                deleted_count: 1,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_member_exists(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        axum::extract::Query(query): axum::extract::Query<
            std::collections::HashMap<String, String>,
        >,
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}/exists", key);

        let member = query.get("member").cloned().unwrap_or_default();
        match handler.redis.set().sismember(&key, &member) {
            Ok(exists) => Ok(Json(ApiResponse::success(BooleanValue { value: exists }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_set_cardinality(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}/cardinality", key);

        match handler.redis.set().scard(&key) {
            Ok(cardinality) => Ok(Json(ApiResponse::success(IntegerValue {
                value: cardinality as i64,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_random_member(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}/random", key);

        match handler.redis.set().srandmember(&key) {
            Ok(Some(member)) => Ok(Json(ApiResponse::success(StringValue { value: member }))),
            Ok(None) => Ok(Json(ApiResponse::success(StringValue {
                value: String::new(),
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn pop_random_member(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/pop", key);

        match handler.redis.set().spop(&key) {
            Ok(Some(member)) => Ok(Json(ApiResponse::success(StringValue { value: member }))),
            Ok(None) => Ok(Json(ApiResponse::success(StringValue {
                value: String::new(),
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn move_set_member(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/move", key);

        let member = request.get("member").cloned().unwrap_or_default();
        let destination = request.get("destination").cloned().unwrap_or_default();

        match handler.redis.set().smove(&key, &destination, &member) {
            Ok(moved) => Ok(Json(ApiResponse::success(BooleanValue { value: moved }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_union(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(mut keys): Json<Vec<String>>,
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/union", key);

        keys.push(key.clone());
        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match handler.redis.set().sunion(&key_refs) {
            Ok(members) => {
                let mut map = std::collections::HashMap::new();
                map.insert(key, members.join(","));
                Ok(Json(ApiResponse::success(KeyValues { key_values: map })))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_intersection(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(mut keys): Json<Vec<String>>,
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/intersection", key);

        keys.push(key.clone());
        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match handler.redis.set().sinter(&key_refs) {
            Ok(members) => {
                let mut map = std::collections::HashMap::new();
                map.insert(key, members.join(","));
                Ok(Json(ApiResponse::success(KeyValues { key_values: map })))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_difference(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(mut keys): Json<Vec<String>>,
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/difference", key);

        keys.push(key.clone());
        let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
        match handler.redis.set().sdiff(&key_refs) {
            Ok(members) => {
                let mut map = std::collections::HashMap::new();
                map.insert(key, members.join(","));
                Ok(Json(ApiResponse::success(KeyValues { key_values: map })))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_add_set_members(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<std::collections::HashMap<String, Vec<String>>>,
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/batch/add");

        let mut total_added = 0u64;
        let set_members: Vec<(&str, Vec<&str>)> = request
            .iter()
            .map(|(key, members)| {
                total_added += members.len() as u64;
                (key.as_str(), members.iter().map(|s| s.as_str()).collect())
            })
            .collect();

        match handler.redis.set().sadd_many(set_members) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse {
                deleted_count: total_added,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_remove_set_members(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<std::collections::HashMap<String, Vec<String>>>,
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/batch/remove");

        let mut total_removed = 0u64;
        let set_members: Vec<(&str, Vec<&str>)> = request
            .iter()
            .map(|(key, members)| {
                total_removed += members.len() as u64;
                (key.as_str(), members.iter().map(|s| s.as_str()).collect())
            })
            .collect();

        match handler.redis.set().srem_many(set_members) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse {
                deleted_count: total_removed,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_get_set_members(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>,
    ) -> Result<
        Json<ApiResponse<std::collections::HashMap<String, Vec<String>>>>,
        (StatusCode, Json<ApiResponse<()>>),
    > {
        debug!("POST /sets/batch/members");

        let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
        match handler.redis.set().smembers_many(key_refs) {
            Ok(members_vec) => {
                let mut result = std::collections::HashMap::new();
                for (key, members) in keys.iter().zip(members_vec.iter()) {
                    result.insert(key.clone(), members.clone());
                }
                Ok(Json(ApiResponse::success(result)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_delete_sets(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>,
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/batch/delete");

        let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
        match handler.redis.string().del_many(key_refs) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse {
                deleted_count: keys.len() as u64,
            }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}
