use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::{Group, Member};
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    MessageSyncEventTrait, OtherClientEventTrait,
};

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
