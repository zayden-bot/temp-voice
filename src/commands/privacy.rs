use std::collections::HashMap;

use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, GuildId, PermissionOverwrite,
    PermissionOverwriteType, Permissions, ResolvedValue,
};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelData};

pub async fn privacy(
    ctx: &Context,
    interaction: &CommandInteraction,
    mut options: HashMap<&str, ResolvedValue<'_>>,
    guild_id: GuildId,
    channel_id: ChannelId,
    row: &VoiceChannelData,
) -> Result<(), Error> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !row.is_trusted(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotTrusted));
    }

    let privacy = match options.remove("privacy") {
        Some(ResolvedValue::String(privacy)) => privacy,
        _ => "visible",
    };

    match privacy {
        "spectator" => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new()
                        .content("Spectator mode is not yet implemented."),
                )
                .await
                .unwrap();
        }
        "open-mic" => {
            interaction
                .edit_response(
                    ctx,
                    EditInteractionResponse::new().content("Open mic mode is not yet implemented."),
                )
                .await
                .unwrap();
        }
        _ => {}
    }

    let everyone_role = guild_id.everyone_role();

    let perm = match privacy {
        "lock" => PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::CONNECT,
            kind: PermissionOverwriteType::Role(everyone_role),
        },
        "unlock" => PermissionOverwrite {
            allow: Permissions::CONNECT,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(everyone_role),
        },
        "invisible" => PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(everyone_role),
        },
        "visible" => PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(everyone_role),
        },
        _ => unreachable!("Invalid privacy option"),
    };

    channel_id.create_permission(ctx, perm).await.unwrap();

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel privacy updated."),
        )
        .await
        .unwrap();

    Ok(())
}
