pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CommandNotInGuild,
    MemberNotInVoiceChannel,
    MissingPermissions,

    Serenity(serenity::Error),
}

impl From<serenity::Error> for Error {
    fn from(error: serenity::Error) -> Self {
        Self::Serenity(error)
    }
}
