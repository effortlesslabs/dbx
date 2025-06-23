use super::super::get_test_base_url;
use reqwest::Client;
use serde_json::json;
use std::time::{ SystemTime, UNIX_EPOCH };

#[tokio::test]
async fn test_set_add_and_members() {
    let base_url = get_test_base_url().await;
    let client = Client::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let key = format!("test_set_key_{}", timestamp);
    let member = "member1";

    // Add member to set
    let res = client
        .post(&format!("{}/redis/set/{}", base_url, key))
        .json(&json!({"member": member}))
        .send().await
        .unwrap();
    assert!(res.status().is_success());
    let added: usize = res.json().await.unwrap();
    assert!(added >= 1);

    // Get set members
    let res = client.get(&format!("{}/redis/set/{}/members", base_url, key)).send().await.unwrap();
    assert!(res.status().is_success());
    let members: Vec<String> = res.json().await.unwrap();
    assert!(members.contains(&member.to_string()));
}

#[tokio::test]
async fn test_set_remove() {
    let base_url = get_test_base_url().await;
    let client = Client::new();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let key = format!("test_set_key_del_{}", timestamp);
    let member = "member1";

    // Add member to set
    let _ = client
        .post(&format!("{}/redis/set/{}", base_url, key))
        .json(&json!({"member": member}))
        .send().await
        .unwrap();

    // Remove member from set
    let res = client
        .delete(&format!("{}/redis/set/{}/{}", base_url, key, member))
        .send().await
        .unwrap();
    assert!(res.status().is_success());
    let removed: usize = res.json().await.unwrap();
    assert!(removed >= 1);
}
