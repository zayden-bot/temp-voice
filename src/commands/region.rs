use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, ResolvedOption,
    ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn region(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
) -> Result<(), Error> {
    let options = parse_options(options);

    let region = match options.get("region") {
        Some(ResolvedValue::String(region)) => Some(region.to_string()),
        _ => None,
    };

    channel_id
        .edit(ctx, EditChannel::new().voice_region(region))
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel region updated."),
        )
        .await?;

    Ok(())
}
