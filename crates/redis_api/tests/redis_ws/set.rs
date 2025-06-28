use super::super::get_test_ws_base_url;
use tokio_tungstenite::connect_async;
use futures::{ SinkExt, StreamExt };
use serde_json::json;

#[tokio::test]
async fn test_ws_set_add_and_members() {
    let base_url = get_test_ws_base_url().await.replace("http", "ws");
    let url = format!("{}/redis_ws/set/ws", base_url);
    let (mut ws, _) = connect_async(url).await.expect("Failed to connect");
    let key = "ws_test_set_key";
    let member = "member1";

    // Add member to set
    let add_msg = json!({"type": "add", "data": {"key": key, "member": member}}).to_string();
    ws.send(tokio_tungstenite::tungstenite::Message::Text(add_msg)).await.unwrap();
    let _ = ws.next().await;

    // Get set members
    let members_msg = json!({"type": "members", "data": {"key": key}}).to_string();
    ws.send(tokio_tungstenite::tungstenite::Message::Text(members_msg)).await.unwrap();
    if let Some(Ok(tokio_tungstenite::tungstenite::Message::Text(resp))) = ws.next().await {
        let v: serde_json::Value = serde_json::from_str(&resp).unwrap();
        assert_eq!(v["type"], "result");
        assert!(
            v["data"]["value"]
                .as_array()
                .unwrap()
                .iter()
                .any(|m| m == member)
        );
    } else {
        panic!("No response from ws");
    }
}
