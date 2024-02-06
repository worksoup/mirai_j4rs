use crate::event::{BotEventTrait, FriendEventTrait, FriendInfoChangeEventTrait, UserEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct FriendAddEvent {
    instance: Instance,
}

impl BotEventTrait for FriendAddEvent {}

impl UserEventTrait for FriendAddEvent {}

impl FriendEventTrait for FriendAddEvent {}

impl FriendInfoChangeEventTrait for FriendAddEvent {}
