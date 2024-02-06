use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupSettingsChangeEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupAllowConfessTalkEvent")]
pub struct GroupAllowConfessTalkEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<bool> for GroupAllowConfessTalkEvent {}
impl GroupMemberInfoChangeEventTrait for GroupAllowConfessTalkEvent {}

impl BotEventTrait for GroupAllowConfessTalkEvent {}
impl GroupEventTrait for GroupAllowConfessTalkEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupAllowConfessTalkEvent {}
impl MiraiEventTrait for GroupAllowConfessTalkEvent {}
impl BroadcastControllableTrait for GroupAllowConfessTalkEvent {}
impl BotPassiveEventTrait for GroupAllowConfessTalkEvent {}
