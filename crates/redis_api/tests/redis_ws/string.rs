use crate::get_test_server;
use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

async fn connect_to_string_ws() -> (
    futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    futures::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
) {
    let server = get_test_server().await;
    let ws_url = format!("ws://{}/redis_ws/string/ws", server.addr);
    let (ws_stream, _) = connect_async(Url::parse(&ws_url).unwrap())
        .await
        .expect("Failed to connect");
    let (write, read) = ws_stream.split();
    (write, read)
}

async fn send_message_and_get_response(
    write: &mut futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    read: &mut futures::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
    message: Value,
) -> Value {
    let message_str = serde_json::to_string(&message).unwrap();
    write.send(Message::Text(message_str)).await.unwrap();

    if let Some(Ok(Message::Text(response))) = read.next().await {
        serde_json::from_str(&response).unwrap()
    } else {
        panic!("Expected text message response");
    }
}

#[tokio::test]
async fn test_redis_ws_set_get_string_basic() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let test_key = "ws_test_basic";
    let test_value = "basic_value";

    // Set string
    let set_message = json!({
        "type": "set",
        "data": {
            "key": test_key,
            "value": test_value
        }
    });

    let set_response = send_message_and_get_response(&mut write, &mut read, set_message).await;
    assert_eq!(set_response["type"], "result");
    assert_eq!(set_response["data"]["key"], test_key);
    assert_eq!(set_response["data"]["value"], json!(test_value));

    // Get string
    let get_message = json!({
        "type": "get",
        "data": {
            "key": test_key
        }
    });

    let get_response = send_message_and_get_response(&mut write, &mut read, get_message).await;
    assert_eq!(get_response["type"], "result");
    assert_eq!(get_response["data"]["key"], test_key);
    assert_eq!(get_response["data"]["value"], json!(test_value));
}

#[tokio::test]
async fn test_redis_ws_set_get_string_with_special_chars() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let test_key = "ws_test_special";
    let test_value = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

    // Set string with special characters
    let set_message = json!({
        "type": "set",
        "data": {
            "key": test_key,
            "value": test_value
        }
    });

    let set_response = send_message_and_get_response(&mut write, &mut read, set_message).await;
    assert_eq!(set_response["type"], "result");

    // Get string
    let get_message = json!({
        "type": "get",
        "data": {
            "key": test_key
        }
    });

    let get_response = send_message_and_get_response(&mut write, &mut read, get_message).await;
    assert_eq!(get_response["type"], "result");
    assert_eq!(get_response["data"]["value"], json!(test_value));
}

#[tokio::test]
async fn test_redis_ws_get_nonexistent_string() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let nonexistent_key = "ws_test_nonexistent";

    // Try to get nonexistent string
    let get_message = json!({
        "type": "get",
        "data": {
            "key": nonexistent_key
        }
    });

    let get_response = send_message_and_get_response(&mut write, &mut read, get_message).await;
    assert_eq!(get_response["type"], "result");
    assert_eq!(get_response["data"]["key"], nonexistent_key);
    assert_eq!(get_response["data"]["value"], Value::Null);
}

#[tokio::test]
async fn test_redis_ws_delete_string() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let test_key = "ws_test_delete";
    let test_value = "delete_value";

    // Set string first
    let set_message = json!({
        "type": "set",
        "data": {
            "key": test_key,
            "value": test_value
        }
    });

    send_message_and_get_response(&mut write, &mut read, set_message).await;

    // Verify it exists
    let get_message = json!({
        "type": "get",
        "data": {
            "key": test_key
        }
    });

    let get_response = send_message_and_get_response(&mut write, &mut read, get_message).await;
    assert_eq!(get_response["data"]["value"], json!(test_value));

    // Delete string
    let del_message = json!({
        "type": "del",
        "data": {
            "key": test_key
        }
    });

    let del_response = send_message_and_get_response(&mut write, &mut read, del_message).await;
    assert_eq!(del_response["type"], "deleted");
    assert_eq!(del_response["data"]["key"], test_key);
    assert_eq!(del_response["data"]["deleted"], true);

    // Verify it's gone
    let get_message2 = json!({
        "type": "get",
        "data": {
            "key": test_key
        }
    });

    let get_response2 = send_message_and_get_response(&mut write, &mut read, get_message2).await;
    assert_eq!(get_response2["data"]["value"], Value::Null);
}

#[tokio::test]
async fn test_redis_ws_delete_nonexistent_string() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let nonexistent_key = "ws_test_delete_nonexistent";

    // Try to delete nonexistent string
    let del_message = json!({
        "type": "del",
        "data": {
            "key": nonexistent_key
        }
    });

    let del_response = send_message_and_get_response(&mut write, &mut read, del_message).await;
    assert_eq!(del_response["type"], "deleted");
    assert_eq!(del_response["data"]["key"], nonexistent_key);
    assert_eq!(del_response["data"]["deleted"], false);
}

