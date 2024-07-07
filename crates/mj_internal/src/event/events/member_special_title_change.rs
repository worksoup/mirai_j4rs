use j4rs::errors::J4RsError;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};

#[mj_event]
pub struct MemberSpecialTitleChangeEvent {
    instance: Instance,
}

impl BotEventTrait for MemberSpecialTitleChangeEvent {}

impl GroupEventTrait for MemberSpecialTitleChangeEvent {}

impl BaseGroupMemberInfoChangeEventTrait for MemberSpecialTitleChangeEvent {}

impl GroupMemberInfoChangeEventTrait for MemberSpecialTitleChangeEvent {}

impl UserEventTrait for MemberSpecialTitleChangeEvent {}

impl GroupMemberEventTrait for MemberSpecialTitleChangeEvent {}
impl GroupOperableEventTrait for MemberSpecialTitleChangeEvent {}
