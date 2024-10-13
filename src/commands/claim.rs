use serenity::all::{ChannelId, EditInteractionResponse};
use serenity::all::{
    CommandInteraction, Context, GuildChannel, PermissionOverwrite, PermissionOverwriteType,
    Permissions,
};

use crate::Error;
use crate::VoiceChannelManager;
use crate::VoiceStateCache;

pub async fn claim(
    ctx: &Context,
    interaction: &CommandInteraction,
    channel: GuildChannel,
) -> Result<(), Error> {
    if is_claimable(ctx, channel.id).await {
        return Err(Error::OwnerInChannel);
    }

    let mut channel_data = VoiceChannelManager::take(ctx, channel.id).await?;
    channel_data.owner = interaction.user.id;
    channel_data.save(ctx).await;

    channel
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

async fn is_claimable(ctx: &Context, channel_id: ChannelId) -> bool {
    let data = ctx.data.read().await;

    let owner = {
        let manager = data
            .get::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = manager
            .get(&channel_id)
            .expect("Expected channel in manager");
        channel_data.owner
    };

    let owner_state = {
        let cache = data
            .get::<VoiceStateCache>()
            .expect("Expected VoiceStateCache in TypeMap");

        cache.get(&owner)
    };

    owner_state.and_then(|state| state.channel_id) == Some(channel_id)
}
