use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, FriendEventTrait, FriendInfoChangeEventTrait, UserEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct FriendNickChangedEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for FriendNickChangedEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for FriendNickChangedEvent<B> {}

impl<B: BotBackend> FriendEventTrait<B> for FriendNickChangedEvent<B> {}
impl<B: BotBackend> FriendInfoChangeEventTrait<B> for FriendNickChangedEvent<B> {}
