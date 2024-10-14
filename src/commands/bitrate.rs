use serenity::all::{
    ChannelId, CommandInteraction, Context, EditChannel, EditInteractionResponse, ResolvedOption,
    ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn bitrate(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
) -> Result<(), Error> {
    let options = parse_options(options);

    let kbps = match options.get("kbps") {
        Some(ResolvedValue::Integer(kbps)) => *kbps as u32,
        _ => unreachable!("Kbps option is required"),
    };

    channel_id
        .edit(ctx, EditChannel::new().bitrate(kbps * 1000))
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel bitrate updated."),
        )
        .await?;

    Ok(())
}
