use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, GuildChannel, PermissionOverwrite,
    PermissionOverwriteType, Permissions, ResolvedOption, ResolvedValue, RoleId,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn privacy(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    everyone_role: RoleId,
    channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let privacy = match options.get("privacy") {
        Some(ResolvedValue::String(privacy)) => privacy,
        _ => "visible",
    };

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

    channel.create_permission(ctx, perm).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel privacy updated."),
        )
        .await?;

    Ok(())
}
