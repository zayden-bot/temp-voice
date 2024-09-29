use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditChannel, GuildChannel, ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn name(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    mut channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let name = match options.get("name") {
        Some(ResolvedValue::String(name)) => name.to_string(),
        _ => format!("{}'s Channel", interaction.user.name),
    };

    channel.edit(ctx, EditChannel::new().name(name)).await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Channel name updated.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
