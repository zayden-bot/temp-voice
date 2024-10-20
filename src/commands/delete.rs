use serenity::all::{ChannelId, CommandInteraction, Context, EditInteractionResponse};
use sqlx::{Database, Pool};

use crate::error::PermissionError;
use crate::{Error, VoiceChannelData, VoiceChannelManager};

pub async fn delete<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    channel_id: ChannelId,
    row: VoiceChannelData,
) -> Result<(), Error> {
    if row.is_owner(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    row.delete::<Db, Manager>(pool).await?;

    channel_id.delete(ctx).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Channel deleted."),
        )
        .await?;

    Ok(())
}
