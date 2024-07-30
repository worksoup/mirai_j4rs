use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, FriendEventTrait, UserEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct FriendInputStatusChangedEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for FriendInputStatusChangedEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for FriendInputStatusChangedEvent<B> {}

impl<B: BotBackend> FriendEventTrait<B> for FriendInputStatusChangedEvent<B> {}
