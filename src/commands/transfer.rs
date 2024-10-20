use serenity::all::{
    ChannelId, CommandInteraction, Context, PermissionOverwrite, PermissionOverwriteType,
    Permissions,
};
use serenity::all::{EditInteractionResponse, ResolvedOption, ResolvedValue};
use zayden_core::parse_options;

use crate::error::PermissionError;
use crate::{Error, Result, TemporaryVoiceChannelManager};

pub async fn transfer<Manager: TemporaryVoiceChannelManager>(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
) -> Result<()> {
    let is_owner = Manager::verify_owner(ctx, channel_id, interaction.user.id).await?;

    if !is_owner {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    let options = parse_options(options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _)) => user,
        _ => unreachable!("User option is required"),
    };

    let mut channel_data = Manager::take(ctx, channel_id).await?;
    channel_data.owner = user.id;
    channel_data.save(ctx).await;

    channel_id
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
