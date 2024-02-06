use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait,
};

#[mj_event]
pub struct BotUnmuteEvent {
    instance: Instance,
}

impl BotEventTrait for BotUnmuteEvent {}

impl GroupEventTrait for BotUnmuteEvent {}

impl BotPassiveEventTrait for BotUnmuteEvent {}

impl BaseGroupMemberInfoChangeEventTrait for BotUnmuteEvent {}

impl GroupMemberInfoChangeEventTrait for BotUnmuteEvent {}
