use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberEventTrait, GroupMemberInfoChangeEventTrait, UserEventTrait,
};

pub trait MemberJoinEventTrait:
    GroupMemberEventTrait + BotPassiveEventTrait + GroupMemberInfoChangeEventTrait
{
}
#[mj_event("event.events.MemberJoinEvent$Active")]
pub struct Active {
    instance: Instance,
}

impl GroupMemberEventTrait for Active {}

impl GroupEventTrait for Active {}

impl BotEventTrait for Active {}

impl UserEventTrait for Active {}

impl BotPassiveEventTrait for Active {}

impl GroupMemberInfoChangeEventTrait for Active {}

impl BaseGroupMemberInfoChangeEventTrait for Active {}

impl MemberJoinEventTrait for Active {}
#[mj_event("event.events.MemberJoinEvent$Invite")]
pub struct Invite {
    instance: Instance,
}

impl GroupMemberEventTrait for Invite {}

impl GroupEventTrait for Invite {}

impl BotEventTrait for Invite {}

impl UserEventTrait for Invite {}

impl BotPassiveEventTrait for Invite {}

impl GroupMemberInfoChangeEventTrait for Invite {}

impl BaseGroupMemberInfoChangeEventTrait for Invite {}

impl MemberJoinEventTrait for Invite {}
#[mj_event("event.events.MemberJoinEvent$Retrieve")]
pub struct Retrieve {
    instance: Instance,
}

impl GroupMemberEventTrait for Retrieve {}

impl GroupEventTrait for Retrieve {}

impl BotEventTrait for Retrieve {}

impl UserEventTrait for Retrieve {}

impl BotPassiveEventTrait for Retrieve {}

impl GroupMemberInfoChangeEventTrait for Retrieve {}

impl BaseGroupMemberInfoChangeEventTrait for Retrieve {}

impl MemberJoinEventTrait for Retrieve {}

#[mj_event]
pub struct MemberJoinEvent {
    instance: Instance,
}

impl GroupMemberEventTrait for MemberJoinEvent {}

impl GroupEventTrait for MemberJoinEvent {}

impl BotEventTrait for MemberJoinEvent {}

impl UserEventTrait for MemberJoinEvent {}

impl BotPassiveEventTrait for MemberJoinEvent {}

impl GroupMemberInfoChangeEventTrait for MemberJoinEvent {}

impl BaseGroupMemberInfoChangeEventTrait for MemberJoinEvent {}

impl MemberJoinEventTrait for MemberJoinEvent {}
