use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:3000/ws";
    let url = Url::parse(url)?;

    println!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket handshake has been completed");

    let (mut write, mut read) = ws_stream.split();

    // Send a ping command
    let ping_message = json!({
        "id": "ping-1",
        "command": {
            "action": "ping"
        }
    });

    println!("Sending ping command: {}", ping_message);
    write.send(Message::Text(ping_message.to_string())).await?;

    // Send a set command
    let set_message = json!({
        "id": "set-1",
        "command": {
            "action": "set",
            "params": {
                "key": "test_key",
                "value": "test_value",
                "ttl": 3600
            }
        }
    });

    println!("Sending set command: {}", set_message);
    write.send(Message::Text(set_message.to_string())).await?;

    // Send a get command
    let get_message = json!({
        "id": "get-1",
        "command": {
            "action": "get",
            "params": {
                "key": "test_key"
            }
        }
    });

    println!("Sending get command: {}", get_message);
    write.send(Message::Text(get_message.to_string())).await?;

    // Send a batch set command
    let batch_set_message = json!({
        "id": "batch-set-1",
        "command": {
            "action": "batch_set",
            "params": {
                "key_values": {
                    "batch_key1": "value1",
                    "batch_key2": "value2",
                    "batch_key3": "value3"
                },
                "ttl": 1800
            }
        }
    });

    println!("Sending batch set command: {}", batch_set_message);
    write
        .send(Message::Text(batch_set_message.to_string()))
        .await?;

    // Send a batch get command
    let batch_get_message = json!({
        "id": "batch-get-1",
        "command": {
            "action": "batch_get",
            "params": {
                "keys": ["batch_key1", "batch_key2", "batch_key3"]
            }
        }
    });

    println!("Sending batch get command: {}", batch_get_message);
    write
        .send(Message::Text(batch_get_message.to_string()))
        .await?;

    // Listen for responses
    println!("Listening for responses...");
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text);

                // Parse the response to check if it's the last one
                if let Ok(response) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(id) = response["id"].as_str() {
                        if id == "batch-get-1" {
                            println!("Received all responses, closing connection");
                            break;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed by server");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
