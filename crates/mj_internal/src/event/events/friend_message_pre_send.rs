use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    UserMessagePreSendEventTrait,
};

#[mj_event]
pub struct FriendMessagePreSendEvent {
    instance: Instance,
}

impl MessagePreSendEventTrait for FriendMessagePreSendEvent {}

impl BotEventTrait for FriendMessagePreSendEvent {}

impl BotActiveEventTrait for FriendMessagePreSendEvent {}

impl CancellableEventTrait for FriendMessagePreSendEvent {}

impl UserMessagePreSendEventTrait for FriendMessagePreSendEvent {}
