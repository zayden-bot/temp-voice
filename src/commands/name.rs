use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, ResolvedOption,
    ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn name(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
) -> Result<(), Error> {
    let options = parse_options(options);

    let name = match options.get("name") {
        Some(ResolvedValue::String(name)) => name.to_string(),
        _ => format!("{}'s Channel", interaction.user.name),
    };

    channel_id.edit(ctx, EditChannel::new().name(name)).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel name updated."),
        )
        .await?;

    Ok(())
}
