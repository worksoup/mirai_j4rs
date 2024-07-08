use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    UserMessagePreSendEventTrait,
};

#[mj_event]
pub struct StrangerMessagePreSendEvent {
    instance: Instance,
}

impl MessagePreSendEventTrait for StrangerMessagePreSendEvent {}

impl BotEventTrait for StrangerMessagePreSendEvent {}

impl BotActiveEventTrait for StrangerMessagePreSendEvent {}

impl CancellableEventTrait for StrangerMessagePreSendEvent {}

impl UserMessagePreSendEventTrait for StrangerMessagePreSendEvent {}
