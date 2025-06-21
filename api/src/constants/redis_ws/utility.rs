/// Utility commands enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UtilityCommand {
    ListKeys,
    Ping,
    Subscribe,
    Unsubscribe,
}

impl UtilityCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            UtilityCommand::ListKeys => "list_keys",
            UtilityCommand::Ping => "ping",
            UtilityCommand::Subscribe => "subscribe",
            UtilityCommand::Unsubscribe => "unsubscribe",
        }
    }
}
