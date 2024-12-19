use async_trait::async_trait;
use serenity::all::{ChannelId, GuildId};
use sqlx::any::AnyQueryResult;
use sqlx::{Database, FromRow, Pool};

#[async_trait]
pub trait TempVoiceGuildManager<Db: Database> {
    async fn save(
        pool: &Pool<Db>,
        id: GuildId,
        category: ChannelId,
        creator_channel: ChannelId,
    ) -> sqlx::Result<AnyQueryResult>;

    async fn get(pool: &Pool<Db>, id: GuildId) -> sqlx::Result<TempVoiceRow>;

    async fn get_category(pool: &Pool<Db>, id: GuildId) -> sqlx::Result<ChannelId>;

    async fn get_creator_channel(pool: &Pool<Db>, id: GuildId) -> sqlx::Result<ChannelId>;
}

#[derive(FromRow)]
pub struct TempVoiceRow {
    pub id: i64,
    pub temp_voice_category: i64,
    pub temp_voice_creator_channel: i64,
}

impl TempVoiceRow {
    pub fn guild_id(&self) -> GuildId {
        GuildId::from(self.id as u64)
    }

    pub fn category(&self) -> ChannelId {
        ChannelId::from(self.temp_voice_category as u64)
    }

    pub fn creator_channel(&self) -> ChannelId {
        ChannelId::from(self.temp_voice_creator_channel as u64)
    }
}
