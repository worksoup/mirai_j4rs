use crate::contact::NormalMember;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    MessageSyncEventTrait, MiraiEventTrait, OtherClientEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupTempMessageSyncEvent")]
pub struct GroupTempMessageSyncEvent {
    instance: Instance,
}
impl MessageSyncEventTrait<NormalMember, NormalMember> for GroupTempMessageSyncEvent {}
impl GroupAwareMessageTrait<NormalMember, NormalMember> for GroupTempMessageSyncEvent {}

impl BotEventTrait for GroupTempMessageSyncEvent {}
impl OtherClientEventTrait for GroupTempMessageSyncEvent {}
impl BotPassiveEventTrait for GroupTempMessageSyncEvent {}
impl MiraiEventTrait for GroupTempMessageSyncEvent {}
impl MessageEventTrait<NormalMember, NormalMember> for GroupTempMessageSyncEvent {}