#[tokio::test]
async fn test_redis_ws_string_overwrite() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let test_key = "ws_test_overwrite";
    let value1 = "value1";
    let value2 = "value2";

    // Set initial value
    let set_message1 = json!({
        "type": "set",
        "data": {
            "key": test_key,
            "value": value1
        }
    });

    send_message_and_get_response(&mut write, &mut read, set_message1).await;

    // Verify initial value
    let get_message = json!({
        "type": "get",
        "data": {
            "key": test_key
        }
    });

    let get_response = send_message_and_get_response(&mut write, &mut read, get_message).await;
    assert_eq!(get_response["data"]["value"], json!(value1));

    // Overwrite with new value
    let set_message2 = json!({
        "type": "set",
        "data": {
            "key": test_key,
            "value": value2
        }
    });

    send_message_and_get_response(&mut write, &mut read, set_message2).await;

    // Verify new value
    let get_message2 = json!({
        "type": "get",
        "data": {
            "key": test_key
        }
    });

    let get_response2 = send_message_and_get_response(&mut write, &mut read, get_message2).await;
    assert_eq!(get_response2["data"]["value"], json!(value2));
}

#[tokio::test]
async fn test_redis_ws_string_info() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let test_key = "ws_test_info";
    let test_value = "info_value";

    // Set string first
    let set_message = json!({
        "type": "set",
        "data": {
            "key": test_key,
            "value": test_value
        }
    });

    send_message_and_get_response(&mut write, &mut read, set_message).await;

    // Get string info
    let info_message = json!({
        "type": "info",
        "data": {
            "key": test_key
        }
    });

    let info_response = send_message_and_get_response(&mut write, &mut read, info_message).await;
    assert_eq!(info_response["type"], "info_result");
    assert!(info_response["data"]["info"].is_object());
}

#[tokio::test]
async fn test_redis_ws_batch_string_operations() {
    let (mut write, mut read) = connect_to_string_ws().await;
    let operations = vec![
        ("batch_key_1", "batch_value_1"),
        ("batch_key_2", "batch_value_2"),
        ("batch_key_3", "batch_value_3"),
    ];

    // Batch set strings
    let batch_ops: Vec<Value> = operations
        .iter()
        .map(|(key, value)| json!({"key": key, "value": value}))
        .collect();

    let batch_set_message = json!({
        "type": "batch_set",
        "data": {
            "operations": batch_ops
        }
    });

    let batch_set_response =
        send_message_and_get_response(&mut write, &mut read, batch_set_message).await;
    assert_eq!(batch_set_response["type"], "result");

    // Batch get strings
    let keys: Vec<String> = operations.iter().map(|(key, _)| key.to_string()).collect();

    let batch_get_message = json!({
        "type": "batch_get",
        "data": {
            "keys": keys
        }
    });

    let batch_get_response =
        send_message_and_get_response(&mut write, &mut read, batch_get_message).await;
    assert_eq!(batch_get_response["type"], "batch_result");
    assert_eq!(
        batch_get_response["data"]["keys"].as_array().unwrap().len(),
        3
    );
    assert_eq!(
        batch_get_response["data"]["values"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
}

#[tokio::test]
async fn test_redis_ws_concurrent_string_operations() {
    let mut handles = vec![];

    for i in 0..5 {
        let handle = tokio::spawn(async move {
            let (mut write, mut read) = connect_to_string_ws().await;
            let test_key = format!("ws_concurrent_{}", i);
            let test_value = format!("concurrent_value_{}", i);

            // Set string
            let set_message = json!({
                "type": "set",
                "data": {
                    "key": test_key,
                    "value": test_value
                }
            });

            let set_response =
                send_message_and_get_response(&mut write, &mut read, set_message).await;
            assert_eq!(set_response["type"], "result");

            // Get string
            let get_message = json!({
                "type": "get",
                "data": {
                    "key": test_key
                }
            });

            let get_response =
                send_message_and_get_response(&mut write, &mut read, get_message).await;
            assert_eq!(get_response["type"], "result");
            assert_eq!(get_response["data"]["value"], json!(test_value));

            // Delete string
            let del_message = json!({
                "type": "del",
                "data": {
                    "key": test_key
                }
            });

            let del_response =
                send_message_and_get_response(&mut write, &mut read, del_message).await;
            assert_eq!(del_response["type"], "deleted");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_redis_ws_ping_pong() {
    let (mut write, mut read) = connect_to_string_ws().await;

    let ping_message = json!({
        "type": "ping"
    });

    let response = send_message_and_get_response(&mut write, &mut read, ping_message).await;
    assert_eq!(response["type"], "pong");
}

#[cfg(test)]
mod tests {
    // Empty for now - WebSocket tests will be implemented later
}
