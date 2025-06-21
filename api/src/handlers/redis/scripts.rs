use axum::{extract::State, http::StatusCode, response::Json};
use redis::Script;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::debug;

use crate::{
    handlers::redis::RedisHandler,
    middleware::handle_redis_error,
    models::{ApiResponse, BooleanValue, IntegerValue, KeyValues},
};

pub async fn rate_limiter_script(
    State(handler): State<Arc<RedisHandler>>,
    Json(request): Json<RateLimiterRequest>,
) -> Result<Json<ApiResponse<BooleanValue>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /scripts/rate-limiter");

    let script = Script::new(
        r#"
        local key = KEYS[1]
        local limit = tonumber(ARGV[1])
        local window = tonumber(ARGV[2])
        
        local current = redis.call('GET', key)
        if current == false then
            redis.call('SETEX', key, window, 1)
            return 1
        end
        
        local count = tonumber(current)
        if count >= limit then
            return 0
        end
        
        redis.call('INCR', key)
        return 1
    "#,
    );

    let key_slice: [&str; 1] = [&request.key];
    let arg_slice: [String; 2] = [request.limit.to_string(), request.window.to_string()];
    let result: i32 = match handler
        .redis
        .string()
        .eval_script::<i32, _, _>(&script, &key_slice, &arg_slice)
    {
        Ok(result) => result,
        Err(e) => {
            return Err(handle_redis_error(e));
        }
    };

    Ok(Json(ApiResponse::success(BooleanValue {
        value: result == 1,
    })))
}

pub async fn multi_counter_script(
    State(handler): State<Arc<RedisHandler>>,
    Json(request): Json<MultiCounterRequest>,
) -> Result<Json<ApiResponse<Vec<IntegerValue>>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /scripts/multi-counter");

    let script = Script::new(
        r#"
        local results = {}
        for i, counter in ipairs(ARGV) do
            local key, increment = string.match(counter, "([^:]+):([^:]+)")
            local result = redis.call('INCRBY', key, tonumber(increment))
            table.insert(results, result)
        end
        return results
    "#,
    );

    let args: Vec<String> = request
        .counters
        .iter()
        .map(|(key, increment)| format!("{}:{}", key, increment))
        .collect();

    let result: Vec<i64> = match handler
        .redis
        .string()
        .eval_script::<Vec<i64>, &[&str], &[String]>(&script, &[], &args)
    {
        Ok(result) => result,
        Err(e) => {
            return Err(handle_redis_error(e));
        }
    };

    let values: Vec<IntegerValue> = result.iter().map(|v| IntegerValue { value: *v }).collect();

    Ok(Json(ApiResponse::success(values)))
}

pub async fn multi_set_ttl_script(
    State(handler): State<Arc<RedisHandler>>,
    Json(request): Json<MultiSetTtlRequest>,
) -> Result<Json<ApiResponse<KeyValues>>, (StatusCode, Json<ApiResponse<()>>)> {
    debug!("POST /scripts/multi-set-ttl");

    let script = Script::new(
        r#"
        local ttl = tonumber(ARGV[1])
        local results = {}
        
        for i = 2, #ARGV, 2 do
            local key = ARGV[i]
            local value = ARGV[i + 1]
            redis.call('SETEX', key, ttl, value)
            results[key] = value
        end
        
        return results
    "#,
    );

    let mut args = vec![request.ttl.to_string()];
    for (key, value) in &request.key_values {
        args.push(key.clone());
        args.push(value.clone());
    }

    let _: () = match handler
        .redis
        .string()
        .eval_script::<(), &[&str], &[String]>(&script, &[], &args)
    {
        Ok(_) => (),
        Err(e) => {
            return Err(handle_redis_error(e));
        }
    };

    Ok(Json(ApiResponse::success(KeyValues {
        key_values: request.key_values,
    })))
}

pub struct RateLimiterRequest {
    pub key: String,
    pub limit: i64,
    pub window: i64,
}

pub struct MultiCounterRequest {
    pub counters: Vec<(String, i64)>,
}

pub struct MultiSetTtlRequest {
    pub key_values: HashMap<String, String>,
    pub ttl: u64,
}
