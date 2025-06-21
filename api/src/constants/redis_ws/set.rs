/// Set commands enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetCommand {
    SAdd,
    SRem,
    SMembers,
    SCard,
    SIsMember,
    SPop,
}

impl SetCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            SetCommand::SAdd => "sadd",
            SetCommand::SRem => "srem",
            SetCommand::SMembers => "smembers",
            SetCommand::SCard => "scard",
            SetCommand::SIsMember => "sismember",
            SetCommand::SPop => "spop",
        }
    }
}
