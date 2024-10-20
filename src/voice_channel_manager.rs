use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use serenity::all::{ChannelId, Context, UserId};
use serenity::prelude::TypeMapKey;
use sqlx::any::AnyQueryResult;
use sqlx::{Database, Pool};

use crate::{Error, Result};

#[async_trait]
pub trait TemporaryVoiceChannelManager {
    async fn register_voice_channel(ctx: &Context, channel_id: ChannelId, owner_id: UserId);
    async fn take(ctx: &Context, channel_id: ChannelId) -> Result<TemporaryChannelData>;
    async fn verify_owner(ctx: &Context, channel_id: ChannelId, user_id: UserId) -> Result<bool>;
    async fn verify_trusted(ctx: &Context, channel_id: ChannelId, user_id: UserId) -> Result<bool>;
    async fn verify_password(ctx: &Context, channel_id: ChannelId, pass: &str) -> Result<bool>;
}

pub struct VoiceChannelMap;

#[async_trait]
impl TemporaryVoiceChannelManager for VoiceChannelMap {
    async fn register_voice_channel(ctx: &Context, channel_id: ChannelId, owner_id: UserId) {
        let channel_data = TemporaryChannelData::new(channel_id, owner_id);
        channel_data.save(ctx).await;
    }

    async fn take(ctx: &Context, channel_id: ChannelId) -> Result<TemporaryChannelData> {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<Self>()
            .expect("Expected VoiceChannelManager in TypeMap");

        match manager.remove(&channel_id) {
            Some(channel_data) => Ok(channel_data),
            None => Err(Error::ChannelNotFound(channel_id)),
        }
    }

    async fn verify_owner(ctx: &Context, channel_id: ChannelId, user_id: UserId) -> Result<bool> {
        let data = ctx.data.read().await;
        let manager = data
            .get::<Self>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let owner = match manager.get(&channel_id) {
            Some(channel_data) => channel_data.owner,
            None => return Err(Error::ChannelNotFound(channel_id)),
        };

        Ok(owner == user_id)
    }

    async fn verify_trusted(ctx: &Context, channel_id: ChannelId, user_id: UserId) -> Result<bool> {
        let data = ctx.data.read().await;
        let manager = data
            .get::<Self>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = match manager.get(&channel_id) {
            Some(channel_data) => channel_data,
            None => return Err(Error::ChannelNotFound(channel_id)),
        };

        Ok(channel_data.owner == user_id || channel_data.trusted.contains(&user_id))
    }

    async fn verify_password(ctx: &Context, channel_id: ChannelId, pass: &str) -> Result<bool> {
        let data = ctx.data.read().await;
        let manager = data
            .get::<Self>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let password = match manager.get(&channel_id) {
            Some(channel_data) => channel_data.password.as_deref(),
            None => return Err(Error::ChannelNotFound(channel_id)),
        };

        Ok(password == Some(pass))
    }
}

impl TypeMapKey for VoiceChannelMap {
    type Value = HashMap<ChannelId, TemporaryChannelData>;
}

pub struct TemporaryChannelData {
    pub channel_id: ChannelId,
    pub owner: UserId,
    pub trusted: HashSet<UserId>,
    pub open_invites: HashSet<UserId>,
    pub password: Option<String>,
}

impl TemporaryChannelData {
    pub fn new(channel_id: ChannelId, owner: impl Into<UserId>) -> Self {
        Self {
            channel_id,
            owner: owner.into(),
            trusted: HashSet::new(),
            open_invites: HashSet::new(),
            password: None,
        }
    }

    pub fn trust(&mut self, id: impl Into<UserId>) {
        self.trusted.insert(id.into());
    }

    pub fn untrust(&mut self, id: impl Into<UserId>) {
        self.trusted.remove(&id.into());
    }

    pub fn create_invite(&mut self, id: impl Into<UserId>) {
        self.open_invites.insert(id.into());
    }

    pub fn block(&mut self, id: UserId) {
        self.trusted.remove(&id);
        self.open_invites.remove(&id);
    }

    pub fn reset(&mut self) {
        self.trusted.clear();
        self.open_invites.clear();
        self.password = None;
    }

    pub async fn save(self, ctx: &Context) {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<VoiceChannelMap>()
            .expect("Expected VoiceChannelManager in TypeMap");
        manager.insert(self.channel_id, self);
    }
}

#[async_trait]
pub trait PersistentVoiceChannelManager {
    async fn persist(
        pool: &Pool<impl Database>,
        channel_data: &TemporaryChannelData,
    ) -> sqlx::Result<AnyQueryResult>;
    async fn is_persistent(pool: &Pool<impl Database>, channel_id: ChannelId)
        -> sqlx::Result<bool>;
}

pub struct PersistentChannelData {
    id: i64,
    owner_id: i64,
    trusted_ids: Vec<i64>,
    password: Option<String>,
}

impl PersistentChannelData {
    pub fn channel_id(&self) -> ChannelId {
        ChannelId::new(self.id as u64)
    }
}
