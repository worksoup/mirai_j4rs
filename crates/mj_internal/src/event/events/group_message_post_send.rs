use crate::contact::Group;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupMessagePostSendEvent")]
pub struct GroupMessagePostSendEvent {
    instance: Instance,
}

impl MessagePostSendEventTrait<Group> for GroupMessagePostSendEvent {}

impl BotEventTrait for GroupMessagePostSendEvent {}

impl BotActiveEventTrait for GroupMessagePostSendEvent {}

impl MiraiEventTrait for GroupMessagePostSendEvent {}
