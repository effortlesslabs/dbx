/// Hash commands enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashCommand {
    HSet,
    HGet,
    HDel,
    HExists,
    HLen,
    HKeys,
    HVals,
    HGetAll,
    HMSet,
    HMGet,
}

impl HashCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            HashCommand::HSet => "hset",
            HashCommand::HGet => "hget",
            HashCommand::HDel => "hdel",
            HashCommand::HExists => "hexists",
            HashCommand::HLen => "hlen",
            HashCommand::HKeys => "hkeys",
            HashCommand::HVals => "hvals",
            HashCommand::HGetAll => "hgetall",
            HashCommand::HMSet => "hmset",
            HashCommand::HMGet => "hmget",
        }
    }
}
