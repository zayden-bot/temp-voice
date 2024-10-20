use serenity::all::{
    CommandInteraction, Context, GuildId, PermissionOverwrite, PermissionOverwriteType, Permissions,
};
use serenity::all::{EditInteractionResponse, ResolvedOption, ResolvedValue};
use zayden_core::parse_options;

use crate::{Error, Result, TemporaryVoiceChannelManager};

pub async fn join<Manager: TemporaryVoiceChannelManager>(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    guild_id: GuildId,
) -> Result<()> {
    let options = parse_options(options);

    let channel = match options.get("channel") {
        Some(ResolvedValue::Channel(channel)) => *channel,
        _ => unreachable!("Channel option is required"),
    };

    let pass = match options.get("pass") {
        Some(ResolvedValue::String(pass)) => *pass,
        _ => unreachable!("Password option is required"),
    };

    let is_valid = Manager::verify_password(ctx, channel.id, pass).await?;

    if !is_valid {
        return Err(Error::InvalidPassword);
    }

    channel
        .id
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(interaction.user.id),
            },
        )
        .await?;

    guild_id
        .move_member(ctx, interaction.user.id, channel.id)
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Successfully joined channel."),
        )
        .await?;

    Ok(())
}
