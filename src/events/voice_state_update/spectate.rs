use serenity::all::{ChannelId, Context, EditMember, VoiceState};
use sqlx::{Database, Pool};

use crate::{voice_channel_manager::VoiceChannelMode, Result, VoiceChannelManager};

pub async fn spectate<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    new: &VoiceState,
) -> Result<()> {
    if let Some(channel_id) = new.channel_id {
        on_join::<Db, Manager>(ctx, pool, new, channel_id).await;
        return Ok(());
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

    let guild_id = new.guild_id.unwrap();

    let builder = match data.mode {
        VoiceChannelMode::Spectator => EditMember::new().mute(false),
        _ => EditMember::new().mute(true),
    };

    guild_id
        .edit_member(ctx, new.user_id, builder)
        .await
        .unwrap();
}
