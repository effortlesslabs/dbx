use redis::{ AsyncCommands, aio::ConnectionManager };
use tokio::sync::mpsc;
use crate::error::{ RedisError, RedisResult };
use crate::connection::RedisConnection;

/// Message received from a Redis Pub/Sub channel
#[derive(Debug, Clone)]
pub struct PubSubMessage {
    pub channel: String,
    pub payload: String,
}

/// Handles Redis Pub/Sub operations
pub struct RedisPubSub {
    connection: RedisConnection,
    subscriber: Option<redis::aio::PubSub>,
    message_tx: Option<mpsc::Sender<PubSubMessage>>,
}

impl RedisPubSub {
    /// Create a new Redis Pub/Sub handler
    pub fn new(connection: RedisConnection) -> Self {
        Self {
            connection,
            subscriber: None,
            message_tx: None,
        }
    }

    /// Subscribe to a channel
    pub async fn subscribe(&mut self, channel: &str) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        let mut pubsub = conn.into_pubsub();

        pubsub.subscribe(channel).await.map_err(|e| RedisError::PubSub(e.to_string()))?;

        self.subscriber = Some(pubsub);
        Ok(())
    }

    /// Subscribe to multiple channels
    pub async fn subscribe_many(&mut self, channels: &[&str]) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        let mut pubsub = conn.into_pubsub();

        for channel in channels {
            pubsub.subscribe(channel).await.map_err(|e| RedisError::PubSub(e.to_string()))?;
        }

        self.subscriber = Some(pubsub);
        Ok(())
    }

    /// Unsubscribe from a channel
    pub async fn unsubscribe(&mut self, channel: &str) -> RedisResult<()> {
        if let Some(ref mut pubsub) = self.subscriber {
            pubsub.unsubscribe(channel).await.map_err(|e| RedisError::PubSub(e.to_string()))?;
        }
        Ok(())
    }

    /// Unsubscribe from all channels
    pub async fn unsubscribe_all(&mut self) -> RedisResult<()> {
        if let Some(ref mut pubsub) = self.subscriber {
            pubsub.unsubscribe_all().await.map_err(|e| RedisError::PubSub(e.to_string()))?;
        }
        Ok(())
    }

    /// Publish a message to a channel
    pub async fn publish(&self, channel: &str, message: &str) -> RedisResult<()> {
        let mut conn = self.connection.get_connection().await?;
        conn.publish(channel, message).await.map_err(|e| RedisError::PubSub(e.to_string()))?;
        Ok(())
    }

    /// Start listening for messages
    pub async fn listen(&mut self) -> RedisResult<mpsc::Receiver<PubSubMessage>> {
        let (tx, rx) = mpsc::channel(100);
        self.message_tx = Some(tx);

        if let Some(ref mut pubsub) = self.subscriber {
            let mut msg_tx = self.message_tx.as_ref().unwrap().clone();

            tokio::spawn(async move {
                let mut stream = pubsub.on_message();
                while let Some(msg) = stream.next().await {
                    let channel = msg.get_channel_name().to_string();
                    let payload = msg.get_payload::<String>().unwrap_or_else(|_| "".to_string());

                    if let Err(_) = msg_tx.send(PubSubMessage { channel, payload }).await {
                        break;
                    }
                }
            });
        }

        Ok(rx)
    }

    /// Get the number of subscribers for a channel
    pub async fn numsub(&self, channel: &str) -> RedisResult<u64> {
        let mut conn = self.connection.get_connection().await?;
        let result: Vec<(String, u64)> = redis
            ::cmd("PUBSUB")
            .arg("NUMSUB")
            .arg(channel)
            .query_async(&mut conn).await
            .map_err(|e| RedisError::PubSub(e.to_string()))?;

        Ok(
            result
                .get(0)
                .map(|(_, count)| *count)
                .unwrap_or(0)
        )
    }

    /// Get all channels with active subscribers
    pub async fn channels(&self, pattern: Option<&str>) -> RedisResult<Vec<String>> {
        let mut conn = self.connection.get_connection().await?;
        let mut cmd = redis::cmd("PUBSUB").arg("CHANNELS");

        if let Some(pattern) = pattern {
            cmd.arg(pattern);
        }

        cmd.query_async(&mut conn).await.map_err(|e| RedisError::PubSub(e.to_string()))
    }
}
