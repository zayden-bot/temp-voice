use std::collections::{HashMap, HashSet};

use serenity::{
    all::{ChannelId, UserId},
    prelude::TypeMapKey,
};

pub struct VoiceChannelManager;

impl TypeMapKey for VoiceChannelManager {
    type Value = HashMap<ChannelId, VoiceChannelData>;
}

pub struct VoiceChannelData {
    pub owner: UserId,
    trusted: HashSet<UserId>,
    open_invites: HashSet<UserId>,
    pub password: Option<String>,
}

impl VoiceChannelData {
    pub fn new(owner: impl Into<UserId>) -> Self {
        Self {
            owner: owner.into(),
            trusted: HashSet::new(),
            open_invites: HashSet::new(),
            password: None,
        }
    }

    pub fn add_trusted(&mut self, id: impl Into<UserId>) {
        self.trusted.insert(id.into());
    }

    pub fn remove_trusted(&mut self, id: impl Into<UserId>) {
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
}
