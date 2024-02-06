use crate::event::{BotEventTrait, FriendEventTrait, UserEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct FriendInputStatusChangedEvent {
    instance: Instance,
}

impl BotEventTrait for FriendInputStatusChangedEvent {}

impl UserEventTrait for FriendInputStatusChangedEvent {}

impl FriendEventTrait for FriendInputStatusChangedEvent {}
