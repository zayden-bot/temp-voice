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
    let is_owner = {
        let data = ctx.data.read().await;
        let manager = data
            .get::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get(&channel.id)
            .expect("Expected channel in manager");

        channel_data.owner == interaction.user.id
    };

    if !is_owner {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get_mut(&channel.id)
            .expect("Expected channel in manager");
        channel_data.reset()
    }

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
