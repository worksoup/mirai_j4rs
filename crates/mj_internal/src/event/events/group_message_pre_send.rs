use j4rs::Instance;

use mj_helper_macro::mj_event;

// TODO
use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
};

#[mj_event]
pub struct GroupMessagePreSendEvent {
    instance: Instance,
}

impl BotEventTrait for GroupMessagePreSendEvent {}

impl BotActiveEventTrait for GroupMessagePreSendEvent {}

impl CancellableEventTrait for GroupMessagePreSendEvent {}

impl MessagePreSendEventTrait for GroupMessagePreSendEvent {}
