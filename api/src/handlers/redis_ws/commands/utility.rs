use crate::handlers::redis_ws::handler::RedisWsHandler;
use crate::models::RedisWsResponse;

pub async fn handle_utility_list_keys(
    handler: &RedisWsHandler,
    pattern: Option<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let pattern = pattern.unwrap_or_else(|| "*".to_string());
    let keys: Vec<String> = redis::cmd("KEYS").arg(&pattern).query(&mut conn)?;
    Ok(RedisWsResponse::ArrayValue { value: keys })
}

pub async fn handle_utility_ping(
    handler: &RedisWsHandler
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.ping()?;
    Ok(RedisWsResponse::BooleanValue { value: result })
}

pub async fn handle_utility_subscribe(
    handler: &RedisWsHandler,
    channels: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let channel_refs: Vec<&str> = channels
        .iter()
        .map(|c| c.as_str())
        .collect();
    let _: Vec<String> = redis::cmd("SUBSCRIBE").arg(&channel_refs).query(&mut conn)?;
    Ok(RedisWsResponse::ArrayValue { value: channels })
}

pub async fn handle_utility_unsubscribe(
    handler: &RedisWsHandler,
    channels: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let channel_refs: Vec<&str> = channels
        .iter()
        .map(|c| c.as_str())
        .collect();
    let _: Vec<String> = redis::cmd("UNSUBSCRIBE").arg(&channel_refs).query(&mut conn)?;
    Ok(RedisWsResponse::ArrayValue { value: channels })
}
