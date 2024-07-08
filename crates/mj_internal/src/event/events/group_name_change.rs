use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};

#[mj_event]
pub struct GroupNameChangeEvent {
    instance: Instance,
}
impl GroupSettingsChangeEventTrait<String> for GroupNameChangeEvent {}
impl GroupOperableEventTrait for GroupNameChangeEvent {}
impl GroupMemberInfoChangeEventTrait for GroupNameChangeEvent {}

impl BotEventTrait for GroupNameChangeEvent {}
impl GroupEventTrait for GroupNameChangeEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupNameChangeEvent {}
impl BroadcastControllableTrait for GroupNameChangeEvent {}
impl BotPassiveEventTrait for GroupNameChangeEvent {}
