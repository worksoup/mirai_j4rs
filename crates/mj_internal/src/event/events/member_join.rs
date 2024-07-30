use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberEventTrait, GroupMemberInfoChangeEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

pub trait MemberJoinEventTrait<B: BotBackend>:
    GroupMemberEventTrait<B> + BotPassiveEventTrait<B> + GroupMemberInfoChangeEventTrait<B>
{
}
#[mj_event("event.events.MemberJoinEvent$Active")]
pub struct Active<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMemberEventTrait<B> for Active<B> {}

impl<B: BotBackend> GroupEventTrait<B> for Active<B> {}

impl<B: BotBackend> BotEventTrait<B> for Active<B> {}

impl<B: BotBackend> UserEventTrait<B> for Active<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for Active<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Active<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Active<B> {}

impl<B: BotBackend> MemberJoinEventTrait<B> for Active<B> {}
#[mj_event("event.events.MemberJoinEvent$Invite")]
pub struct Invite<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMemberEventTrait<B> for Invite<B> {}

impl<B: BotBackend> GroupEventTrait<B> for Invite<B> {}

impl<B: BotBackend> BotEventTrait<B> for Invite<B> {}

impl<B: BotBackend> UserEventTrait<B> for Invite<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for Invite<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Invite<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Invite<B> {}

impl<B: BotBackend> MemberJoinEventTrait<B> for Invite<B> {}
#[mj_event("event.events.MemberJoinEvent$Retrieve")]
pub struct Retrieve<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMemberEventTrait<B> for Retrieve<B> {}

impl<B: BotBackend> GroupEventTrait<B> for Retrieve<B> {}

impl<B: BotBackend> BotEventTrait<B> for Retrieve<B> {}

impl<B: BotBackend> UserEventTrait<B> for Retrieve<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for Retrieve<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for Retrieve<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for Retrieve<B> {}

impl<B: BotBackend> MemberJoinEventTrait<B> for Retrieve<B> {}

#[mj_event]
pub struct MemberJoinEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMemberEventTrait<B> for MemberJoinEvent<B> {}

impl<B: BotBackend> GroupEventTrait<B> for MemberJoinEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for MemberJoinEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for MemberJoinEvent<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for MemberJoinEvent<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for MemberJoinEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for MemberJoinEvent<B> {}

impl<B: BotBackend> MemberJoinEventTrait<B> for MemberJoinEvent<B> {}
