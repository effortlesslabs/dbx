use redis_rs::{ DbxClient, StringOperation };

#[tokio::test]
async fn test_sdk_integration() {
    // This test demonstrates the SDK API without requiring a real server
    // In a real scenario, you would have a test server running

    // Test client creation
    let client = DbxClient::new("http://localhost:8080").unwrap();

    // Test string client creation
    let string_client = client.string();
    assert_eq!(string_client.base_url().as_str(), "http://localhost:8080/");

    // Test set client creation
    let set_client = client.set();
    assert_eq!(set_client.base_url().as_str(), "http://localhost:8080/");

    // Test that we can create StringOperation structs
    let operation = StringOperation {
        key: "test_key".to_string(),
        value: Some("test_value".to_string()),
        ttl: Some(3600),
    };

    assert_eq!(operation.key, "test_key");
    assert_eq!(operation.value, Some("test_value".to_string()));
    assert_eq!(operation.ttl, Some(3600));

    // Test that we can create batch operations
    let operations = vec![
        StringOperation {
            key: "key1".to_string(),
            value: Some("value1".to_string()),
            ttl: None,
        },
        StringOperation {
            key: "key2".to_string(),
            value: Some("value2".to_string()),
            ttl: Some(7200),
        }
    ];

    assert_eq!(operations.len(), 2);
    assert_eq!(operations[0].key, "key1");
    assert_eq!(operations[1].key, "key2");
}

#[tokio::test]
async fn test_error_handling() {
    // Test that we can create clients with invalid URLs and get proper errors
    let result = DbxClient::new("invalid-url");
    assert!(result.is_err());

    // Test that we can create clients with valid URLs
    let result = DbxClient::new("http://localhost:8080");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_client_cloning() {
    let client1 = DbxClient::new("http://localhost:8080").unwrap();
    let client2 = client1.clone();

    // Both clients should have the same base URL
    assert_eq!(client1.base_url(), client2.base_url());

    // Both clients should have access to string and set operations
    let string_client1 = client1.string();
    let string_client2 = client2.string();
    assert_eq!(string_client1.base_url(), string_client2.base_url());

    let set_client1 = client1.set();
    let set_client2 = client2.set();
    assert_eq!(set_client1.base_url(), set_client2.base_url());
}
