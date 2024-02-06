use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    MiraiEventTrait, TempMessagePreSendEventTrait, UserMessagePreSendEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupTempMessagePreSendEvent")]
pub struct GroupTempMessagePreSendEvent {
    instance: Instance,
}

impl UserMessagePreSendEventTrait for GroupTempMessagePreSendEvent {}

impl MessagePreSendEventTrait for GroupTempMessagePreSendEvent {}

impl BotEventTrait for GroupTempMessagePreSendEvent {}

impl BotActiveEventTrait for GroupTempMessagePreSendEvent {}

impl CancellableEventTrait for GroupTempMessagePreSendEvent {}

impl MiraiEventTrait for GroupTempMessagePreSendEvent {}

impl TempMessagePreSendEventTrait for GroupTempMessagePreSendEvent {}
