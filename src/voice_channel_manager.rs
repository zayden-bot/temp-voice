use std::collections::HashSet;

use async_trait::async_trait;
use serenity::all::{ChannelId, UserId};
use sqlx::any::AnyQueryResult;
use sqlx::prelude::FromRow;
use sqlx::{Database, Pool};

use crate::Result;

#[async_trait]
pub trait VoiceChannelManager<Db: Database> {
    async fn save(
        pool: &Pool<Db>,
        id: impl Into<i64> + Send,
        owner_id: impl Into<i64> + Send,
        trusted_ids: &[i64],
        password: Option<&str>,
        persistent: impl Into<bool> + Send,
    ) -> sqlx::Result<AnyQueryResult>;
    async fn get(pool: &Pool<Db>, id: ChannelId) -> sqlx::Result<Option<VoiceChannelData>>;
    async fn delete(pool: &Pool<Db>, id: ChannelId) -> sqlx::Result<AnyQueryResult>;
}

#[derive(FromRow)]
pub struct VoiceChannelRow {
    pub id: i64,
    pub owner_id: i64,
    pub trusted_ids: Vec<i64>,
    pub password: Option<String>,
    pub persistent: bool,
}

#[derive(Default)]
pub struct VoiceChannelData {
    pub id: ChannelId,
    pub owner_id: UserId,
    pub trusted_ids: HashSet<UserId>,
    pub open_invites: HashSet<UserId>,
    pub password: Option<String>,
    pub persistent: bool,
}

impl VoiceChannelData {
    pub fn new(id: ChannelId, owner_id: UserId) -> Self {
        Self {
            id,
            owner_id,
            trusted_ids: HashSet::new(),
            open_invites: HashSet::new(),
            password: None,
            persistent: false,
        }
    }

    pub fn is_owner(&self, user_id: UserId) -> bool {
        self.owner_id == user_id
    }

    pub fn is_trusted(&self, user_id: UserId) -> bool {
        self.trusted_ids.contains(&user_id) || self.owner_id == user_id
    }

    pub fn verify_password(&self, pass: &str) -> bool {
        self.password.as_deref() == Some(pass)
    }

    pub fn toggle_persist(&mut self) {
        self.persistent = !self.persistent;
    }

    pub fn is_persistent(&self) -> bool {
        self.persistent
    }

    pub fn trust(&mut self, id: impl Into<UserId>) {
        self.trusted_ids.insert(id.into());
    }

    pub fn untrust(&mut self, id: impl Into<UserId>) {
        self.trusted_ids.remove(&id.into());
    }

    pub fn create_invite(&mut self, id: impl Into<UserId>) {
        self.open_invites.insert(id.into());
    }

    pub fn block(&mut self, id: UserId) {
        self.trusted_ids.remove(&id);
        self.open_invites.remove(&id);
    }

    pub fn reset(&mut self) {
        self.trusted_ids.clear();
        self.open_invites.clear();
        self.password = None;
    }

    pub async fn save<Db: Database, Manager: VoiceChannelManager<Db>>(
        self,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let trusted_ids = self
            .trusted_ids
            .iter()
            .map(|id| id.get() as i64)
            .collect::<Vec<_>>();

        Manager::save(
            pool,
            self.id.get() as i64,
            self.owner_id.get() as i64,
            &trusted_ids,
            self.password.as_deref(),
            self.persistent,
        )
        .await
        .unwrap();

        Ok(())
    }

    pub async fn delete<Db: Database, Manager: VoiceChannelManager<Db>>(
        self,
        pool: &Pool<Db>,
    ) -> Result<()> {
        Manager::delete(pool, self.id).await.unwrap();

        Ok(())
    }
}

impl From<VoiceChannelRow> for VoiceChannelData {
    fn from(row: VoiceChannelRow) -> Self {
        Self {
            id: ChannelId::new(row.id as u64),
            owner_id: UserId::new(row.owner_id as u64),
            trusted_ids: row
                .trusted_ids
                .into_iter()
                .map(|id| UserId::new(id as u64))
                .collect(),
            open_invites: HashSet::new(),
            password: row.password,
            persistent: row.persistent,
        }
    }
}
