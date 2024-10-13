use std::time::Duration;

use serenity::all::EditInteractionResponse;
use serenity::all::{
    ChannelId, ChannelType, Context, CreateChannel, GuildId, PermissionOverwrite,
    PermissionOverwriteType, Permissions, ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::{Error, VoiceChannelManager};

use crate::get_voice_state;

const CATEGORY_ID: ChannelId = ChannelId::new(923679215205892098);

pub async fn create(
    ctx: &Context,
    interaction: &serenity::all::CommandInteraction,
    guild_id: GuildId,
    options: &Vec<ResolvedOption<'_>>,
) -> Result<(), Error> {
    let options = parse_options(options);

    let name = match options.get("name") {
        Some(ResolvedValue::String(name)) => name.to_string(),
        _ => format!("{}'s Channel", interaction.user.name),
    };

    let limit = match options.get("limit") {
        Some(ResolvedValue::Integer(limit)) => (*limit).clamp(0, 99) as u32,
        _ => 0,
    };

    let privacy = match options.get("privacy") {
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

    let vc_builder = CreateChannel::new(name)
        .kind(ChannelType::Voice)
        .category(CATEGORY_ID)
        .user_limit(limit)
        .permissions(perms);

    let vc = guild_id.create_channel(ctx, vc_builder).await?;
    VoiceChannelManager::register_voice_channel(ctx, vc.id, interaction.user.id).await;

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
