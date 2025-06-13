#[cfg(test)]
mod tests {
    use super::*;
    use dbx_core::config::DbConfig;
    use serde_json::json;
    use std::time::Duration;
    use tokio;
    use test_log::test;

    const TEST_REDIS_URL: &str = "redis://127.0.0.1:6379";

    async fn setup_test_db() -> RedisDatabase {
        let config = DbConfig {
            url: TEST_REDIS_URL.to_string(),
            ..Default::default()
        };
        let db = RedisDatabase::new(config);
        db.connect().await.expect("Failed to connect to Redis");
        db
    }

    #[tokio::test]
    async fn test_basic_operations() {
        let db = setup_test_db().await;
        let mut commands = db.commands.lock().await;

        // Test SET/GET
        commands.set("test_key", "test_value").await.expect("Failed to set key");
        let value: String = commands.get("test_key").await.expect("Failed to get key");
        assert_eq!(value, "test_value");

        // Test DEL
        commands.del("test_key").await.expect("Failed to delete key");
        let result: Option<String> = commands.get("test_key").await.expect("Failed to get key");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_hash_operations() {
        let db = setup_test_db().await;
        let mut commands = db.commands.lock().await;

        // Test HSET/HGET
        commands.hset("test_hash", "field1", "value1").await.expect("Failed to hset");
        let value: String = commands.hget("test_hash", "field1").await.expect("Failed to hget");
        assert_eq!(value, "value1");

        // Test HGETALL
        let hash: HashMap<String, String> = commands
            .hgetall("test_hash").await
            .expect("Failed to hgetall");
        assert_eq!(hash.get("field1"), Some(&"value1".to_string()));
    }

    #[tokio::test]
    async fn test_list_operations() {
        let db = setup_test_db().await;
        let mut commands = db.commands.lock().await;

        // Test LPUSH/LPOP
        commands.lpush("test_list", "value1").await.expect("Failed to lpush");
        let value: String = commands.lpop("test_list").await.expect("Failed to lpop");
        assert_eq!(value, "value1");
    }

    #[tokio::test]
    async fn test_set_operations() {
        let db = setup_test_db().await;
        let mut commands = db.commands.lock().await;

        // Test SADD/SMEMBERS
        commands.sadd("test_set", "member1").await.expect("Failed to sadd");
        let members: Vec<String> = commands.smembers("test_set").await.expect("Failed to smembers");
        assert!(members.contains(&"member1".to_string()));
    }

    #[tokio::test]
    async fn test_sorted_set_operations() {
        let db = setup_test_db().await;
        let mut commands = db.commands.lock().await;

        // Test ZADD/ZRANGE
        commands.zadd("test_zset", "member1", 1.0).await.expect("Failed to zadd");
        let members: Vec<String> = commands
            .zrange("test_zset", 0, -1).await
            .expect("Failed to zrange");
        assert!(members.contains(&"member1".to_string()));
    }

    #[tokio::test]
    async fn test_transactions() {
        let db = setup_test_db().await;
        let mut transaction = db.transaction.lock().await;

        // Test transaction
        transaction.begin().await.expect("Failed to begin transaction");
        transaction.set("key1", "value1").await.expect("Failed to set in transaction");
        transaction.set("key2", "value2").await.expect("Failed to set in transaction");
        transaction.commit().await.expect("Failed to commit transaction");

        // Verify transaction results
        let mut commands = db.commands.lock().await;
        let value1: String = commands.get("key1").await.expect("Failed to get key1");
        let value2: String = commands.get("key2").await.expect("Failed to get key2");
        assert_eq!(value1, "value1");
        assert_eq!(value2, "value2");
    }

    #[tokio::test]
    async fn test_prepared_statements() {
        let db = setup_test_db().await;
        let mut prepared = db.prepared.lock().await;

        // Test prepared statement
        let query = prepared
            .prepare("SET {key} {value}").await
            .expect("Failed to prepare statement");
        let params = vec![
            QueryParam::String("test_key".to_string()),
            QueryParam::String("test_value".to_string())
        ];
        prepared.execute(&query, &params).await.expect("Failed to execute prepared statement");

        // Verify result
        let mut commands = db.commands.lock().await;
        let value: String = commands.get("test_key").await.expect("Failed to get key");
        assert_eq!(value, "test_value");
    }

    #[tokio::test]
    async fn test_pubsub() {
        let db = setup_test_db().await;
        let mut pubsub = db.pubsub.lock().await;

        // Test subscribe and publish
        let channel = "test_channel";
        let message = "test_message";

        // Subscribe in a separate task
        let mut pubsub_clone = pubsub.clone();
        tokio::spawn(async move {
            let mut messages = pubsub_clone.listen(channel).await.expect("Failed to listen");
            let received = messages.next().await.expect("Failed to receive message");
            assert_eq!(received.channel, channel);
            assert_eq!(received.message, message);
        });

        // Publish message
        pubsub.publish(channel, message).await.expect("Failed to publish");
    }

    #[tokio::test]
    async fn test_script() {
        let db = setup_test_db().await;
        let mut script = db.script.lock().await;

        // Test Lua script
        let script_content =
            r#"
            redis.call('SET', KEYS[1], ARGV[1])
            return redis.call('GET', KEYS[1])
        "#;
        let script_hash = script.load(script_content).await.expect("Failed to load script");

        let result: String = script
            .execute(&script_hash, &["test_key"], &["test_value"]).await
            .expect("Failed to execute script");

        assert_eq!(result, "test_value");
    }

    #[tokio::test]
    async fn test_pipeline() {
        let db = setup_test_db().await;
        let mut pipeline = db.pipeline.lock().await;

        // Test pipeline operations
        pipeline.set("key1", "value1").await.expect("Failed to set in pipeline");
        pipeline.set("key2", "value2").await.expect("Failed to set in pipeline");
        let results: Vec<String> = pipeline.execute().await.expect("Failed to execute pipeline");

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "OK");
        assert_eq!(results[1], "OK");
    }

