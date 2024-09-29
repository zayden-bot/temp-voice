pub mod commands;
mod error;
pub mod events;

use std::collections::HashMap;

use serenity::all::{
    ChannelId, Context, GuildId, LightMethod, Request, Route, User, UserId, VoiceState,
};
use serenity::prelude::TypeMapKey;

pub use commands::VoiceCommand;
pub use error::Error;
use error::Result;

pub struct State {
    pub channel_id: Option<ChannelId>,
    pub guild_id: Option<GuildId>,
    pub user: User,
}

impl State {
    async fn from_voice_state(ctx: &Context, state: VoiceState) -> Result<Self> {
        let user = if let Some(member) = state.member {
            member.user
        } else {
            ctx.http.get_user(state.user_id).await?
        };

        Ok(Self {
            channel_id: state.channel_id,
            guild_id: state.guild_id,
            user,
        })
    }
}

pub struct VoiceStateCache;

impl VoiceStateCache {
    pub async fn update(ctx: &Context, new: VoiceState) -> Result<Option<State>> {
        let new = new.clone();
        let mut data = ctx.data.write().await;
        let cache = data
            .get_mut::<Self>()
            .expect("Expected VoiceStateCache in TypeMap");

        let old = if new.channel_id.is_none() {
            cache.remove(&new.user_id)
        } else {
            cache.insert(new.user_id, State::from_voice_state(ctx, new).await?)
        };

        Ok(old)
    }
}

impl TypeMapKey for VoiceStateCache {
    type Value = HashMap<UserId, State>;
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
