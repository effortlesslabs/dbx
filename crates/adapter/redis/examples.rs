//! Redis adapter examples
//!
//! This file contains examples of how to use the Redis adapter for various use cases.
//! It demonstrates the use of individual commands, pipelined operations, transactions,
//! and Lua scripts.

use super::Redis;
use redis::RedisResult;

/// Simple key-value operations with Redis strings
pub fn string_operations_example(redis: &Redis) -> RedisResult<()> {
    let redis_string = redis.string();

    // Basic set and get
    redis_string.set("example:name", "Redis Adapter")?;
    let name: Option<String> = redis_string.get("example:name")?;
    println!("Name: {:?}", name);

    // Numeric operations
    redis_string.set("example:counter", "10")?;
    let incremented = redis_string.incr("example:counter")?;
    println!("Incremented counter: {}", incremented);

    // Key with expiration
    redis_string.set_with_expiry("example:session", "temporary-data", 60)?; // 60 seconds TTL

    // Append operation
    redis_string.append("example:name", " v1.0")?;
    let updated_name: Option<String> = redis_string.get("example:name")?;
    println!("Updated name: {:?}", updated_name);

    Ok(())
}

/// Batch operations using pipelines
pub fn batch_operations_example(redis: &Redis) -> RedisResult<()> {
    // Use the BatchOperations helper
    let batch = Redis::batch();

    // Set multiple keys at once
    let user_data = vec![
        ("user:1:name", "Alice"),
        ("user:1:email", "alice@example.com"),
        ("user:1:role", "admin"),
        ("user:2:name", "Bob"),
        ("user:2:email", "bob@example.com"),
        ("user:2:role", "user"),
    ];
    batch.set_many(redis, user_data)?;

    // Get multiple keys at once
    let keys = vec!["user:1:name", "user:1:email", "user:2:name"];
    let values = batch.get_many(redis, keys)?;
    println!("Batch retrieved values: {:?}", values);

    // Set multiple keys with expiration
    let session_data = vec![
        ("session:1", "token123", 3600), // 1 hour
        ("session:2", "token456", 1800), // 30 minutes
    ];
    batch.set_many_with_expiry(redis, session_data)?;

    // Increment multiple counters
    let counters = vec!["stats:visits", "stats:api_calls", "stats:errors"];
    let new_values = batch.incr_many(redis, counters)?;
    println!("Updated counter values: {:?}", new_values);

    // Custom amounts for increment
    let score_updates = vec![
        ("user:1:score", 10),
        ("user:2:score", 5),
        ("leaderboard:total", 15),
    ];
    let new_scores = batch.incr_many_by(redis, score_updates)?;
    println!("Updated scores: {:?}", new_scores);

    // Delete multiple keys
    let expired_keys = vec!["temp:1", "temp:2", "old_session:123"];
    batch.del_many(redis, expired_keys)?;

    Ok(())
}

/// Transaction example with MULTI/EXEC
pub fn transaction_example(redis: &Redis) -> RedisResult<()> {
    let redis_string = redis.string();

    // Execute multiple commands in a transaction (all or nothing)
    let results: ((), i64, Option<String>) = redis_string.transaction(|pipe| {
        pipe.cmd("SET")
            .arg("tx:key1")
            .arg("transaction-value")
            .cmd("INCRBY")
            .arg("tx:counter")
            .arg(5)
            .cmd("GET")
            .arg("tx:key1")
    })?;

    println!(
        "Transaction results: Counter={}, Value={:?}",
        results.1, results.2
    );

    Ok(())
}

