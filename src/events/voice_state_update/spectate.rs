use serenity::all::{
    ChannelId, Context, DiscordJsonError, EditMember, ErrorResponse, HttpError, VoiceState,
};
use sqlx::{Database, Pool};

use crate::{voice_channel_manager::VoiceChannelMode, CachedState, Result, VoiceChannelManager};

pub async fn spectate<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    new: &VoiceState,
    old: Option<&CachedState>,
) -> Result<()> {
    if let Some(old) = old {
        if old.channel_id == new.channel_id {
            return Ok(());
        }
    }

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
    let guild_id = new.guild_id.unwrap();

    let builder = match ChannelManager::get(pool, channel_id).await.unwrap() {
        Some(row) => {
            if row.is_trusted(new.user_id) {
                EditMember::new().mute(false)
            } else {
                match row.mode {
                    VoiceChannelMode::Spectator => EditMember::new().mute(true),
                    _ => EditMember::new().mute(false),
                }
            }
        }
        None => EditMember::new().mute(false),
    };

    match guild_id.edit_member(ctx, new.user_id, builder).await {
        // User is not connected to voice channel
        Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 40032, .. },
            ..
        }))) => {}
        e => {
            e.unwrap();
        }
    };
}
