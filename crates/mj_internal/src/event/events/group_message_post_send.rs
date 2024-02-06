use crate::contact::Group;
use crate::event::{BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupMessagePostSendEvent {
    instance: Instance,
}

impl MessagePostSendEventTrait<Group> for GroupMessagePostSendEvent {}

impl BotEventTrait for GroupMessagePostSendEvent {}

impl BotActiveEventTrait for GroupMessagePostSendEvent {}
