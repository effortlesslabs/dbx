use redis_rs::{ DbxClient, StringOperation };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = DbxClient::new("http://localhost:8080")?;

    println!("=== DBX Redis SDK Example ===\n");

    // String operations
    println!("1. String Operations:");
    println!("   Setting values...");

    // Set simple string
    client.string().set_simple("user:1:name", "Alice").await?;
    println!("   ✓ Set user:1:name = Alice");

    // Set string with TTL
    client.string().set_with_ttl("user:1:session", "abc123", 3600).await?;
    println!("   ✓ Set user:1:session = abc123 (TTL: 3600s)");

    // Get values
    let name = client.string().get("user:1:name").await?;
    println!("   ✓ Retrieved user:1:name = {:?}", name);

    let session = client.string().get("user:1:session").await?;
    println!("   ✓ Retrieved user:1:session = {:?}", session);

    // Get string info
    let info = client.string().info("user:1:name").await?;
    if let Some(info) = info {
        println!("   ✓ String info: key={}, size={}, ttl={:?}", info.key, info.size, info.ttl);
    }

    // Batch operations
    println!("\n2. Batch String Operations:");
    println!("   Setting multiple values...");

    let operations = vec![
        StringOperation {
            key: "user:2:name".to_string(),
            value: Some("Bob".to_string()),
            ttl: None,
        },
        StringOperation {
            key: "user:2:email".to_string(),
            value: Some("bob@example.com".to_string()),
            ttl: None,
        },
        StringOperation {
            key: "user:2:age".to_string(),
            value: Some("30".to_string()),
            ttl: Some(7200), // 2 hours
        }
    ];

    client.string().batch_set(&operations).await?;
    println!("   ✓ Batch set 3 values");

    // Batch get
    let keys = vec![
        "user:2:name".to_string(),
        "user:2:email".to_string(),
        "user:2:age".to_string()
    ];
    let values = client.string().batch_get(&keys).await?;
    println!("   ✓ Batch retrieved: {:?}", values);

    // Pattern search
    println!("\n3. Pattern Search:");
    let patterns = vec!["user:*:name".to_string()];
    let results = client.string().get_by_patterns(&patterns, Some(false)).await?;
    println!("   ✓ Pattern search results: {}", results);

    // Set operations
    println!("\n4. Set Operations:");
    println!("   Creating and manipulating sets...");

    // Create sets
    client.set().add_many("users:online", &["user:1", "user:2", "user:3"]).await?;
    println!("   ✓ Added 3 users to online set");

    client.set().add_many("users:premium", &["user:1", "user:4"]).await?;
    println!("   ✓ Added 2 users to premium set");

    client.set().add_many("users:admin", &["user:1", "user:5"]).await?;
    println!("   ✓ Added 2 users to admin set");

    // Get set members
    let online_users = client.set().members("users:online").await?;
    println!("   ✓ Online users: {:?}", online_users);

    let premium_users = client.set().members("users:premium").await?;
    println!("   ✓ Premium users: {:?}", premium_users);

    // Check membership
    let is_premium = client.set().contains("users:premium", "user:1").await?;
    println!("   ✓ User 1 is premium: {}", is_premium);

    let is_admin = client.set().contains("users:admin", "user:2").await?;
    println!("   ✓ User 2 is admin: {}", is_admin);

    // Get set sizes
    let online_count = client.set().size("users:online").await?;
    println!("   ✓ Online users count: {}", online_count);

    // Set operations
    println!("\n5. Set Operations (Intersect, Union, Difference):");

    let premium_online = client
        .set()
        .intersect(&["users:online".to_string(), "users:premium".to_string()]).await?;
    println!("   ✓ Premium online users (intersection): {:?}", premium_online);

    let all_users = client
        .set()
        .union(
            &["users:online".to_string(), "users:premium".to_string(), "users:admin".to_string()]
        ).await?;
    println!("   ✓ All users (union): {:?}", all_users);

    let online_only = client
        .set()
        .difference(&["users:online".to_string(), "users:premium".to_string()]).await?;
    println!("   ✓ Online only (difference): {:?}", online_only);

    // Remove a member
    let removed = client.set().remove("users:online", "user:3").await?;
    println!("   ✓ Removed user:3 from online set ({} removed)", removed);

    // Cleanup
    println!("\n6. Cleanup:");
    println!("   Deleting test data...");

    let deleted = client.string().delete("user:1:name").await?;
    println!("   ✓ Deleted user:1:name: {}", deleted);

    let deleted = client.string().delete("user:1:session").await?;
    println!("   ✓ Deleted user:1:session: {}", deleted);

    let deleted = client.string().delete("user:2:name").await?;
    println!("   ✓ Deleted user:2:name: {}", deleted);

    let deleted = client.string().delete("user:2:email").await?;
    println!("   ✓ Deleted user:2:email: {}", deleted);

    let deleted = client.string().delete("user:2:age").await?;
    println!("   ✓ Deleted user:2:age: {}", deleted);

    println!("\n=== Example completed successfully! ===");

    Ok(())
}
