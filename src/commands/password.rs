use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, PermissionOverwrite,
    PermissionOverwriteType, Permissions, RoleId,
};
use serenity::all::{EditInteractionResponse, ResolvedOption, ResolvedValue};
use zayden_core::parse_options;

use crate::error::PermissionError;
use crate::{Error, Result, TemporaryVoiceChannelManager};

pub async fn password<Manager: TemporaryVoiceChannelManager>(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
    everyone_role: RoleId,
) -> Result<()> {
    let is_owner = Manager::verify_owner(ctx, channel_id, interaction.user.id).await?;

    if !is_owner {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    let options = parse_options(options);

    let pass = match options.get("pass") {
        Some(ResolvedValue::String(pass)) => pass,
        _ => unreachable!("Password option is required"),
    };

    let mut channel_data = Manager::take(ctx, channel_id).await?;
    channel_data.password = Some(pass.to_string());
    channel_data.save(ctx).await;

    let perms = vec![
        PermissionOverwrite {
            allow: Permissions::all(),
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(interaction.user.id),
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::CONNECT,
            kind: PermissionOverwriteType::Role(everyone_role),
        },
    ];

    channel_id
        .edit(ctx, EditChannel::new().permissions(perms))
        .await?;

    interaction
        .edit_response(ctx, EditInteractionResponse::new().content("Password set."))
        .await?;

    Ok(())
}
