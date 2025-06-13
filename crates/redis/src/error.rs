use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("Connection error: {0}")] Connection(String),
    #[error("Command error: {0}")] Command(String),
    #[error("Not connected to Redis")]
    NotConnected,
}

pub type RedisResult<T> = Result<T, RedisError>;
