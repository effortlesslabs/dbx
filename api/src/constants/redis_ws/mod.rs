pub mod string;
pub mod batch;
pub mod set;
pub mod hash;
pub mod key;
pub mod admin;
pub mod utility;

pub use string::StringCommand;
pub use batch::BatchCommand;
pub use set::SetCommand;
pub use hash::HashCommand;
pub use key::KeyCommand;
pub use admin::AdminCommand;
pub use utility::UtilityCommand;

/// Get all supported commands as string slice
pub fn get_supported_commands() -> Vec<&'static str> {
    let mut commands = Vec::new();

    // String commands
    commands.extend_from_slice(
        &[
            StringCommand::Get.as_str(),
            StringCommand::Set.as_str(),
            StringCommand::Delete.as_str(),
            StringCommand::Exists.as_str(),
            StringCommand::Ttl.as_str(),
            StringCommand::Incr.as_str(),
            StringCommand::IncrBy.as_str(),
            StringCommand::SetNx.as_str(),
            StringCommand::Cas.as_str(),
        ]
    );

    // Batch commands
    commands.extend_from_slice(
        &[
            BatchCommand::BatchGet.as_str(),
            BatchCommand::BatchSet.as_str(),
            BatchCommand::BatchDelete.as_str(),
            BatchCommand::BatchIncr.as_str(),
            BatchCommand::BatchIncrBy.as_str(),
        ]
    );

    // Set commands
    commands.extend_from_slice(
        &[
            SetCommand::SAdd.as_str(),
            SetCommand::SRem.as_str(),
            SetCommand::SMembers.as_str(),
            SetCommand::SCard.as_str(),
            SetCommand::SIsMember.as_str(),
            SetCommand::SPop.as_str(),
        ]
    );

    // Hash commands
    commands.extend_from_slice(
        &[
            HashCommand::HSet.as_str(),
            HashCommand::HGet.as_str(),
            HashCommand::HDel.as_str(),
            HashCommand::HExists.as_str(),
            HashCommand::HLen.as_str(),
            HashCommand::HKeys.as_str(),
            HashCommand::HVals.as_str(),
            HashCommand::HGetAll.as_str(),
            HashCommand::HMSet.as_str(),
            HashCommand::HMGet.as_str(),
        ]
    );

    // Key commands
    commands.extend_from_slice(&[KeyCommand::Keys.as_str(), KeyCommand::Del.as_str()]);

    // Admin commands
    commands.extend_from_slice(
        &[
            AdminCommand::FlushAll.as_str(),
            AdminCommand::FlushDb.as_str(),
            AdminCommand::DbSize.as_str(),
            AdminCommand::Info.as_str(),
        ]
    );

    // Utility commands
    commands.extend_from_slice(
        &[
            UtilityCommand::ListKeys.as_str(),
            UtilityCommand::Ping.as_str(),
            UtilityCommand::Subscribe.as_str(),
            UtilityCommand::Unsubscribe.as_str(),
        ]
    );

    commands
}
