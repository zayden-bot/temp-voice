use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditChannel, GuildChannel, PermissionOverwrite, PermissionOverwriteType, Permissions,
    ResolvedOption, ResolvedValue, RoleId,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn privacy(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    everyone_role: RoleId,
    mut channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

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
            kind: PermissionOverwriteType::Role(everyone_role),
        }),
        "unlock" => perms.push(PermissionOverwrite {
            allow: Permissions::CONNECT,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(everyone_role),
        }),
        "invisible" => perms.push(PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL,
            kind: PermissionOverwriteType::Role(everyone_role),
        }),
        "visible" => perms.push(PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(everyone_role),
        }),
        _ => unreachable!("Invalid privacy option"),
    };

    channel
        .edit(ctx, EditChannel::new().permissions(perms))
        .await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Channel privacy updated.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
