use serenity::all::{CommandInteraction, Context, EditInteractionResponse};
use sqlx::{Database, Pool};

use crate::error::PermissionError;
use crate::{Error, Result, VoiceChannelManager, VoiceChannelRow};

pub async fn persist<Db: Database, Manager: VoiceChannelManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    mut row: VoiceChannelRow,
) -> Result<()> {
    interaction.defer_ephemeral(ctx).await.unwrap();

    if row.is_owner(interaction.user.id) {
        return Err(Error::MissingPermissions(PermissionError::NotOwner));
    }

    if interaction.member.as_ref().unwrap().premium_since.is_none() {
        return Err(Error::PremiumRequired);
    }

    row.toggle_persist();
    let state = if row.is_persistent() {
        "enabled"
    } else {
        "disabled"
    };

    row.save::<Db, Manager>(pool).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new()
                .content(format!("Channel persistence is now {}.", state)),
        )
        .await
        .unwrap();

    Ok(())
}
