use crate::handlers::redis_ws::handler::RedisWsHandler;
use crate::models::RedisWsResponse;

pub async fn handle_string_get(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    redis
        .string()
        .get(&key)
        .map(|value| RedisWsResponse::StringValue { value })
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
}

pub async fn handle_string_set(
    handler: &RedisWsHandler,
    key: String,
    value: String,
    ttl: Option<u64>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    if let Some(ttl_seconds) = ttl {
        redis.string().set_with_expiry(&key, &value, ttl_seconds as usize)?;
    } else {
        redis.string().set(&key, &value)?;
    }
    Ok(RedisWsResponse::BooleanValue { value: true })
}

pub async fn handle_string_delete(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    redis.string().del(&key)?;
    Ok(RedisWsResponse::BooleanValue { value: true })
}

pub async fn handle_string_exists(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.string().exists(&key)?;
    Ok(RedisWsResponse::BooleanValue { value: result })
}

pub async fn handle_string_ttl(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.string().ttl(&key)?;
    Ok(RedisWsResponse::IntegerValue { value: result })
}

pub async fn handle_string_incr(
    handler: &RedisWsHandler,
    key: String
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.string().incr(&key)?;
    Ok(RedisWsResponse::IntegerValue { value: result })
}

pub async fn handle_string_incr_by(
    handler: &RedisWsHandler,
    key: String,
    increment: i64
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let result = redis.string().incr_by(&key, increment)?;
    Ok(RedisWsResponse::IntegerValue { value: result })
}

pub async fn handle_string_set_nx(
    handler: &RedisWsHandler,
    key: String,
    value: String,
    ttl: Option<u64>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let exists = redis.string().exists(&key)?;
    if !exists {
        if let Some(ttl_seconds) = ttl {
            redis.string().set_with_expiry(&key, &value, ttl_seconds as usize)?;
        } else {
            redis.string().set(&key, &value)?;
        }
        Ok(RedisWsResponse::BooleanValue { value: true })
    } else {
        Ok(RedisWsResponse::BooleanValue { value: false })
    }
}

pub async fn handle_string_compare_and_set(
    handler: &RedisWsHandler,
    key: String,
    expected_value: String,
    new_value: String,
    ttl: Option<u64>
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let current_value = redis.string().get(&key)?;
    if current_value.as_ref() == Some(&expected_value) {
        if let Some(ttl_seconds) = ttl {
            redis.string().set_with_expiry(&key, &new_value, ttl_seconds as usize)?;
        } else {
            redis.string().set(&key, &new_value)?;
        }
        Ok(RedisWsResponse::BooleanValue { value: true })
    } else {
        Ok(RedisWsResponse::BooleanValue { value: false })
    }
}
