use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };
use futures::{ SinkExt, StreamExt };
use serde_json::{ json, Value };
use url::Url;
use crate::get_test_server;

async fn connect_to_admin_ws() -> (
    futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message
    >,
    futures::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
) {
    let server = get_test_server().await;
    let ws_url = format!("ws://{}/redis_ws/admin/ws", server.addr);
    let (ws_stream, _) = connect_async(Url::parse(&ws_url).unwrap()).await.expect(
        "Failed to connect"
    );
    let (write, read) = ws_stream.split();
    (write, read)
}

async fn send_message_and_get_response(
    write: &mut futures::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message
    >,
    read: &mut futures::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    message: Value
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
async fn test_admin_ws_ping() {
    let (mut write, mut read) = connect_to_admin_ws().await;

    let message = json!({
        "type": "ping"
    });

    let response = send_message_and_get_response(&mut write, &mut read, message).await;
    assert_eq!(response["type"], "ping_result");
    assert_eq!(response["data"]["response"], "PONG");
}

// #[tokio::test]
// async fn test_admin_ws_info() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "info"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "info_result");
//     assert!(response["data"]["info"].as_str().unwrap().contains("redis_version"));
// }

// #[tokio::test]
// async fn test_admin_ws_dbsize() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "dbsize"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "dbsize_result");
//     assert!(response["data"]["size"].as_i64().unwrap() >= 0);
// }

// #[tokio::test]
// async fn test_admin_ws_health() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "health"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "health_result");
//     assert!(response["data"]["health"]["is_healthy"].as_bool().unwrap());
//     assert_eq!(response["data"]["health"]["ping_response"], "PONG");
// }

// #[tokio::test]
// async fn test_admin_ws_status() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "status"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "status_result");
//     assert!(response["data"]["status"]["uptime_seconds"].as_i64().unwrap() >= 0);
//     assert!(response["data"]["status"]["version"].as_str().is_some());
// }

// #[tokio::test]
// async fn test_admin_ws_memory_stats() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "memory_stats"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "memory_stats_result");
//     assert!(response["data"]["stats"]["used_memory"].as_str().is_some());
// }

// #[tokio::test]
// async fn test_admin_ws_client_stats() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "client_stats"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "client_stats_result");
//     assert!(response["data"]["stats"].as_object().is_some());
// }

// #[tokio::test]
// async fn test_admin_ws_server_stats() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "server_stats"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "server_stats_result");
//     assert!(response["data"]["stats"].as_object().is_some());
// }

// #[tokio::test]
// async fn test_admin_ws_config_get() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message =
//         json!({
//         "type": "config_get",
//         "data": {
//             "parameter": "maxmemory"
//         }
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "config_get_result");
//     assert_eq!(response["data"]["parameter"], "maxmemory");
//     assert!(response["data"]["value"].as_str().is_some());
// }

// #[tokio::test]
// async fn test_admin_ws_config_get_all() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "config_get_all"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     assert_eq!(response["type"], "config_get_all_result");
//     assert!(response["data"]["config"].as_object().is_some());
// }

// #[tokio::test]
// async fn test_admin_ws_config_set() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message =
//         json!({
//         "type": "config_set",
//         "data": {
//             "parameter": "timeout",
//             "value": "300"
//         }
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     // config_set may fail if parameter is read-only or invalid
//     assert!(response["type"] == "config_set_result" || response["type"] == "error");
// }

// #[tokio::test]
// async fn test_admin_ws_config_resetstat() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "config_resetstat"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     // config_resetstat may fail in some environments
//     assert!(response["type"] == "config_resetstat_result" || response["type"] == "error");
// }

// #[tokio::test]
// async fn test_admin_ws_config_rewrite() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "config_rewrite"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     // config_rewrite may fail in test environments due to file permissions
//     // Accept both success and error responses
//     assert!(response["type"] == "config_rewrite_result" || response["type"] == "error");
// }

// #[tokio::test]
// async fn test_admin_ws_flushdb() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "flushdb"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     // flushdb should succeed but may fail in some environments
//     assert!(response["type"] == "flushdb_result" || response["type"] == "error");
// }

// #[tokio::test]
// async fn test_admin_ws_flushall() {
//     let (mut write, mut read) = connect_to_admin_ws().await;

//     let message = json!({
//         "type": "flushall"
//     });

//     let response = send_message_and_get_response(&mut write, &mut read, message).await;
//     // flushall should succeed but may fail in some environments
//     assert!(response["type"] == "flushall_result" || response["type"] == "error");
// }

// #[tokio::test]
// async fn test_admin_ws_concurrent_operations() {
//     let mut handles = vec![];

//     for _i in 0..5 {
//         let handle = tokio::spawn(async move {
//             let (mut write, mut read) = connect_to_admin_ws().await;

//             // Test ping
//             let ping_message = json!({"type": "ping"});
//             let ping_response = send_message_and_get_response(
//                 &mut write,
//                 &mut read,
//                 ping_message
//             ).await;
//             assert_eq!(ping_response["type"], "ping_result");

//             // Test dbsize
//             let dbsize_message = json!({"type": "dbsize"});
//             let dbsize_response = send_message_and_get_response(
//                 &mut write,
//                 &mut read,
//                 dbsize_message
//             ).await;
//             assert_eq!(dbsize_response["type"], "dbsize_result");

//             // Test health
//             let health_message = json!({"type": "health"});
//             let health_response = send_message_and_get_response(
//                 &mut write,
//                 &mut read,
//                 health_message
//             ).await;
//             assert_eq!(health_response["type"], "health_result");
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }

// #[cfg(test)]
// mod tests {
//     // Empty for now - WebSocket tests will be implemented later
// }
