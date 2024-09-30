use serenity::all::{CommandInteraction, Context, EditInteractionResponse, GuildChannel};

use crate::Error;

pub async fn delete(
    ctx: &Context,
    interaction: &CommandInteraction,
    channel: GuildChannel,
) -> Result<(), Error> {
    channel.delete(ctx).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel deleted."),
        )
        .await?;

    Ok(())
}
