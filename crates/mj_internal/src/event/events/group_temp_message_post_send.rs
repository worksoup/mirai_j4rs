use crate::contact::Member;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, MiraiEventTrait,
    TempMessagePostSendEventTrait, UserMessagePostSendEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupTempMessagePostSendEvent")]
pub struct GroupTempMessagePostSendEvent {
    instance: Instance,
}
impl TempMessagePostSendEventTrait for GroupTempMessagePostSendEvent {}

impl UserMessagePostSendEventTrait<Member> for GroupTempMessagePostSendEvent {}
impl MessagePostSendEventTrait<Member> for GroupTempMessagePostSendEvent {}
impl BotEventTrait for GroupTempMessagePostSendEvent {}
impl BotActiveEventTrait for GroupTempMessagePostSendEvent {}
impl MiraiEventTrait for GroupTempMessagePostSendEvent {}
