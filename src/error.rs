use serenity::all::{ChannelId, Mentionable};
use zayden_core::ErrorResponse;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PermissionError {
    NotOwner,
    NotTrusted,
}

#[derive(Debug)]
pub enum Error {
    CommandNotInGuild,
    MemberNotInVoiceChannel,
    OwnerInChannel,
    InvalidPassword,
    PremiumRequired,
    UserIsOwner,
    MissingPermissions(PermissionError),
    ChannelNotFound(ChannelId),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> String {
        match self {
            Error::CommandNotInGuild => String::from("This command can only be used in a guild."),
            Error::MemberNotInVoiceChannel => {
                String::from("You must be in a voice channel or use the `channel` option to specify a channel to use this command.")
            }
            Error::OwnerInChannel => {
                String::from("Cannot use this command while the channel owner is present.")
            }
            Error::InvalidPassword => String::from("Invalid password."),
            Error::PremiumRequired => String::from("Only Server Boosters can use this command."),
            Error::UserIsOwner => String::from("You are already the owner of this channel."),
            Error::MissingPermissions(PermissionError::NotOwner) => {
                String::from("Only the channel owner can use this command.")
            }
            Error::MissingPermissions(PermissionError::NotTrusted) => {
                String::from("You must be trusted to use this command.")
            }
            Error::ChannelNotFound(id) => format!(
                "Channel not found: {}\nTry using `/voice claim` to claim the channel.",
                id.mention()
            ),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
