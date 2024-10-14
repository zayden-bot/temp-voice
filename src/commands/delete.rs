use serenity::all::{ChannelId, CommandInteraction, Context, EditInteractionResponse};

use crate::Error;

pub async fn delete(
    ctx: &Context,
    interaction: &CommandInteraction,
    channel_id: ChannelId,
) -> Result<(), Error> {
    channel_id.delete(ctx).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel deleted."),
        )
        .await?;

    Ok(())
}
