use crate::handlers::redis_ws::handler::RedisWsHandler;
use crate::models::RedisWsResponse;

pub async fn handle_key_keys(
    handler: &RedisWsHandler,
    pattern: Option<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let pattern = pattern.unwrap_or_else(|| "*".to_string());
    let keys: Vec<String> = redis::cmd("KEYS").arg(&pattern).query(&mut conn)?;
    Ok(RedisWsResponse::ArrayValue { value: keys })
}

pub async fn handle_key_del(
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
