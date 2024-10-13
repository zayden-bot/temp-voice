use serenity::all::{
    CommandInteraction, Context, EditChannel, EditInteractionResponse, GuildChannel,
    ResolvedOption, ResolvedValue,
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

    let limit = match options.get("user_limit") {
        Some(ResolvedValue::Integer(limit)) => (*limit).clamp(0, 99) as u32,
        _ => 0,
    };

    channel
        .edit(ctx, EditChannel::new().user_limit(limit))
        .await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content(format!("User limit set to {}", limit)),
        )
        .await?;

    Ok(())
}
