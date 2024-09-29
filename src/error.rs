use zayden_core::ErrorResponse;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CommandNotInGuild,
    MemberNotInVoiceChannel,
    MissingPermissions,

    Serenity(serenity::Error),
}

impl ErrorResponse for Error {
    fn to_response(&self) -> String {
        match self {
            Error::CommandNotInGuild => String::from("This command can only be used in a guild."),
            Error::MemberNotInVoiceChannel => {
                String::from("You must be in a voice channel to use this command.")
            }
            Error::MissingPermissions => {
                String::from("Only the channel owner can use this command.")
            }
            _ => String::new(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<serenity::Error> for Error {
    fn from(error: serenity::Error) -> Self {
        Self::Serenity(error)
    }
}
