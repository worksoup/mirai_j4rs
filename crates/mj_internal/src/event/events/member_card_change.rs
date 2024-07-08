use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, UserEventTrait,
};

#[mj_event]
pub struct MemberCardChangeEvent {
    instance: Instance,
}
impl GroupMemberEventTrait for MemberCardChangeEvent {}
impl GroupMemberInfoChangeEventTrait for MemberCardChangeEvent {}

impl GroupEventTrait for MemberCardChangeEvent {}
impl BotEventTrait for MemberCardChangeEvent {}
impl UserEventTrait for MemberCardChangeEvent {}

impl BaseGroupMemberInfoChangeEventTrait for MemberCardChangeEvent {}
