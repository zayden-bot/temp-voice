use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    GuildId, ResolvedOption, ResolvedValue,
};
use zayden_core::parse_options;

use crate::Error;

pub async fn kick(
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &Vec<ResolvedOption<'_>>,
    guild_id: GuildId,
) -> Result<(), Error> {
    let options = parse_options(options);

    let user = match options.get("member") {
        Some(ResolvedValue::User(user, _)) => *user,
        _ => unreachable!("Member option is required"),
    };

    guild_id.disconnect_member(ctx, user).await?;

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("User kicked from channel.")
                    .ephemeral(true),
            ),
        )
        .await?;

    Ok(())
}
