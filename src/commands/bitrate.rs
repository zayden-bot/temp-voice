use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditChannel, GuildChannel, ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn bitrate(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    mut channel: GuildChannel,
) -> Result<(), Error> {
    let options = parse_options(options);

    let kbps = match options.get("kbps") {
        Some(ResolvedValue::Integer(kbps)) => *kbps as u32,
        _ => unreachable!("Kbps option is required"),
    };

    channel
        .edit(ctx, EditChannel::new().bitrate(kbps * 1000))
        .await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Channel bitrate updated.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
