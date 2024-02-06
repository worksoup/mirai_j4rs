use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};
use j4rs::Instance;
use mj_macro::mj_event;

#[mj_event]
pub struct MemberMuteEvent {
    instance: Instance,
}

impl GroupEventTrait for MemberMuteEvent {}

impl BotEventTrait for MemberMuteEvent {}

impl UserEventTrait for MemberMuteEvent {}

impl GroupMemberEventTrait for MemberMuteEvent {}
impl GroupOperableEventTrait for MemberMuteEvent {}

impl BaseGroupMemberInfoChangeEventTrait for MemberMuteEvent {}

impl GroupMemberInfoChangeEventTrait for MemberMuteEvent {}
