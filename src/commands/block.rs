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

    {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get_mut(&channel.id)
            .expect("Expected channel in manager");
        channel_data.block(user.id);
    }

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