    #[tokio::test]
    async fn test_stream() {
        let db = setup_test_db().await;
        let mut stream = db.stream.lock().await;

        // Test stream operations
        let key = "test_stream";
        let fields = vec![("field1", "value1"), ("field2", "value2")];
        let id = stream.add(key, &fields).await.expect("Failed to add to stream");

        // Read the message
        let messages = stream.read(key, "0", Some(1)).await.expect("Failed to read from stream");
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, id);
        assert_eq!(messages[0].fields.get("field1"), Some(&"value1".to_string()));
        assert_eq!(messages[0].fields.get("field2"), Some(&"value2".to_string()));

        // Test consumer group
        let group = "test_group";
        let consumer = "test_consumer";
        stream.create_group(key, group, "0").await.expect("Failed to create group");

        let group_messages = stream
            .read_group(key, group, consumer, Some(1)).await
            .expect("Failed to read from group");
        assert_eq!(group_messages.len(), 1);

        // Acknowledge message
        stream.ack(key, group, &[&id]).await.expect("Failed to ack message");
    }

    #[tokio::test]
    async fn test_hyperloglog() {
        let db = setup_test_db().await;
        let mut hll = db.hll.lock().await;

        // Test HyperLogLog operations
        let key = "test_hll";
        let elements = vec!["element1", "element2", "element3"];

        // Add elements
        let changed = hll.add(key, &elements).await.expect("Failed to add to HLL");
        assert!(changed);

        // Count elements
        let count = hll.count(key).await.expect("Failed to count HLL");
        assert_eq!(count, 3);

        // Test merging
        let key2 = "test_hll2";
        hll.add(key2, &["element4", "element5"]).await.expect("Failed to add to second HLL");
        hll.merge("merged_hll", &[key, key2]).await.expect("Failed to merge HLLs");

        let merged_count = hll.count("merged_hll").await.expect("Failed to count merged HLL");
        assert_eq!(merged_count, 5);
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let db = setup_test_db().await;

        // Test connection pool
        let pool_size = 5;
        for _ in 0..pool_size {
            db.get_connection().await.expect("Failed to get connection");
        }

        // Verify pool size
        let mut conn = db.connection.lock().await;
        assert_eq!(conn.pool_size(), pool_size);
    }

    #[tokio::test]
    async fn test_error_handling() {
        let db = setup_test_db().await;
        let mut commands = db.commands.lock().await;

        // Test invalid data type
        let result = commands.set("test_key", 123).await;
        assert!(result.is_err());

        // Test non-existent key
        let result: Option<String> = commands
            .get("non_existent_key").await
            .expect("Failed to get key");
        assert!(result.is_none());

        // Test invalid command
        let result = commands.query("INVALID_COMMAND").await;
        assert!(result.is_err());
    }
}
