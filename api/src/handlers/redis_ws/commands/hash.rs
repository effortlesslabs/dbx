use crate::handlers::redis_ws::handler::RedisWsHandler;
use crate::models::RedisWsResponse;
use std::collections::HashMap;

pub async fn handle_hash_hset(
    handler: &RedisWsHandler,
    key: String,
    field: String,
    value: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.hash().hset(&key, &field, &value)?;
    Ok(RedisWsResponse::BooleanValue { value: result })
}

pub async fn handle_hash_hget(
    handler: &RedisWsHandler,
    key: String,
    field: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.hash().hget(&key, &field)?;
    Ok(RedisWsResponse::StringValue { value: result })
}

pub async fn handle_hash_hdel(
    handler: &RedisWsHandler,
    key: String,
    field: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let fields = vec![field.as_str()];
    let result = redis.hash().hdel(&key, &fields)?;
    Ok(RedisWsResponse::IntegerValue { value: result as i64 })
}

pub async fn handle_hash_hexists(
    handler: &RedisWsHandler,
    key: String,
    field: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.hash().hexists(&key, &field)?;
    Ok(RedisWsResponse::BooleanValue { value: result })
}

pub async fn handle_hash_hlen(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.hash().hlen(&key)?;
    Ok(RedisWsResponse::IntegerValue { value: result as i64 })
}

pub async fn handle_hash_hkeys(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let results = redis.hash().hkeys(&key)?;
    Ok(RedisWsResponse::ArrayValue { value: results })
}

pub async fn handle_hash_hvals(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let results = redis.hash().hvals(&key)?;
    Ok(RedisWsResponse::ArrayValue { value: results })
}

pub async fn handle_hash_hgetall(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let results = redis.hash().hgetall(&key)?;
    Ok(RedisWsResponse::ObjectValue { value: results })
}

pub async fn handle_hash_hmset(
    handler: &RedisWsHandler,
    key: String,
    fields: HashMap<String, String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let fields_ref: Vec<(&str, &str)> = fields
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    redis.hash().hmset(&key, &fields_ref)?;
    Ok(RedisWsResponse::BooleanValue { value: true })
}

pub async fn handle_hash_hmget(
    handler: &RedisWsHandler,
    key: String,
    fields: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let fields_ref: Vec<&str> = fields
        .iter()
        .map(|f| f.as_str())
        .collect();
    let results = redis.hash().hmget(&key, &fields_ref)?;
    Ok(RedisWsResponse::ArrayValue {
        value: results
            .into_iter()
            .map(|v| v.unwrap_or_default())
            .collect(),
    })
}
