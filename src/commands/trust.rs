use serenity::all::{ChannelId, EditInteractionResponse};
use serenity::all::{
    CommandInteraction, Context, PermissionOverwrite, PermissionOverwriteType, Permissions,
    ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::voice_channel_manager::TemporaryVoiceChannelManager;
use crate::Error;

pub async fn trust<Manager: TemporaryVoiceChannelManager>(
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
    channel_data.trust(user.id);
    channel_data.save(ctx).await;

    channel_id
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL
                    | Permissions::MANAGE_CHANNELS
                    | Permissions::CONNECT
                    | Permissions::SET_VOICE_CHANNEL_STATUS,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user.id),
            },
        )
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Set user to trusted."),
        )
        .await?;

    Ok(())
}
