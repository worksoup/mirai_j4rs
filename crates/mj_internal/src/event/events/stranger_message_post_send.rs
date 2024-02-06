use j4rs::Instance;

use mj_macro::mj_event;

use crate::contact::Stranger;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, UserMessagePostSendEventTrait,
};

#[mj_event]
pub struct StrangerMessagePostSendEvent {
    instance: Instance,
}

impl MessagePostSendEventTrait<Stranger> for StrangerMessagePostSendEvent {}

impl BotEventTrait for StrangerMessagePostSendEvent {}

impl BotActiveEventTrait for StrangerMessagePostSendEvent {}

impl UserMessagePostSendEventTrait<Stranger> for StrangerMessagePostSendEvent {}
