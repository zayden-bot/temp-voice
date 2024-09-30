use serenity::all::{
    CommandInteraction, Context, EditChannel, GuildChannel, PermissionOverwrite,
    PermissionOverwriteType, Permissions, RoleId,
};
use serenity::all::{EditInteractionResponse, ResolvedOption, ResolvedValue};
use zayden_core::parse_options;

use crate::error::PermissionError;
use crate::VoiceChannelManager;
use crate::{Error, Result};

pub async fn password(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    everyone_role: RoleId,
    mut channel: GuildChannel,
) -> Result<()> {
    let is_owner = {
        let data = ctx.data.read().await;
        let manager = data
            .get::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get(&channel.id)
            .expect("Expected channel in manager");

        channel_data.owner == interaction.user.id
    };

    if !is_owner {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    let options = parse_options(options);

    let pass = match options.get("pass") {
        Some(ResolvedValue::String(pass)) => pass,
        _ => unreachable!("Password option is required"),
    };

    {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get_mut(&channel.id)
            .expect("Expected channel in manager");
        channel_data.password = Some(pass.to_string());
    }

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

    channel
        .edit(ctx, EditChannel::new().permissions(perms))
        .await?;

    interaction
        .edit_response(ctx, EditInteractionResponse::new().content("Password set."))
        .await?;

    Ok(())
}
