pub mod commands;
mod error;
pub mod events;

use std::collections::HashMap;

use serenity::all::{ChannelId, Context, GuildId, LightMethod, Request, Route, UserId, VoiceState};
use serenity::prelude::TypeMapKey;

pub use commands::VoiceCommand;
pub use error::Error;
use error::Result;

pub struct VoiceStateCache;

impl TypeMapKey for VoiceStateCache {
    type Value = HashMap<UserId, Option<ChannelId>>;
}

pub async fn get_voice_state(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
) -> serenity::Result<VoiceState> {
    ctx.http
        .fire::<VoiceState>(Request::new(
            Route::GuildVoiceStates { guild_id, user_id },
            LightMethod::Get,
        ))
        .await
}
