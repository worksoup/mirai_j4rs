use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait, MiraiEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_all;

#[mj_all("net.mamoe.mirai.event.events.GroupAllowMemberInviteEvent")]
pub struct GroupAllowMemberInviteEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<bool> for GroupAllowMemberInviteEvent {}
impl GroupOperableEventTrait for GroupAllowMemberInviteEvent {}
impl GroupMemberInfoChangeEventTrait for GroupAllowMemberInviteEvent {}

impl MiraiEventTrait for GroupAllowMemberInviteEvent {}
impl BotEventTrait for GroupAllowMemberInviteEvent {}
impl GroupEventTrait for GroupAllowMemberInviteEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupAllowMemberInviteEvent {}
impl BroadcastControllableTrait for GroupAllowMemberInviteEvent {}
impl BotPassiveEventTrait for GroupAllowMemberInviteEvent {}
