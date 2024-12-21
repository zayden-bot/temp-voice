use serenity::all::{
    ChannelType, Context, CreateChannel, DiscordJsonError, ErrorResponse, HttpError,
    PermissionOverwrite, PermissionOverwriteType, Permissions, VoiceState,
};
use sqlx::{Database, Pool};

use crate::{
    delete_voice_channel_if_inactive, Result, TempVoiceGuildManager, VoiceChannelData,
    VoiceChannelManager,
};

pub async fn channel_creator<
    Db: Database,
    GuildManager: TempVoiceGuildManager<Db>,
    ChannelManager: VoiceChannelManager<Db>,
>(
    ctx: &Context,
    pool: &Pool<Db>,
    new: &VoiceState,
) -> Result<()> {
    let guild_id = new
        .guild_id
        .expect("Should be in a guild as voice channels are guild only");

    let creator_channel = GuildManager::get_creator_channel(pool, guild_id).await?;

    let creator_channel_id = match new.channel_id {
        Some(channel) if channel == creator_channel => channel,
        _ => return Ok(()),
    };

    let creator_category = creator_channel_id
        .to_channel(ctx)
        .await?
        .guild()
        .expect("Should be in a guild")
        .parent_id
        .expect("Should be in a category");

    let member = new.member.as_ref().expect("Should be in a guild");

    let perms = vec![PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(member.user.id),
    }];

    let vc_builder = CreateChannel::new(format!("{}'s Channel", member.display_name()))
        .kind(ChannelType::Voice)
        .category(creator_category)
        .permissions(perms);

    let vc = guild_id.create_channel(ctx, vc_builder).await?;

    match guild_id.move_member(ctx, member.user.id, vc.id).await {
        Ok(_) => {}
        // Target user is not connected to voice.
        Err(serenity::Error::Http(HttpError::UnsuccessfulRequest(ErrorResponse {
            error: DiscordJsonError { code: 40032, .. },
            ..
        }))) => {
            if delete_voice_channel_if_inactive(ctx, guild_id, member.user.id, &vc).await? {
                return Ok(());
            }
        }
        Err(e) => {
            return Err(e.into());
        }
    };

    let row = VoiceChannelData::new(vc.id, new.user_id);
    row.save::<Db, ChannelManager>(pool).await?;

    Ok(())
}
