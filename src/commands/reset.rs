use serenity::all::EditInteractionResponse;
use serenity::all::{
    CommandInteraction, Context, EditChannel, GuildChannel, PermissionOverwriteType, RoleId,
};

use crate::error::PermissionError;
use crate::VoiceChannelManager;
use crate::{Error, Result};

pub async fn reset(
    ctx: &Context,
    interaction: &CommandInteraction,
    mut channel: GuildChannel,
    everyone_role: RoleId,
) -> Result<()> {
    let is_owner = VoiceChannelManager::verify_owner(ctx, channel.id, interaction.user.id).await?;

    if !is_owner {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    let mut channel_data = VoiceChannelManager::take(ctx, channel.id).await?;
    channel_data.reset();
    channel_data.save(ctx).await;

    let everyone_permissions = channel
        .permission_overwrites
        .iter()
        .find(|perm| perm.kind == PermissionOverwriteType::Role(everyone_role))
        .expect("Expected everyone role in channel permissions");

    channel
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
