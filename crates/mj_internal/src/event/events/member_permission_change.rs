use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberEventTrait, GroupMemberInfoChangeEventTrait, UserEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct MemberPermissionChangeEvent {
    instance: Instance,
}

impl GroupEventTrait for MemberPermissionChangeEvent {}

impl BotEventTrait for MemberPermissionChangeEvent {}

impl UserEventTrait for MemberPermissionChangeEvent {}

impl GroupMemberEventTrait for MemberPermissionChangeEvent {}
impl BotPassiveEventTrait for MemberPermissionChangeEvent {}

impl BaseGroupMemberInfoChangeEventTrait for MemberPermissionChangeEvent {}

impl GroupMemberInfoChangeEventTrait for MemberPermissionChangeEvent {}
