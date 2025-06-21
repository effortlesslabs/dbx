use axum::{ extract::{ Path, State }, http::StatusCode, response::Json };
use std::sync::Arc;
use tracing::debug;

use crate::{
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{ ApiResponse, BooleanValue, DeleteResponse, IntegerValue, KeyValues, StringValue },
};

impl RedisHandler {
    // Set operation handlers

    pub async fn get_set_members(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<Vec<StringValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().smembers(&key) {
            Ok(members) => {
                let values: Vec<StringValue> = members
                    .iter()
                    .map(|m| StringValue { value: m.clone() })
                    .collect();
                Ok(Json(ApiResponse::success(values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn add_set_member(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<crate::models::SetRequest>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().sadd(&key, &[&request.value]) {
            Ok(_) => Ok(Json(ApiResponse::success(StringValue { value: request.value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_set(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /sets/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().del(&key) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: 1 }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_member_exists(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, member)): Path<(String, String)>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}/{}", key, member);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().sismember(&key, &member) {
            Ok(exists) => Ok(Json(ApiResponse::success(BooleanValue { value: exists }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_set_cardinality(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}/cardinality", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().scard(&key) {
            Ok(cardinality) =>
                Ok(Json(ApiResponse::success(IntegerValue { value: cardinality as i64 }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_random_member(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /sets/{}/random", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().srandmember(&key) {
            Ok(Some(member)) => Ok(Json(ApiResponse::success(StringValue { value: member }))),
            Ok(None) => Ok(Json(ApiResponse::success(StringValue { value: String::new() }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn pop_random_member(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/pop", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().spop(&key) {
            Ok(Some(member)) => Ok(Json(ApiResponse::success(StringValue { value: member }))),
            Ok(None) => Ok(Json(ApiResponse::success(StringValue { value: String::new() }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn move_set_member(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<std::collections::HashMap<String, String>>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/move", key);

        let member = request.get("member").cloned().unwrap_or_default();
        let destination = request.get("destination").cloned().unwrap_or_default();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().smove(&key, &destination, &member) {
            Ok(moved) => Ok(Json(ApiResponse::success(BooleanValue { value: moved }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_union(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(mut keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/union", key);

        keys.push(key.clone());
        let key_refs: Vec<&str> = keys
            .iter()
            .map(|k| k.as_str())
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().sunion(&key_refs) {
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
        Json(mut keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/intersection", key);

        keys.push(key.clone());
        let key_refs: Vec<&str> = keys
            .iter()
            .map(|k| k.as_str())
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().sinter(&key_refs) {
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
        Json(mut keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/{}/difference", key);

        keys.push(key.clone());
        let key_refs: Vec<&str> = keys
            .iter()
            .map(|k| k.as_str())
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().sdiff(&key_refs) {
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
        Json(request): Json<std::collections::HashMap<String, Vec<String>>>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/batch/add");

        let mut total_added = 0u64;
        let set_members: Vec<(&str, Vec<&str>)> = request
            .iter()
            .map(|(key, members)| {
                total_added += members.len() as u64;
                (
                    key.as_str(),
                    members
                        .iter()
                        .map(|s| s.as_str())
                        .collect(),
                )
            })
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().sadd_many(set_members) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: total_added }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_remove_set_members(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<std::collections::HashMap<String, Vec<String>>>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/batch/remove");

        let mut total_removed = 0u64;
        let set_members: Vec<(&str, Vec<&str>)> = request
            .iter()
            .map(|(key, members)| {
                total_removed += members.len() as u64;
                (
                    key.as_str(),
                    members
                        .iter()
                        .map(|s| s.as_str())
                        .collect(),
                )
            })
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().srem_many(set_members) {
            Ok(_) =>
                Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: total_removed }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_delete_sets(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /sets/batch/delete");

        let key_refs: Vec<&str> = keys
            .iter()
            .map(|s| s.as_str())
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.string().del_many(key_refs) {
            Ok(_) =>
                Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: keys.len() as u64 }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn remove_set_member(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, member)): Path<(String, String)>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /sets/{}/{}", key, member);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.set().srem(&key, &[&member]) {
            Ok(_) => Ok(Json(ApiResponse::success(DeleteResponse { deleted_count: 1 }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}

pub async fn batch_get_set_members(
    State(handler): State<Arc<RedisHandler>>,
    Json(keys): Json<Vec<String>>
) -> Result<Json<ApiResponse<Vec<SetMembersResponse>>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /sets/batch/members");

    let key_refs: Vec<&str> = keys
        .iter()
        .map(|k| k.as_str())
        .collect();

    let redis = match handler.get_redis() {
        Ok(redis) => redis,
        Err(e) => {
            return Err(handle_redis_error(e));
        }
    };

    match redis.set().smembers_many(key_refs) {
        Ok(results) => {
            let responses: Vec<SetMembersResponse> = keys
                .iter()
                .zip(results.iter())
                .map(|(key, members)| SetMembersResponse {
                    key: key.clone(),
                    members: members
                        .iter()
                        .map(|m| StringValue { value: m.clone() })
                        .collect(),
                })
                .collect();
            Ok(Json(ApiResponse::success(responses)))
        }
        Err(e) => Err(handle_redis_error(e)),
    }
}

pub struct SetMembersResponse {
    pub key: String,
    pub members: Vec<StringValue>,
}
