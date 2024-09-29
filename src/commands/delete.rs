use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    GuildChannel,
};

use crate::Error;

pub async fn delete(
    ctx: &Context,
    interaction: &CommandInteraction,
    channel: GuildChannel,
) -> Result<(), Error> {
    channel.delete(ctx).await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Channel deleted.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
