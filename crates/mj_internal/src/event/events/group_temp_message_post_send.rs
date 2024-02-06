use crate::contact::Member;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, TempMessagePostSendEventTrait,
    UserMessagePostSendEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupTempMessagePostSendEvent {
    instance: Instance,
}
impl TempMessagePostSendEventTrait for GroupTempMessagePostSendEvent {}

impl UserMessagePostSendEventTrait<Member> for GroupTempMessagePostSendEvent {}
impl MessagePostSendEventTrait<Member> for GroupTempMessagePostSendEvent {}
impl BotEventTrait for GroupTempMessagePostSendEvent {}
impl BotActiveEventTrait for GroupTempMessagePostSendEvent {}
