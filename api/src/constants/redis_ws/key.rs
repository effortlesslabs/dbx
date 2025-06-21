/// Key commands enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCommand {
    Keys,
    Del,
}

impl KeyCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            KeyCommand::Keys => "keys",
            KeyCommand::Del => "del",
        }
    }
}
