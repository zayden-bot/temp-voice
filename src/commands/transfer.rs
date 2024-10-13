use serenity::all::{
    CommandInteraction, Context, GuildChannel, PermissionOverwrite, PermissionOverwriteType,
    Permissions,
};
use serenity::all::{EditInteractionResponse, ResolvedOption, ResolvedValue};
use zayden_core::parse_options;

use crate::error::PermissionError;
use crate::VoiceChannelManager;
use crate::{Error, Result};

pub async fn transfer(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel: GuildChannel,
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

    let options = parse_options(options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => user,
        _ => unreachable!("User option is required"),
    };

    let mut channel_data = VoiceChannelManager::take(ctx, channel.id).await?;
    channel_data.owner = user.id;
    channel_data.save(ctx).await;

    channel
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user.id),
            },
        )
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Transferred channel."),
        )
        .await?;

    Ok(())
}
