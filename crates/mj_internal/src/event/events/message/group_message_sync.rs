use crate::contact::{Group, Member};
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    MessageSyncEventTrait, OtherClientEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupMessageSyncEvent {
    instance: Instance,
}

impl GroupAwareMessageTrait<Member, Group> for GroupMessageSyncEvent {}
impl MessageSyncEventTrait<Member, Group> for GroupMessageSyncEvent {}

impl BotPassiveEventTrait for GroupMessageSyncEvent {}
impl MessageEventTrait<Member, Group> for GroupMessageSyncEvent {}
impl OtherClientEventTrait for GroupMessageSyncEvent {}
impl BotEventTrait for GroupMessageSyncEvent {}
