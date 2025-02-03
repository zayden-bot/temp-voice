use serenity::all::{Context, Guild};

use crate::VoiceStateCache;

pub async fn guild_create(ctx: &Context, guild: &Guild) {
    let cache = VoiceStateCache::new_with_states(ctx, &guild.voice_states).await;

    let mut data = ctx.data.write().await;
    data.insert::<VoiceStateCache>(cache);
}
