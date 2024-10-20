use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, PermissionOverwrite,
    PermissionOverwriteType, Permissions, ResolvedOption, ResolvedValue,
};
use serenity::all::{CreateMessage, Mentionable};
use zayden_core::parse_options;

use crate::{Error, TemporaryVoiceChannelManager};

pub async fn invite<Manager: TemporaryVoiceChannelManager>(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
) -> Result<(), Error> {
    let options = parse_options(options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _member)) => user,
        _ => unreachable!("User option is required"),
    };

    let mut channel_data = Manager::take(ctx, channel_id).await?;
    channel_data.create_invite(user.id);
    channel_data.save(ctx).await;

    channel_id
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
            CreateMessage::new().content(format!(
                "You have been invited to {}.",
                channel_id.mention()
            )),
        )
        .await;

    let content = match result {
        Ok(_) => "Sent invite to user.",
        Err(_) => "Invited user, but failed to send DM.",
    };

    interaction
        .edit_response(ctx, EditInteractionResponse::new().content(content))
        .await?;

    Ok(())
}
