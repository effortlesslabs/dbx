use super::super::get_test_ws_base_url;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::connect_async;

#[tokio::test]
async fn test_ws_hash_set_and_get() {
    let base_url = get_test_ws_base_url().await.replace("http", "ws");
    let url = format!("{}/redis_ws/hash/ws", base_url);
    let (mut ws, _) = connect_async(url).await.expect("Failed to connect");
    let key = "ws_test_hash_key";
    let field = "field1";
    let value = "value1";

    // Set hash field
    let set_msg =
        json!({"type": "set", "data": {"key": key, "field": field, "value": value}}).to_string();
    ws.send(tokio_tungstenite::tungstenite::Message::Text(set_msg))
        .await
        .unwrap();
    let _ = ws.next().await;

    // Get hash field
    let get_msg = json!({"type": "get", "data": {"key": key, "field": field}}).to_string();
    ws.send(tokio_tungstenite::tungstenite::Message::Text(get_msg))
        .await
        .unwrap();
    if let Some(Ok(tokio_tungstenite::tungstenite::Message::Text(resp))) = ws.next().await {
        let v: serde_json::Value = serde_json::from_str(&resp).unwrap();
        assert_eq!(v["type"], "result");
        assert_eq!(v["data"]["value"], value);
    } else {
        panic!("No response from ws");
    }
}
