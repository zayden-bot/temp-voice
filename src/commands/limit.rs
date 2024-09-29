use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditChannel, GuildChannel, ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn limit(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    mut channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let limit = match options.get("limit") {
        Some(ResolvedValue::Integer(limit)) => (*limit).clamp(0, 99) as u32,
        _ => 0,
    };

    channel
        .edit(ctx, EditChannel::new().user_limit(limit))
        .await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Channel user limit updated.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
