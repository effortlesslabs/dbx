/// Batch commands enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchCommand {
    BatchGet,
    BatchSet,
    BatchDelete,
    BatchIncr,
    BatchIncrBy,
}

impl BatchCommand {
    pub fn as_str(&self) -> &'static str {
        match self {
            BatchCommand::BatchGet => "batch_get",
            BatchCommand::BatchSet => "batch_set",
            BatchCommand::BatchDelete => "batch_delete",
            BatchCommand::BatchIncr => "batch_incr",
            BatchCommand::BatchIncrBy => "batch_incrby",
        }
    }
}
