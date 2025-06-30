use super::super::get_test_base_url;
use reqwest::Client;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_hash_set_and_get() {
    let base_url = get_test_base_url().await;
    let client = Client::new();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let key = format!("test_hash_key_{}", timestamp);
    let field = "field1";
    let value = "value1";

    // Set hash field
    let res = client
        .post(&format!("{}/redis/hash/{}/{}", base_url, key, field))
        .json(&json!({"value": value}))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    // Get hash field
    let res = client
        .get(&format!("{}/redis/hash/{}/{}", base_url, key, field))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let got: Option<String> = res.json().await.unwrap();
    assert_eq!(got, Some(value.to_string()));
}

#[tokio::test]
async fn test_hash_delete() {
    let base_url = get_test_base_url().await;
    let client = Client::new();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let key = format!("test_hash_key_del_{}", timestamp);
    let field = "field1";
    let value = "value1";

    // Set hash field
    let _ = client
        .post(&format!("{}/redis/hash/{}/{}", base_url, key, field))
        .json(&json!({"value": value}))
        .send()
        .await
        .unwrap();

    // Delete hash field
    let res = client
        .delete(&format!("{}/redis/hash/{}/{}", base_url, key, field))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    let deleted: bool = res.json().await.unwrap();
    assert!(deleted);
}
