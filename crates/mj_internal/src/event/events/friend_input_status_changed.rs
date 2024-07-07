use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, FriendEventTrait, UserEventTrait};

#[mj_event]
pub struct FriendInputStatusChangedEvent {
    instance: Instance,
}

impl BotEventTrait for FriendInputStatusChangedEvent {}

impl UserEventTrait for FriendInputStatusChangedEvent {}

impl FriendEventTrait for FriendInputStatusChangedEvent {}
