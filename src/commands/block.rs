use serenity::all::{
    CommandInteraction, Context, EditInteractionResponse, GuildChannel, GuildId,
    PermissionOverwrite, PermissionOverwriteType, Permissions, ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::{Error, VoiceChannelManager};

pub async fn block(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    guild_id: GuildId,
    channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let user = match options.get("user") {
        Some(ResolvedValue::User(user, _member)) => user,
        _ => unreachable!("User option is required"),
    };

    let mut channel_data = VoiceChannelManager::take(ctx, channel.id).await?;
    channel_data.block(user.id);
    channel_data.save(ctx).await;

    channel
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::all(),
                kind: PermissionOverwriteType::Member(user.id),
            },
        )
        .await?;

    guild_id.disconnect_member(ctx, user.id).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Set user to blocked."),
        )
        .await?;

    Ok(())
}
