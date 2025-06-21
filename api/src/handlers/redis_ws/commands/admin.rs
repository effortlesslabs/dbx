use crate::handlers::redis_ws::handler::RedisWsHandler;
use crate::models::RedisWsResponse;

pub async fn handle_admin_flush_all(
    handler: &RedisWsHandler
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let _: String = redis::cmd("FLUSHALL").query(&mut conn)?;
    Ok(RedisWsResponse::BooleanValue { value: true })
}

pub async fn handle_admin_flush_db(
    handler: &RedisWsHandler
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let _: String = redis::cmd("FLUSHDB").query(&mut conn)?;
    Ok(RedisWsResponse::BooleanValue { value: true })
}

pub async fn handle_admin_db_size(
    handler: &RedisWsHandler
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let size: i64 = redis::cmd("DBSIZE").query(&mut conn)?;
    Ok(RedisWsResponse::IntegerValue { value: size })
}

pub async fn handle_admin_info(
    handler: &RedisWsHandler
) -> Result<RedisWsResponse, Box<dyn std::error::Error + Send + Sync>> {
    let redis = handler.get_redis().await?;
    let mut conn = redis.get_connection()?;
    let info: String = redis::cmd("INFO").query(&mut conn)?;
    Ok(RedisWsResponse::StringValue { value: Some(info) })
}
