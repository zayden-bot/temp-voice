use serenity::all::{
    ChannelId, ChannelType, Context, CreateChannel, PermissionOverwrite, PermissionOverwriteType,
    Permissions, VoiceState,
};

use crate::{Result, TemporaryVoiceChannelManager};

const CHANNEL_ID: ChannelId = ChannelId::new(1289436847688253550);

pub async fn channel_creator<Manager: TemporaryVoiceChannelManager>(
    ctx: &Context,
    new: &VoiceState,
) -> Result<()> {
    let creator_channel_id = match new.channel_id {
        Some(channel) if channel == CHANNEL_ID => channel,
        _ => return Ok(()),
    };

    let guild_id = new
        .guild_id
        .expect("Should be in a guild as voice channels are guild only");

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
    Manager::register_voice_channel(ctx, vc.id, new.user_id).await;

    guild_id.move_member(ctx, member.user.id, vc.id).await?;

    Ok(())
}
