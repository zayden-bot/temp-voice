use std::collections::{HashMap, HashSet};

use serenity::all::{ChannelId, Context, UserId};
use serenity::prelude::TypeMapKey;

use crate::{Error, Result};

pub struct VoiceChannelManager;

impl VoiceChannelManager {
    pub async fn register_voice_channel(ctx: &Context, channel_id: ChannelId, owner_id: UserId) {
        let channel_data = VoiceChannelData::new(channel_id, owner_id);
        channel_data.save(ctx).await;
    }

    pub async fn take(ctx: &Context, channel_id: ChannelId) -> Result<VoiceChannelData> {
        let mut data = ctx.data.write().await;
        let manager = data
            .get_mut::<Self>()
            .expect("Expected VoiceChannelManager in TypeMap");

        match manager.remove(&channel_id) {
            Some(channel_data) => Ok(channel_data),
            None => Err(Error::ChannelNotFound(channel_id)),
        }
    }

    pub async fn verify_owner(
        ctx: &Context,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<bool> {
        let data = ctx.data.read().await;
        let manager = data
            .get::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let owner = match manager.get(&channel_id) {
            Some(channel_data) => channel_data.owner,
            None => return Err(Error::ChannelNotFound(channel_id)),
        };

        Ok(owner == user_id)
    }

    pub async fn verify_trusted(
        ctx: &Context,
        channel_id: ChannelId,
        user_id: UserId,
    ) -> Result<bool> {
        let data = ctx.data.read().await;
        let manager = data
            .get::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let channel_data = match manager.get(&channel_id) {
            Some(channel_data) => channel_data,
            None => return Err(Error::ChannelNotFound(channel_id)),
        };

        Ok(channel_data.trusted.contains(&user_id))
    }

    pub async fn verify_password(ctx: &Context, channel_id: ChannelId, pass: &str) -> Result<bool> {
        let data = ctx.data.read().await;
        let manager = data
            .get::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        let password = match manager.get(&channel_id) {
            Some(channel_data) => channel_data.password.as_deref(),
            None => return Err(Error::ChannelNotFound(channel_id)),
        };

        Ok(password == Some(pass))
    }
}

impl TypeMapKey for VoiceChannelManager {
    type Value = HashMap<ChannelId, VoiceChannelData>;
}

pub struct VoiceChannelData {
    channel_id: ChannelId,
    pub owner: UserId,
    trusted: HashSet<UserId>,
    open_invites: HashSet<UserId>,
    pub password: Option<String>,
}

impl VoiceChannelData {
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
            .get_mut::<VoiceChannelManager>()
            .expect("Expected VoiceChannelManager in TypeMap");
        manager.insert(self.channel_id, self);
    }
}
