use serenity::all::{
    ChannelId, CommandInteraction, Context, EditInteractionResponse, ResolvedOption, ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::{Error, PersistentVoiceChannelManager, TemporaryVoiceChannelManager};

pub async fn persist<
    TempManager: TemporaryVoiceChannelManager,
    PersistentManager: PersistentVoiceChannelManager,
>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<impl Database>,
    options: &Vec<ResolvedOption<'_>>,
    channel_id: Option<ChannelId>,
) -> Result<(), Error> {
    if interaction.member.as_ref().unwrap().premium_since.is_none() {
        return Err(Error::PremiumRequired);
    }

    let options = parse_options(options);

    let channel_id = match (options.get("channel"), channel_id) {
        (Some(ResolvedValue::Channel(channel)), _) => channel.id,
        (_, Some(channel_id)) => channel_id,
        _ => return Err(Error::MemberNotInVoiceChannel),
    };

    let channel_data = TempManager::take(ctx, channel_id).await?;

    PersistentManager::persist(pool, &channel_data).await?;

    channel_data.save(ctx).await;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Set user to blocked."),
        )
        .await?;

    Ok(())
}
