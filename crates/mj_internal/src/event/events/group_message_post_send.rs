use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Group;
use crate::event::{BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait};

#[mj_event]
pub struct GroupMessagePostSendEvent {
    instance: Instance,
}

impl MessagePostSendEventTrait<Group> for GroupMessagePostSendEvent {}

impl BotEventTrait for GroupMessagePostSendEvent {}

impl BotActiveEventTrait for GroupMessagePostSendEvent {}
