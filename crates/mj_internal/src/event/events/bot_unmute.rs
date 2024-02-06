use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BotUnmuteEvent {
    instance: Instance,
}

impl BotEventTrait for BotUnmuteEvent {}

impl GroupEventTrait for BotUnmuteEvent {}

impl BotPassiveEventTrait for BotUnmuteEvent {}

impl BaseGroupMemberInfoChangeEventTrait for BotUnmuteEvent {}

impl GroupMemberInfoChangeEventTrait for BotUnmuteEvent {}
