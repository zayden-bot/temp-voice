use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, GuildId, PermissionOverwrite, PermissionOverwriteType,
    Permissions,
};
use serenity::all::{EditInteractionResponse, ResolvedValue};

use crate::{Error, Result, VoiceChannelData};

pub async fn join(
    ctx: &Context,
    interaction: &CommandInteraction,
    mut options: HashMap<&str, ResolvedValue<'_>>,
    guild_id: GuildId,
    channel_id: ChannelId,
    row: &VoiceChannelData,
) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    let pass = match options.remove("pass") {
        Some(ResolvedValue::String(pass)) => pass,
        _ => unreachable!("Password option is required"),
    };

    if !row.verify_password(pass) {
        return Err(Error::InvalidPassword);
    }

    channel_id
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(interaction.user.id),
            },
        )
        .await
        .unwrap();

    guild_id
        .move_member(ctx, interaction.user.id, channel_id)
        .await
        .unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Successfully joined channel."),
        )
        .await
        .unwrap();

    Ok(())
}
