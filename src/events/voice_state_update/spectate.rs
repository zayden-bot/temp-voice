use serenity::all::{ChannelId, Context, EditMember, VoiceState};
use sqlx::{Database, Pool};

use crate::{voice_channel_manager::VoiceChannelMode, CachedState, Result, VoiceChannelManager};

pub async fn spectate<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    new: &VoiceState,
    old: Option<CachedState>,
) -> Result<()> {
    if let Some(channel_id) = new.channel_id {
        on_join::<Db, Manager>(ctx, pool, new, channel_id).await;
        return Ok(());
    }

    if let Some(old) = old {
        if let Some(channel_id) = old.channel_id {
            on_leave::<Db, Manager>(ctx, pool, &old, channel_id).await;
            return Ok(());
        }
    }

    Ok(())
}

async fn on_join<Db: Database, ChannelManager: VoiceChannelManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    new: &VoiceState,
    channel_id: ChannelId,
) {
    let Some(data) = ChannelManager::get(pool, channel_id).await.unwrap() else {
        return;
    };

    if !matches!(data.mode, VoiceChannelMode::Spectator) {
        return;
    }

    let guild_id = new.guild_id.unwrap();
    guild_id
        .edit_member(ctx, new.user_id, EditMember::new().mute(true))
        .await
        .unwrap();
}

async fn on_leave<Db: Database, ChannelManager: VoiceChannelManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    old: &CachedState,
    channel_id: ChannelId,
) {
    let Some(data) = ChannelManager::get(pool, channel_id).await.unwrap() else {
        return;
    };

    if !matches!(data.mode, VoiceChannelMode::Spectator) {
        return;
    }

    old.guild_id
        .edit_member(ctx, old.user_id, EditMember::new().mute(false))
        .await
        .unwrap();
}
