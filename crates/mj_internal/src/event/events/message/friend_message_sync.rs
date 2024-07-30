use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Friend;
use crate::event::{BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct FriendMessageSyncEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> OtherClientEventTrait<B> for FriendMessageSyncEvent<B> {}
impl<B: BotBackend> MessageEventTrait<B, Friend<B>, Friend<B>> for FriendMessageSyncEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for FriendMessageSyncEvent<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for FriendMessageSyncEvent<B> {}
