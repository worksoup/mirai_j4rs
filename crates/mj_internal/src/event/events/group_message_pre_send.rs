// TODO
use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupMessagePreSendEvent")]
pub struct GroupMessagePreSendEvent {
    instance: Instance,
}

impl BotEventTrait for GroupMessagePreSendEvent {}

impl BotActiveEventTrait for GroupMessagePreSendEvent {}

impl CancellableEventTrait for GroupMessagePreSendEvent {}

impl MiraiEventTrait for GroupMessagePreSendEvent {}

impl MessagePreSendEventTrait for GroupMessagePreSendEvent {}
