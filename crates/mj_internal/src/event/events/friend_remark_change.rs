use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BotEventTrait, BroadcastControllableTrait, FriendEventTrait, FriendInfoChangeEventTrait,
    UserEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct FriendRemarkChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> FriendEventTrait<B> for FriendRemarkChangeEvent<B> {}
impl<B: BotBackend> FriendInfoChangeEventTrait<B> for FriendRemarkChangeEvent<B> {}
impl<B:BotBackend> BroadcastControllableTrait<B> for FriendRemarkChangeEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for FriendRemarkChangeEvent<B> {}
impl<B: BotBackend> UserEventTrait<B> for FriendRemarkChangeEvent<B> {}
