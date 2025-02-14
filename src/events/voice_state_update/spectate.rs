use serenity::all::{Context, EditMember, VoiceState};
use sqlx::{Database, Pool};

use crate::{
    voice_channel_manager::VoiceChannelMode, Result, TempVoiceGuildManager, VoiceChannelManager,
};

pub async fn spectate<
    Db: Database,
    GuildManager: TempVoiceGuildManager<Db>,
    ChannelManager: VoiceChannelManager<Db>,
>(
    ctx: &Context,
    pool: &Pool<Db>,
    new: &VoiceState,
) -> Result<()> {
    let Some(channel_id) = new.channel_id else {
        return Ok(());
    };

    let Some(data) = ChannelManager::get(pool, channel_id).await.unwrap() else {
        return Ok(());
    };

    if !matches!(data.mode, VoiceChannelMode::Spectator) {
        return Ok(());
    }

    let guild_id = new.guild_id.unwrap();
    guild_id
        .edit_member(ctx, new.user_id, EditMember::new().mute(true))
        .await
        .unwrap();

    Ok(())
}
