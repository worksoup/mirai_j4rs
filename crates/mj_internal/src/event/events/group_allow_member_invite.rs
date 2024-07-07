use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};

#[mj_event]
pub struct GroupAllowMemberInviteEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<bool> for GroupAllowMemberInviteEvent {}
impl GroupOperableEventTrait for GroupAllowMemberInviteEvent {}
impl GroupMemberInfoChangeEventTrait for GroupAllowMemberInviteEvent {}

impl BotEventTrait for GroupAllowMemberInviteEvent {}
impl GroupEventTrait for GroupAllowMemberInviteEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupAllowMemberInviteEvent {}
impl BroadcastControllableTrait for GroupAllowMemberInviteEvent {}
impl BotPassiveEventTrait for GroupAllowMemberInviteEvent {}
