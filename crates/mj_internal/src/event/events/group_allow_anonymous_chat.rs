use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupOperableEventTrait, GroupSettingsChangeEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct GroupAllowAnonymousChatEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<bool> for GroupAllowAnonymousChatEvent {}
impl GroupOperableEventTrait for GroupAllowAnonymousChatEvent {}
impl GroupMemberInfoChangeEventTrait for GroupAllowAnonymousChatEvent {}

impl BotEventTrait for GroupAllowAnonymousChatEvent {}
impl GroupEventTrait for GroupAllowAnonymousChatEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupAllowAnonymousChatEvent {}
impl BroadcastControllableTrait for GroupAllowAnonymousChatEvent {}
impl BotPassiveEventTrait for GroupAllowAnonymousChatEvent {}
