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
    MissingGuildId,
    MemberNotInVoiceChannel,
    OwnerInChannel,
    InvalidPassword,
    PremiumRequired,
    UserIsOwner,
    MissingPermissions(PermissionError),
    ChannelNotFound(String),
}

impl Error {
    pub fn channel_not_found(id: ChannelId) -> Self {
        let response = format!(
            "Channel not found: {}\nTry using `/voice claim` to claim the channel.",
            id.mention()
        );

        Self::ChannelNotFound(response)
    }
}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Error::MissingGuildId => zayden_core::Error::MissingGuildId.to_response(),
            Error::MemberNotInVoiceChannel => {
                "You must be in a voice channel or use the `channel` option to specify a channel to use this command."
            }
            Error::OwnerInChannel => {
                "Cannot use this command while the channel owner is present."
            }
            Error::InvalidPassword => "Invalid password.",
            Error::PremiumRequired => "Only Server Boosters can use this command.",
            Error::UserIsOwner => "You are already the owner of this channel.",
            Error::MissingPermissions(PermissionError::NotOwner) => {
                "Only the channel owner can use this command."
            }
            Error::MissingPermissions(PermissionError::NotTrusted) => {
                "You must be trusted to use this command."
            }
            Error::ChannelNotFound(msg) => msg,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
