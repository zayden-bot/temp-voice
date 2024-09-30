use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, GuildChannel, PermissionOverwrite,
    PermissionOverwriteType, Permissions, ResolvedOption, ResolvedValue,
};
use serenity::all::{CreateMessage, Mentionable};
use zayden_core::parse_options;

use crate::Error;
use crate::VoiceChannelManager;

pub async fn invite(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _member)) => user,
        _ => unreachable!("User option is required"),
    };

    {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get_mut(&channel.id)
            .expect("Expected channel in manager");
        channel_data.create_invite(user.id);
    }

    channel
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL | Permissions::CONNECT,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user.id),
            },
        )
        .await?;

    let result = user
        .direct_message(
            ctx,
            CreateMessage::new()
                .content(format!("You have been invited to {}.", channel.mention())),
        )
        .await;

    let content = match result {
        Ok(_) => "Sent invite to user.",
        Err(_) => "Failed to direct message user.",
    };

    interaction
        .edit_response(ctx, EditInteractionResponse::new().content(content))
        .await?;

    Ok(())
}
