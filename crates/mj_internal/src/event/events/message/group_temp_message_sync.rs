use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::NormalMember;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    MessageSyncEventTrait, OtherClientEventTrait,
};

#[mj_event]
pub struct GroupTempMessageSyncEvent {
    instance: Instance,
}
impl MessageSyncEventTrait<NormalMember, NormalMember> for GroupTempMessageSyncEvent {}
impl GroupAwareMessageTrait<NormalMember, NormalMember> for GroupTempMessageSyncEvent {}

impl BotEventTrait for GroupTempMessageSyncEvent {}
impl OtherClientEventTrait for GroupTempMessageSyncEvent {}
impl BotPassiveEventTrait for GroupTempMessageSyncEvent {}
impl MessageEventTrait<NormalMember, NormalMember> for GroupTempMessageSyncEvent {}
