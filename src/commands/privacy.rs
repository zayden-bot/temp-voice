use std::collections::HashMap;

use futures::future;
use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, EditMember, GuildId,
    PermissionOverwrite, PermissionOverwriteType, Permissions, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::error::PermissionError;
use crate::voice_channel_manager::VoiceChannelMode;
use crate::{Error, VoiceChannelManager, VoiceChannelRow, VoiceStateCache};

pub async fn privacy<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    mut options: HashMap<&str, ResolvedValue<'_>>,
    guild_id: GuildId,
    channel_id: ChannelId,
    mut row: VoiceChannelRow,
) -> Result<(), Error> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if !row.is_trusted(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotTrusted));
    }

    let privacy = match options.remove("privacy") {
        Some(ResolvedValue::String(privacy)) => privacy,
        _ => "visible",
    };

    if privacy == "spectator" {
        row.mode = VoiceChannelMode::Spectator;
        row.save::<Db, Manager>(pool).await.unwrap();

        return Ok(());
    }

    let everyone_role = guild_id.everyone_role();

    let perm = match privacy {
        "open" => {
            {
                let data = ctx.data.read().await;
                let cache = data.get::<VoiceStateCache>().unwrap();

                let futures = cache
                    .values()
                    .filter(|s| s.channel_id == Some(channel_id))
                    .map(|s| async {
                        guild_id
                            .edit_member(ctx, s.user_id, EditMember::new().mute(false))
                            .await
                            .unwrap();
                    });

                future::join_all(futures).await;
            };

            row.mode = VoiceChannelMode::Open;
            row.save::<Db, Manager>(pool).await.unwrap();

            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(everyone_role),
            }
        }
        "lock" => PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::CONNECT,
            kind: PermissionOverwriteType::Role(everyone_role),
        },
        "invisible" => PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
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
