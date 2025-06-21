/// Admin commands enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminCommand {
    FlushAll,
    FlushDb,
    DbSize,
    Info,
}

impl AdminCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            AdminCommand::FlushAll => "flush_all",
            AdminCommand::FlushDb => "flush_db",
            AdminCommand::DbSize => "db_size",
            AdminCommand::Info => "info",
        }
    }
}
