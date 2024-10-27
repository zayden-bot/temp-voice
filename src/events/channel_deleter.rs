use serenity::all::{ChannelId, Context, DiscordJsonError, ErrorResponse, HttpError};
use sqlx::{Database, Pool};

use crate::{CachedState, Result, VoiceChannelManager, VoiceStateCache};

const CATEGORY_ID: ChannelId = ChannelId::new(923679215205892098);
const CREATOR_CHANNEL_ID: ChannelId = ChannelId::new(1289436847688253550);

pub async fn channel_deleter<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    pool: &Pool<Db>,
    old: Option<CachedState>,
) -> Result<()> {
    let old = match old {
        Some(old) => old,
        None => return Ok(()),
    };

    let channel_id = match old.channel_id {
        Some(channel_id) if channel_id != CREATOR_CHANNEL_ID => channel_id,
        _ => return Ok(()),
    };

    let row = match Manager::get(pool, channel_id).await {
        Ok(row) => row,
        Err(sqlx::Error::RowNotFound) => return Ok(()),
        Err(e) => return Err(e.into()),
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
        Err(e) => return Err(e.into()),
    };

    if channel
        .guild()
        .expect("Should be in a guild")
        .parent_id
        .expect("Should be in a category")
        != CATEGORY_ID
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
        row.delete::<Db, Manager>(pool).await?;

        match channel_id.delete(ctx).await {
            Ok(_) => {}
            Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
                error: DiscordJsonError { code: 10003, .. },
                ..
            }))) => {
                // Channel already deleted, ignore this error
            }
            Err(e) => return Err(e.into()),
        };
    }

    Ok(())
}
