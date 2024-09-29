use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditChannel, GuildChannel, ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn region(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    mut channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let region = match options.get("region") {
        Some(ResolvedValue::String(region)) => Some(region.to_string()),
        _ => None,
    };

    channel
        .edit(ctx, EditChannel::new().voice_region(region))
        .await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Channel region updated.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
