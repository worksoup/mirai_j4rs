use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupNameChangeEvent")]
pub struct GroupNameChangeEvent {
    instance: Instance,
}
impl GroupSettingsChangeEventTrait<String> for GroupNameChangeEvent {}
impl GroupOperableEventTrait for GroupNameChangeEvent {}
impl GroupMemberInfoChangeEventTrait for GroupNameChangeEvent {}

impl MiraiEventTrait for GroupNameChangeEvent {}
impl BotEventTrait for GroupNameChangeEvent {}
impl GroupEventTrait for GroupNameChangeEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupNameChangeEvent {}
impl BroadcastControllableTrait for GroupNameChangeEvent {}
impl BotPassiveEventTrait for GroupNameChangeEvent {}
