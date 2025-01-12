use serenity::all::{Context, DiscordJsonError, ErrorResponse, HttpError};
use sqlx::{Database, Pool};

use crate::{CachedState, Result, TempVoiceGuildManager, VoiceChannelManager, VoiceStateCache};

pub async fn channel_deleter<
    Db: Database,
    GuildManager: TempVoiceGuildManager<Db>,
    ChannelManager: VoiceChannelManager<Db>,
>(
    ctx: &Context,
    pool: &Pool<Db>,
    old: Option<CachedState>,
) -> Result<()> {
    let old = match old {
        Some(old) => old,
        None => return Ok(()),
    };

    let guild_id = old
        .guild_id
        .expect("Should be in a guild as voice channels are guild only");

    let guild_data = GuildManager::get(pool, guild_id).await.unwrap();

    let channel_id = match old.channel_id {
        Some(channel_id) if channel_id != guild_data.creator_channel() => channel_id,
        _ => return Ok(()),
    };

    let row = match ChannelManager::get(pool, channel_id).await.unwrap() {
        Some(row) => row,
        None => return Ok(()),
    };

    if row.is_persistent() {
        return Ok(());
    }

    let channel = match channel_id.to_channel(ctx).await {
        Ok(channel) => channel,
        Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 10003, .. },
            ..
        }))) => {
            // Channel already deleted, ignore this error
            return Ok(());
        }
        e => e.unwrap(),
    };

    let category = guild_data.category();

    if channel
        .guild()
        .expect("Should be in a guild")
        .parent_id
        .expect("Should be in a category")
        != category
    {
        return Ok(());
    }

    let users = {
        let data = ctx.data.read().await;
        let cache = data
            .get::<VoiceStateCache>()
            .expect("Expected VoiceStateCache in TypeMap");

        cache
            .values()
            .filter(|id| id.channel_id == Some(channel_id))
            .count()
    };

    if users == 0 {
        row.delete::<Db, ChannelManager>(pool).await?;

        match channel_id.delete(ctx).await {
            Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10003, .. },
                ..
            }))) => {
                // Channel already deleted, ignore this error
            }
            result => {
                result.unwrap();
            }
        };
    }

    Ok(())
}
