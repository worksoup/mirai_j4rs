use j4rs::Instance;

use mj_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};

pub trait MemberLeaveEventTrait: GroupMemberEventTrait + GroupMemberInfoChangeEventTrait {}
#[mj_event("event.events.MemberLeaveEvent$Kick")]
pub struct Kick {
    instance: Instance,
}

impl GroupMemberEventTrait for Kick {}

impl UserEventTrait for Kick {}

impl GroupMemberInfoChangeEventTrait for Kick {}

impl BaseGroupMemberInfoChangeEventTrait for Kick {}

impl MemberLeaveEventTrait for Kick {}

impl GroupEventTrait for Kick {}

impl BotEventTrait for Kick {}

impl GroupOperableEventTrait for Kick {}

#[mj_event("event.events.MemberLeaveEvent$Quit")]
pub struct Quit {
    instance: Instance,
}

impl GroupMemberEventTrait for Quit {}

impl GroupEventTrait for Quit {}

impl BotEventTrait for Quit {}

impl UserEventTrait for Quit {}

impl GroupMemberInfoChangeEventTrait for Quit {}

impl BaseGroupMemberInfoChangeEventTrait for Quit {}

impl MemberLeaveEventTrait for Quit {}

#[mj_event]
pub struct MemberLeaveEvent {
    instance: Instance,
}

impl GroupMemberEventTrait for MemberLeaveEvent {}

impl GroupEventTrait for MemberLeaveEvent {}

impl BotEventTrait for MemberLeaveEvent {}

impl UserEventTrait for MemberLeaveEvent {}

impl GroupMemberInfoChangeEventTrait for MemberLeaveEvent {}

impl BaseGroupMemberInfoChangeEventTrait for MemberLeaveEvent {}

impl MemberLeaveEventTrait for MemberLeaveEvent {}
