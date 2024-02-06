use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    UserMessagePreSendEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct FriendMessagePreSendEvent {
    instance: Instance,
}

impl MessagePreSendEventTrait for FriendMessagePreSendEvent {}

impl BotEventTrait for FriendMessagePreSendEvent {}

impl BotActiveEventTrait for FriendMessagePreSendEvent {}

impl CancellableEventTrait for FriendMessagePreSendEvent {}

impl UserMessagePreSendEventTrait for FriendMessagePreSendEvent {}
