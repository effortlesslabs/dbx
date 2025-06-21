use crate::handlers::redis_ws::handler::RedisWsHandler;
use crate::models::RedisWsResponse;

pub async fn handle_batch_get(
    handler: &RedisWsHandler,
    keys: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let key_refs: Vec<&str> = keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    let results = redis.string().get_many(key_refs)?;
    let values: Vec<String> = results
        .into_iter()
        .filter_map(|v| v)
        .collect();
    Ok(RedisWsResponse::ArrayValue { value: values })
}

pub async fn handle_batch_set(
    handler: &RedisWsHandler,
    key_values: std::collections::HashMap<String, String>,
    ttl: Option<u64>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let kvs: Vec<(&str, &str)> = key_values
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    if let Some(ttl_seconds) = ttl {
        let kvs_with_ttl: Vec<(&str, &str, usize)> = key_values
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str(), ttl_seconds as usize))
            .collect();
        redis.string().set_many_with_expiry(kvs_with_ttl)?;
    } else {
        redis.string().set_many(kvs)?;
    }
    Ok(RedisWsResponse::BooleanValue { value: true })
}

pub async fn handle_batch_delete(
    handler: &RedisWsHandler,
    keys: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let key_refs: Vec<&str> = keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    redis.string().del_many(key_refs)?;
    Ok(RedisWsResponse::IntegerValue { value: keys.len() as i64 })
}

pub async fn handle_batch_incr(
    handler: &RedisWsHandler,
    keys: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let key_refs: Vec<&str> = keys
        .iter()
        .map(|k| k.as_str())
        .collect();
    let results = redis.string().incr_many(key_refs)?;
    let sum: i64 = results.iter().sum();
    Ok(RedisWsResponse::IntegerValue { value: sum })
}

pub async fn handle_batch_incr_by(
    handler: &RedisWsHandler,
    key_increments: Vec<(String, i64)>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut total = 0i64;
    for (key, increment) in key_increments {
        let result = redis.string().incr_by(&key, increment)?;
        total += result;
    }
    Ok(RedisWsResponse::IntegerValue { value: total })
}
