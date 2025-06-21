/// String commands enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringCommand {
    Get,
    Set,
    Delete,
    Exists,
    Ttl,
    Incr,
    IncrBy,
    SetNx,
    Cas,
}

impl StringCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            StringCommand::Get => "get",
            StringCommand::Set => "set",
            StringCommand::Delete => "delete",
            StringCommand::Exists => "exists",
            StringCommand::Ttl => "ttl",
            StringCommand::Incr => "incr",
            StringCommand::IncrBy => "incrby",
            StringCommand::SetNx => "setnx",
            StringCommand::Cas => "cas",
        }
    }
}
