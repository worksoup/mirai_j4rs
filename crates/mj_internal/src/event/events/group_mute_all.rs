use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupMuteAllEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<bool> for GroupMuteAllEvent {}
impl GroupOperableEventTrait for GroupMuteAllEvent {}
impl GroupMemberInfoChangeEventTrait for GroupMuteAllEvent {}

impl BotEventTrait for GroupMuteAllEvent {}
impl GroupEventTrait for GroupMuteAllEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupMuteAllEvent {}
impl BroadcastControllableTrait for GroupMuteAllEvent {}
impl BotPassiveEventTrait for GroupMuteAllEvent {}
