use axum::{ extract::{ Path, State }, http::StatusCode, response::Json };
use std::sync::Arc;
use tracing::debug;

use crate::{
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{ ApiResponse, BooleanValue, DeleteResponse, IntegerValue, KeyValues, StringValue },
};

impl RedisHandler {
    // Hash operation handlers

    pub async fn get_hash_field(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, field)): Path<(String, String)>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /hashes/{}/{}", key, field);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hget(&key, &field) {
            Ok(Some(value)) => Ok(Json(ApiResponse::success(StringValue { value }))),
            Ok(None) =>
                Ok(
                    Json(
                        ApiResponse::success(StringValue {
                            value: String::new(),
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_hash_field(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, field)): Path<(String, String)>,
        Json(request): Json<crate::models::SetRequest>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/{}/{}", key, field);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hset(&key, &field, &request.value) {
            Ok(_) =>
                Ok(
                    Json(
                        ApiResponse::success(StringValue {
                            value: request.value,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_hash_field(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, field)): Path<(String, String)>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /hashes/{}/{}", key, field);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hdel(&key, &[&field]) {
            Ok(_) =>
                Ok(
                    Json(
                        ApiResponse::success(DeleteResponse {
                            deleted_count: 1,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn hash_field_exists(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, field)): Path<(String, String)>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /hashes/{}/{}/exists", key, field);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hexists(&key, &field) {
            Ok(exists) => Ok(Json(ApiResponse::success(BooleanValue { value: exists }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn increment_hash_field(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, field)): Path<(String, String)>,
        Json(request): Json<crate::models::IncrByRequest>
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/{}/{}/incr", key, field);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hincrby(&key, &field, request.increment) {
            Ok(value) => Ok(Json(ApiResponse::success(IntegerValue { value }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_hash_field_nx(
        State(handler): State<Arc<RedisHandler>>,
        Path((key, field)): Path<(String, String)>,
        Json(request): Json<crate::models::SetRequest>
    ) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/{}/{}/setnx", key, field);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hsetnx(&key, &field, &request.value) {
            Ok(success) => Ok(Json(ApiResponse::success(BooleanValue { value: success }))),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_hash_length(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<IntegerValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /hashes/{}/length", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hlen(&key) {
            Ok(length) =>
                Ok(
                    Json(
                        ApiResponse::success(IntegerValue {
                            value: length as i64,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_hash_keys(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<Vec<StringValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /hashes/{}/keys", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hkeys(&key) {
            Ok(keys) => {
                let string_values: Vec<StringValue> = keys
                    .into_iter()
                    .map(|k| StringValue { value: k })
                    .collect();
                Ok(Json(ApiResponse::success(string_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_hash_values(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<Vec<StringValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /hashes/{}/values", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hvals(&key) {
            Ok(values) => {
                let string_values: Vec<StringValue> = values
                    .into_iter()
                    .map(|v| StringValue { value: v })
                    .collect();
                Ok(Json(ApiResponse::success(string_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_random_hash_field(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<StringValue>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /hashes/{}/random", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hrandfield(&key) {
            Ok(Some(field)) => Ok(Json(ApiResponse::success(StringValue { value: field }))),
            Ok(None) =>
                Ok(
                    Json(
                        ApiResponse::success(StringValue {
                            value: String::new(),
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_multiple_hash_fields(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(fields): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<Vec<StringValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/{}/mget", key);

        let field_refs: Vec<&str> = fields
            .iter()
            .map(|f| f.as_str())
            .collect();
        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hmget(&key, &field_refs) {
            Ok(values) => {
                let string_values: Vec<StringValue> = values
                    .into_iter()
                    .map(|v| StringValue {
                        value: v.unwrap_or_default(),
                    })
                    .collect();
                Ok(Json(ApiResponse::success(string_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn get_hash_all(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("GET /hashes/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hgetall(&key) {
            Ok(hash_map) => {
                let key_values: std::collections::HashMap<String, String> = hash_map
                    .into_iter()
                    .collect();
                Ok(Json(ApiResponse::success(KeyValues { key_values })))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn set_hash_multiple(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>,
        Json(request): Json<crate::models::SetManyRequest>
    ) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/{}", key);

        let field_values: Vec<(&str, &str)> = request.key_values
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hmset(&key, &field_values) {
            Ok(_) =>
                Ok(
                    Json(
                        ApiResponse::success(KeyValues {
                            key_values: request.key_values,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn delete_hash(
        State(handler): State<Arc<RedisHandler>>,
        Path(key): Path<String>
    ) -> Result<Json<ApiResponse<DeleteResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("DELETE /hashes/{}", key);

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().del(&key) {
            Ok(_) =>
                Ok(
                    Json(
                        ApiResponse::success(DeleteResponse {
                            deleted_count: 1,
                        })
                    )
                ),
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    // Batch operations

    pub async fn batch_set_hash_fields(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<Vec<(String, Vec<(String, String)>)>>
    ) -> Result<Json<ApiResponse<Vec<BooleanValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/batch/set");

        let hash_fields: Vec<(&str, Vec<(&str, &str)>)> = request
            .iter()
            .map(|(key, fields)| {
                let field_refs: Vec<(&str, &str)> = fields
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
                    .collect();
                (key.as_str(), field_refs)
            })
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hset_many(hash_fields) {
            Ok(results) => {
                let boolean_values: Vec<BooleanValue> = results
                    .into_iter()
                    .map(|r| BooleanValue { value: r })
                    .collect();
                Ok(Json(ApiResponse::success(boolean_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_get_hash_fields(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<Vec<(String, String)>>
    ) -> Result<Json<ApiResponse<Vec<StringValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/batch/get");

        let hash_fields: Vec<(&str, &str)> = request
            .iter()
            .map(|(key, field)| (key.as_str(), field.as_str()))
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hget_many(hash_fields) {
            Ok(values) => {
                let string_values: Vec<StringValue> = values
                    .into_iter()
                    .map(|v| StringValue {
                        value: v.unwrap_or_default(),
                    })
                    .collect();
                Ok(Json(ApiResponse::success(string_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_delete_hash_fields(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<Vec<(String, Vec<String>)>>
    ) -> Result<Json<ApiResponse<Vec<DeleteResponse>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/batch/delete");

        let hash_fields: Vec<(&str, Vec<&str>)> = request
            .iter()
            .map(|(key, fields)| {
                let field_refs: Vec<&str> = fields
                    .iter()
                    .map(|f| f.as_str())
                    .collect();
                (key.as_str(), field_refs)
            })
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hdel_many(hash_fields) {
            Ok(results) => {
                let delete_responses: Vec<DeleteResponse> = results
                    .into_iter()
                    .map(|r| DeleteResponse {
                        deleted_count: r as u64,
                    })
                    .collect();
                Ok(Json(ApiResponse::success(delete_responses)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_get_hash_all(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<Vec<KeyValues>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/batch/all");

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

        match redis.hash().hgetall_many(key_refs) {
            Ok(results) => {
                let key_values_list: Vec<KeyValues> = results
                    .into_iter()
                    .map(|hash_map| {
                        let key_values: std::collections::HashMap<String, String> = hash_map
                            .into_iter()
                            .collect();
                        KeyValues { key_values }
                    })
                    .collect();
                Ok(Json(ApiResponse::success(key_values_list)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_check_hash_fields(
        State(handler): State<Arc<RedisHandler>>,
        Json(request): Json<Vec<(String, String)>>
    ) -> Result<Json<ApiResponse<Vec<BooleanValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/batch/exists");

        let hash_fields: Vec<(&str, &str)> = request
            .iter()
            .map(|(key, field)| (key.as_str(), field.as_str()))
            .collect();

        let redis = match handler.get_redis() {
            Ok(redis) => redis,
            Err(e) => {
                return Err(handle_redis_error(e));
            }
        };

        match redis.hash().hexists_many(hash_fields) {
            Ok(results) => {
                let boolean_values: Vec<BooleanValue> = results
                    .into_iter()
                    .map(|r| BooleanValue { value: r })
                    .collect();
                Ok(Json(ApiResponse::success(boolean_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }

    pub async fn batch_get_hash_lengths(
        State(handler): State<Arc<RedisHandler>>,
        Json(keys): Json<Vec<String>>
    ) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
        debug!("POST /hashes/batch/lengths");

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

        match redis.hash().hlen_many(key_refs) {
            Ok(results) => {
                let integer_values: Vec<IntegerValue> = results
                    .into_iter()
                    .map(|r| IntegerValue { value: r as i64 })
                    .collect();
                Ok(Json(ApiResponse::success(integer_values)))
            }
            Err(e) => Err(handle_redis_error(e)),
        }
    }
}
