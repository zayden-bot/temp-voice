use std::collections::HashMap;

use serenity::all::{CommandInteraction, Context, EditInteractionResponse, GuildId, ResolvedValue};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelData};

pub async fn kick(
    ctx: &Context,
    interaction: &CommandInteraction,
    mut options: HashMap<&str, &ResolvedValue<'_>>,
    guild_id: GuildId,
    row: &VoiceChannelData,
) -> Result<(), Error> {
    interaction.defer_ephemeral(ctx).await?;

    if !row.is_trusted(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotTrusted));
    }

    let user = match options.remove("member") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("Member option is required"),
    };

    guild_id.disconnect_member(ctx, user).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("User kicked from channel."),
        )
        .await?;

    Ok(())
}
