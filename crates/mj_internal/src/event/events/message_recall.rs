use crate::event::{BotEventTrait, GroupEventTrait, GroupOperableEventTrait};
use j4rs::Instance;
use mj_macro::mj_event;

pub trait MessageRecallTrait: BotEventTrait {}
#[mj_event("net.mamoe.mirai.event.events.MessageRecallEvent$FriendRecall")]
pub struct FriendRecall {
    instance: Instance,
}

impl BotEventTrait for FriendRecall {}

impl MessageRecallTrait for FriendRecall {}

#[mj_event("net.mamoe.mirai.event.events.MessageRecallEvent$GroupRecall")]
pub struct GroupRecall {
    instance: Instance,
}

impl BotEventTrait for GroupRecall {}

impl MessageRecallTrait for GroupRecall {}

impl GroupEventTrait for GroupRecall {}

impl GroupOperableEventTrait for GroupRecall {}

#[mj_event]
pub struct MessageRecallEvent {
    instance: Instance,
}

impl BotEventTrait for MessageRecallEvent {}

impl MessageRecallTrait for MessageRecallEvent {}
