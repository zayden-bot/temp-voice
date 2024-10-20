use serenity::all::{ChannelId, EditInteractionResponse, GuildId};
use serenity::all::{CommandInteraction, Context, EditChannel, PermissionOverwriteType};

use crate::error::PermissionError;
use crate::{Error, Result, TemporaryVoiceChannelManager};

pub async fn reset<Manager: TemporaryVoiceChannelManager>(
    ctx: &Context,
    interaction: &CommandInteraction,
    guild_id: GuildId,
    channel_id: ChannelId,
) -> Result<()> {
    let is_owner = Manager::verify_owner(ctx, channel_id, interaction.user.id).await?;

    if !is_owner {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    let mut channel_data = Manager::take(ctx, channel_id).await?;
    channel_data.reset();
    channel_data.save(ctx).await;

    let channel = guild_id
        .channels(ctx)
        .await?
        .remove(&channel_id)
        .ok_or(Error::ChannelNotFound(channel_id))?;

    let everyone_permissions = channel
        .permission_overwrites
        .iter()
        .find(|perm| perm.kind == PermissionOverwriteType::Role(guild_id.everyone_role()))
        .expect("Expected everyone role in channel permissions");

    channel_id
        .edit(
            ctx,
            EditChannel::new()
                .name(format!("{}'s Channel", interaction.user.display_name()))
                .user_limit(0)
                .permissions(vec![everyone_permissions.clone()]),
        )
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Reset channel."),
        )
        .await?;

    Ok(())
}
