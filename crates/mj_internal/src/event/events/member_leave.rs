use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

pub trait MemberLeaveEventTrait<B: BotBackend>:
    GroupMemberEventTrait<B> + GroupMemberInfoChangeEventTrait<B>
{
}
#[mj_event("event.events.MemberLeaveEvent$Kick")]
pub struct Kick<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMemberEventTrait<B> for Kick<B> {}

impl<B: BotBackend> UserEventTrait<B> for Kick<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Kick<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Kick<B> {}

impl<B: BotBackend> MemberLeaveEventTrait<B> for Kick<B> {}

impl<B: BotBackend> GroupEventTrait<B> for Kick<B> {}

impl<B: BotBackend> BotEventTrait<B> for Kick<B> {}

impl<B: BotBackend> GroupOperableEventTrait<B> for Kick<B> {}

#[mj_event("event.events.MemberLeaveEvent$Quit")]
pub struct Quit<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMemberEventTrait<B> for Quit<B> {}

impl<B: BotBackend> GroupEventTrait<B> for Quit<B> {}

impl<B: BotBackend> BotEventTrait<B> for Quit<B> {}

impl<B: BotBackend> UserEventTrait<B> for Quit<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Quit<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Quit<B> {}

impl<B: BotBackend> MemberLeaveEventTrait<B> for Quit<B> {}

#[mj_event]
pub struct MemberLeaveEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMemberEventTrait<B> for MemberLeaveEvent<B> {}

impl<B: BotBackend> GroupEventTrait<B> for MemberLeaveEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for MemberLeaveEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for MemberLeaveEvent<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for MemberLeaveEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for MemberLeaveEvent<B> {}

impl<B: BotBackend> MemberLeaveEventTrait<B> for MemberLeaveEvent<B> {}
