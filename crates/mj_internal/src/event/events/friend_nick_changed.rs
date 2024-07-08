use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, FriendEventTrait, FriendInfoChangeEventTrait, UserEventTrait};

#[mj_event]
pub struct FriendNickChangedEvent {
    instance: Instance,
}

impl BotEventTrait for FriendNickChangedEvent {}

impl UserEventTrait for FriendNickChangedEvent {}

impl FriendEventTrait for FriendNickChangedEvent {}
impl FriendInfoChangeEventTrait for FriendNickChangedEvent {}
