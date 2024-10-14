use serenity::all::{ChannelId, EditInteractionResponse, UserId};
use serenity::all::{
    CommandInteraction, Context, PermissionOverwrite, PermissionOverwriteType, Permissions,
};

use crate::voice_channel_manager::VoiceChannelData;
use crate::Error;
use crate::VoiceChannelManager;
use crate::VoiceStateCache;

pub async fn claim(
    ctx: &Context,
    interaction: &CommandInteraction,
    channel_id: ChannelId,
) -> Result<(), Error> {
    let channel_data = match VoiceChannelManager::take(ctx, channel_id).await {
        Ok(mut channel_data) => {
            if is_claimable(ctx, channel_data.owner, channel_id).await {
                return Err(Error::OwnerInChannel);
            }

            channel_data.owner = interaction.user.id;
            channel_data
        }
        Err(_) => VoiceChannelData::new(channel_id, interaction.user.id),
    };

    channel_data.save(ctx).await;

    channel_id
        .create_permission(
            ctx,
            PermissionOverwrite {
                allow: Permissions::all(),
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(interaction.user.id),
            },
        )
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Claimed channel."),
        )
        .await?;

    Ok(())
}

async fn is_claimable(ctx: &Context, owner: UserId, channel_id: ChannelId) -> bool {
    let data = ctx.data.read().await;

    let owner_state = {
        let cache = data
            .get::<VoiceStateCache>()
            .expect("Expected VoiceStateCache in TypeMap");

        cache.get(&owner)
    };

    owner_state.and_then(|state| state.channel_id) == Some(channel_id)
}
