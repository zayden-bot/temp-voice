use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::{Database, Pool};

use crate::error::PermissionError;
use crate::{Error, Result, VoiceChannelData, VoiceChannelManager};

pub async fn persist<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    mut row: VoiceChannelData,
) -> Result<()> {
    if interaction.user.id != row.owner_id {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    // if interaction.member.as_ref().unwrap().premium_since.is_none() {
    //     return Err(Error::PremiumRequired);
    // }

    row.toggle_persist();
    row.save::<Db, Manager>(pool).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Set user to blocked."),
        )
        .await?;

    Ok(())
}
