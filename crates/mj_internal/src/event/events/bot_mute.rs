use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct BotMuteEvent {
    instance: Instance,
}

impl BotEventTrait for BotMuteEvent {}

impl GroupEventTrait for BotMuteEvent {}

impl BotPassiveEventTrait for BotMuteEvent {}

impl BaseGroupMemberInfoChangeEventTrait for BotMuteEvent {}

impl GroupMemberInfoChangeEventTrait for BotMuteEvent {}
