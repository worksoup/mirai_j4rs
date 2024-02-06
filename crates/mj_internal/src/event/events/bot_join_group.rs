use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BotJoinGroupEvent {
    instance: Instance,
}
impl BotEventTrait for BotJoinGroupEvent {}
impl BotPassiveEventTrait for BotJoinGroupEvent {}
impl GroupEventTrait for BotJoinGroupEvent {}
impl BaseGroupMemberInfoChangeEventTrait for BotJoinGroupEvent {}
impl GroupMemberInfoChangeEventTrait for BotJoinGroupEvent {}
