use crate::handlers::redis_ws::handler::RedisWsHandler;
use crate::models::RedisWsResponse;

pub async fn handle_set_sadd(
    handler: &RedisWsHandler,
    key: String,
    members: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let members_ref: Vec<&str> = members
        .iter()
        .map(|m| m.as_str())
        .collect();
    let result = redis.set().sadd(&key, &members_ref)?;
    Ok(RedisWsResponse::IntegerValue { value: result as i64 })
}

pub async fn handle_set_srem(
    handler: &RedisWsHandler,
    key: String,
    members: Vec<String>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let members_ref: Vec<&str> = members
        .iter()
        .map(|m| m.as_str())
        .collect();
    let result = redis.set().srem(&key, &members_ref)?;
    Ok(RedisWsResponse::IntegerValue { value: result as i64 })
}

pub async fn handle_set_smembers(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let results = redis.set().smembers(&key)?;
    Ok(RedisWsResponse::ArrayValue { value: results })
}

pub async fn handle_set_scard(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.set().scard(&key)?;
    Ok(RedisWsResponse::IntegerValue { value: result as i64 })
}

pub async fn handle_set_sismember(
    handler: &RedisWsHandler,
    key: String,
    member: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.set().sismember(&key, &member)?;
    Ok(RedisWsResponse::BooleanValue { value: result })
}

pub async fn handle_set_spop(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.set().spop(&key)?;
    Ok(RedisWsResponse::StringValue { value: result })
}
