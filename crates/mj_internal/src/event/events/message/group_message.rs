use crate::contact::{Group, Member};
use crate::event::{BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupMessageEvent {
    instance: Instance,
}

impl GroupMessageEvent {}

// 实现了 message 所需要的函数。
impl MessageEventTrait<Member, Group> for GroupMessageEvent {}
impl OtherClientEventTrait for GroupMessageEvent {}

impl BotEventTrait for GroupMessageEvent {}

impl BotPassiveEventTrait for GroupMessageEvent {}
