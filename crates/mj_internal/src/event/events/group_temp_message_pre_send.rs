use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    TempMessagePreSendEventTrait, UserMessagePreSendEventTrait,
};

#[mj_event]
pub struct GroupTempMessagePreSendEvent {
    instance: Instance,
}

impl UserMessagePreSendEventTrait for GroupTempMessagePreSendEvent {}

impl MessagePreSendEventTrait for GroupTempMessagePreSendEvent {}

impl BotEventTrait for GroupTempMessagePreSendEvent {}

impl BotActiveEventTrait for GroupTempMessagePreSendEvent {}

impl CancellableEventTrait for GroupTempMessagePreSendEvent {}

impl TempMessagePreSendEventTrait for GroupTempMessagePreSendEvent {}
