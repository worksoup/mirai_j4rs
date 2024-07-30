use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Friend;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, UserMessagePostSendEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct FriendMessagePostSendEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessagePostSendEventTrait<B, Friend<B>> for FriendMessagePostSendEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for FriendMessagePostSendEvent<B> {}

impl<B: BotBackend> BotActiveEventTrait<B> for FriendMessagePostSendEvent<B> {}

impl<B: BotBackend> UserMessagePostSendEventTrait<B, Friend<B>> for FriendMessagePostSendEvent<B> {}
