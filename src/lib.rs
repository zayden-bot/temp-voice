pub mod commands;
mod error;
pub mod events;
pub mod guild_manager;
pub mod voice_channel_manager;

use std::collections::HashMap;
use std::time::Duration;

use serenity::all::{
    ChannelId, Context, GuildChannel, GuildId, LightMethod, Request, Route, User, UserId,
    VoiceState,
};
use serenity::prelude::TypeMapKey;

pub use commands::VoiceCommand;
pub use error::Error;
use error::Result;
pub use guild_manager::{TempVoiceGuildManager, TempVoiceRow};
pub use voice_channel_manager::{VoiceChannelData, VoiceChannelManager};

#[derive(Debug)]
pub struct CachedState {
    pub channel_id: Option<ChannelId>,
    pub guild_id: GuildId,
    pub user: User,
}

impl CachedState {
    pub fn new(channel_id: Option<ChannelId>, guild_id: GuildId, user: User) -> Self {
        Self {
            channel_id,
            guild_id,
            user,
        }
    }

    async fn from_voice_state(ctx: &Context, state: VoiceState) -> Result<Self> {
        let user = if let Some(member) = state.member {
            member.user
        } else {
            state.user_id.to_user(ctx).await.unwrap()
        };

        Ok(Self {
            channel_id: state.channel_id,
            guild_id: state.guild_id.unwrap(),
            user,
        })
    }
}

pub struct VoiceStateCache;

impl VoiceStateCache {
    pub async fn new_with_states(
        guild_id: GuildId,
        states: &HashMap<UserId, VoiceState>,
    ) -> HashMap<UserId, CachedState> {
        let mut cache = HashMap::new();

        for (user_id, state) in states
            .iter()
            .filter(|(_, state)| state.channel_id.is_some())
        {
            cache.insert(
                *user_id,
                CachedState::new(
                    state.channel_id,
                    guild_id,
                    state.member.as_ref().unwrap().user.clone(),
                ),
            );
        }

        cache
    }

    pub async fn update(ctx: &Context, new: VoiceState) -> Result<Option<CachedState>> {
        let mut data = ctx.data.write().await;
        let cache = data
            .get_mut::<Self>()
            .expect("Expected VoiceStateCache in TypeMap");

        let old = if new.channel_id.is_none() {
            cache.remove(&new.user_id)
        } else {
            cache.insert(new.user_id, CachedState::from_voice_state(ctx, new).await?)
        };

        Ok(old)
    }
}

impl TypeMapKey for VoiceStateCache {
    type Value = HashMap<UserId, CachedState>;
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

pub async fn delete_voice_channel_if_inactive(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    vc: &GuildChannel,
) -> Result<bool> {
    tokio::time::sleep(Duration::from_secs(60)).await;

    match get_voice_state(ctx, guild_id, user_id).await {
        Ok(voice_state) if voice_state.channel_id == Some(vc.id) => Ok(false),
        _ => {
            vc.delete(ctx).await.unwrap();
            Ok(true)
        }
    }
}
