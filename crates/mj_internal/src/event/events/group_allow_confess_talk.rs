use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait,
    BroadcastControllableTrait, GroupEventTrait, GroupMemberInfoChangeEventTrait,
    GroupSettingsChangeEventTrait,
};

#[mj_event]
pub struct GroupAllowConfessTalkEvent {
    instance: Instance,
}

impl GroupSettingsChangeEventTrait<bool> for GroupAllowConfessTalkEvent {}
impl GroupMemberInfoChangeEventTrait for GroupAllowConfessTalkEvent {}

impl BotEventTrait for GroupAllowConfessTalkEvent {}
impl GroupEventTrait for GroupAllowConfessTalkEvent {}
impl BaseGroupMemberInfoChangeEventTrait for GroupAllowConfessTalkEvent {}

impl BroadcastControllableTrait for GroupAllowConfessTalkEvent {}
impl BotPassiveEventTrait for GroupAllowConfessTalkEvent {}
