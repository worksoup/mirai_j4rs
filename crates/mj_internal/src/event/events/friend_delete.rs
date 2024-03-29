use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{BotEventTrait, FriendEventTrait, FriendInfoChangeEventTrait, UserEventTrait};

#[mj_event]
pub struct FriendDeleteEvent {
    instance: Instance,
}

impl BotEventTrait for FriendDeleteEvent {}

impl UserEventTrait for FriendDeleteEvent {}

impl FriendEventTrait for FriendDeleteEvent {}

impl FriendInfoChangeEventTrait for FriendDeleteEvent {}
