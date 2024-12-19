use std::collections::HashMap;

use serenity::all::{
    CommandInteraction, Context, CreateChannel, EditInteractionResponse, GuildId, ResolvedValue,
};
use sqlx::{Database, Pool};

use crate::{guild_manager::TempVoiceGuildManager, Result};

pub async fn setup<Db: Database, Manager: TempVoiceGuildManager<Db>>(
    ctx: &Context,
    interaction: &CommandInteraction,
    pool: &Pool<Db>,
    guild_id: GuildId,
    mut options: HashMap<&str, &ResolvedValue<'_>>,
) -> Result<()> {
    interaction.defer_ephemeral(ctx).await?;

    let category = match options.remove("category") {
        Some(ResolvedValue::Channel(category)) => *category,
        _ => unreachable!("Category is required"),
    };

    let creator_channel = guild_id
        .create_channel(
            ctx,
            CreateChannel::new("➕ Creator Channel").category(category.id),
        )
        .await?;

    Manager::save(pool, guild_id, category.id, creator_channel.id).await?;

    interaction
        .edit_response(
            ctx,
            EditInteractionResponse::new().content("Setup complete."),
        )
        .await?;

    Ok(())
}
