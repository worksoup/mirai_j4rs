use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Friend;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, FriendEventTrait, MessageEventTrait,
    OtherClientEventTrait, UserEventTrait, UserMessageEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct FriendMessageEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> FriendMessageEvent<B> {}

impl<B: BotBackend> MessageEventTrait<B, Friend<B>, Friend<B>> for FriendMessageEvent<B> {}
impl<B: BotBackend> OtherClientEventTrait<B> for FriendMessageEvent<B> {}

impl<B: BotBackend> UserMessageEventTrait<B, Friend<B>, Friend<B>> for FriendMessageEvent<B> {}
impl<B: BotBackend> UserEventTrait<B> for FriendMessageEvent<B> {}

impl<B: BotBackend> FriendEventTrait<B> for FriendMessageEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for FriendMessageEvent<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for FriendMessageEvent<B> {}
