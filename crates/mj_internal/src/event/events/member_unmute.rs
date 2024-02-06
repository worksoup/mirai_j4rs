use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct MemberUnmuteEvent {
    instance: Instance,
}

impl GroupEventTrait for MemberUnmuteEvent {}

impl BotEventTrait for MemberUnmuteEvent {}

impl UserEventTrait for MemberUnmuteEvent {}

impl GroupMemberEventTrait for MemberUnmuteEvent {}
impl GroupOperableEventTrait for MemberUnmuteEvent {}

impl BaseGroupMemberInfoChangeEventTrait for MemberUnmuteEvent {}

impl GroupMemberInfoChangeEventTrait for MemberUnmuteEvent {}
