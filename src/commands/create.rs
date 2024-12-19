use std::collections::HashMap;
use std::time::Duration;

use serenity::all::EditInteractionResponse;
use serenity::all::{
    ChannelType, Context, CreateChannel, GuildId, PermissionOverwrite, PermissionOverwriteType,
    Permissions, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{Error, TempVoiceGuildManager, VoiceChannelData, VoiceChannelManager};

use crate::get_voice_state;

pub async fn create<
    Db: Database,
    GuildManager: TempVoiceGuildManager<Db>,
    ChannelManager: VoiceChannelManager<Db>,
>(
    ctx: &Context,
    interaction: &serenity::all::CommandInteraction,
    pool: &Pool<Db>,
    guild_id: GuildId,
    mut options: HashMap<&str, &ResolvedValue<'_>>,
) -> Result<(), Error> {
    let name = match options.remove("name") {
        Some(ResolvedValue::String(name)) => name.to_string(),
        _ => format!("{}'s Channel", interaction.user.name),
    };

    let limit = match options.remove("limit") {
        Some(ResolvedValue::Integer(limit)) => (*limit).clamp(0, 99) as u32,
        _ => 0,
    };

    let privacy = match options.remove("privacy") {
        Some(ResolvedValue::String(privacy)) => privacy,
        _ => "visible",
    };

    let mut perms = vec![PermissionOverwrite {
        allow: Permissions::all(),
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(interaction.user.id),
    }];

    match privacy {
        "lock" => perms.push(PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::CONNECT,
            kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
        }),
        "unlock" => perms.push(PermissionOverwrite {
            allow: Permissions::CONNECT,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
        }),
        "invisible" => perms.push(PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
        }),
        "visible" => perms.push(PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
        }),
        _ => unreachable!("Invalid privacy option"),
    };

    let category = GuildManager::get_category(pool, guild_id).await?;

    let vc_builder = CreateChannel::new(name)
        .kind(ChannelType::Voice)
        .category(category)
        .user_limit(limit)
        .permissions(perms);

    let vc = guild_id.create_channel(ctx, vc_builder).await?;

    let row = VoiceChannelData::new(vc.id, interaction.user.id);
    row.save::<Db, ChannelManager>(pool).await?;

    let move_result = guild_id.move_member(ctx, interaction.user.id, vc.id).await;

    let response_content = match move_result {
        Ok(_) => "Voice channel created and you have been moved successfully.",
        Err(_) => "Voice channel created. You have 1 minute to join.",
    };

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content(response_content),
        )
        .await?;

    if move_result.is_err() {
        tokio::time::sleep(Duration::from_secs(60)).await;

        let voice_state_result = get_voice_state(ctx, guild_id, interaction.user.id).await;

        if voice_state_result.is_err() || voice_state_result.unwrap().channel_id != Some(vc.id) {
            vc.delete(ctx).await?;
        }
    }

    Ok(())
}