/// Lua script examples
pub fn lua_script_example(redis: &Redis) -> RedisResult<()> {
    let redis_string = redis.string();

    // Use a predefined script: get and set atomically
    let get_set_script = super::scripts::get_set();
    let old_value: Option<String> =
        redis_string.eval_script(&get_set_script, &["script:key1"], &["new-value"])?;
    println!("Old value from GetSet: {:?}", old_value);

    // Rate limiter script example
    let rate_limiter = super::scripts::rate_limiter();
    let allowed: i64 = redis_string.eval_script(
        &rate_limiter,
        &["rate:limit:user:123"],
        &[5, 60], // 5 requests per 60 seconds
    )?;

    if allowed == 1 {
        println!("Request allowed");
    } else {
        println!("Rate limit exceeded");
    }

    // Custom script for specific business logic
    // Create a custom script for price updates
    let custom_script = super::primitives::string::RedisString::create_script(
        r#"
        local current = redis.call('GET', KEYS[1])
        if current and tonumber(current) > tonumber(ARGV[1]) then
            redis.call('SET', KEYS[1], ARGV[1])
            return 1
        else
            return 0
        end
    "#,
    );

    let updated: i64 = redis_string.eval_script(&custom_script, &["product:max_price"], &[100])?;

    println!("Price updated: {}", updated);

    Ok(())
}

/// Combining features: transaction + pipeline + script
pub fn combined_example(redis: &Redis) -> RedisResult<()> {
    let redis_string = redis.string();

    // Create a script for checking and updating inventory
    let inventory_script = super::primitives::string::RedisString::create_script(
        r#"
        local current = tonumber(redis.call('GET', KEYS[1]) or "0")
        local requested = tonumber(ARGV[1])

        if current >= requested then
            redis.call('DECRBY', KEYS[1], requested)
            return 1
        else
            return 0
        end
    "#,
    );

    // Transaction that processes an order
    let transaction_result: (i64, (), ()) = redis_string.transaction(|pipe| {
        // First: check inventory using Lua script
        super::primitives::string::RedisString::add_script_to_pipeline(
            pipe,
            &inventory_script,
            &["inventory:product123"],
            &[3], // Order quantity
        );

        // Only if inventory check passes (we'll check the result outside the transaction)
        pipe.cmd("INCR")
            .arg("orders:count")
            .cmd("SADD")
            .arg("user:orders:456")
            .arg("order789")
    })?;

    if transaction_result.0 == 1 {
        println!("Order processed successfully");

        // Use a pipeline for non-critical follow-up operations
        redis_string.with_pipeline(|pipe| {
            pipe.cmd("LPUSH")
                .arg("order:queue")
                .arg("order789")
                .cmd("EXPIRE")
                .arg("order:details:789")
                .arg(86400) // 1 day
                .cmd("INCR")
                .arg("stats:daily:orders")
        })?;
    } else {
        println!("Order failed: insufficient inventory");
    }

    Ok(())
}

/// Usage example with connection pool (when feature is enabled)
#[cfg(feature = "connection-pool")]
pub async fn connection_pool_example(url: &str) -> RedisResult<()> {
    // Create a pool with 10 connections
    let pool = Redis::with_connection_pool(url, 10)?;

    // Get a Redis instance from the pool
    let redis = pool.get_instance()?;

    // Use the Redis instance
    let redis_string = redis.string();
    redis_string.set("pool:example", "connection from pool")?;

    // With async feature enabled
    #[cfg(feature = "async")]
    {
        // Get an async connection
        let async_conn = pool.get_async_connection().await?;

        // Use the async connection directly with async Redis commands
        let _: () = redis::AsyncCommands::set(&mut async_conn, "async:key", "async value").await?;
    }

    Ok(())
}

/// Example of how to use the Redis module in a real application
pub fn usage_example() -> RedisResult<()> {
    // Create a Redis instance
    let redis = Redis::from_url("redis://localhost:6379")?;

    // Basic operations
    string_operations_example(&redis)?;

    // Batch operations
    batch_operations_example(&redis)?;

    // Transactions
    transaction_example(&redis)?;

    // Lua scripts
    lua_script_example(&redis)?;

    // Combined features
    combined_example(&redis)?;

    println!("All examples executed successfully");

    Ok(())
}
