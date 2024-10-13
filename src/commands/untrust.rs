use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, GuildChannel, PermissionOverwriteType,
    ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::{Error, VoiceChannelManager};

pub async fn untrust(
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

    let mut channel_data = VoiceChannelManager::take(ctx, channel.id).await?;
    channel_data.untrust(user.id);
    channel_data.save(ctx).await;

    channel
        .delete_permission(ctx, PermissionOverwriteType::Member(user.id))
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Removed user from trusted."),
        )
        .await?;

    Ok(())
}
