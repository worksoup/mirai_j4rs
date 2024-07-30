use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct MemberMuteEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupEventTrait<B> for MemberMuteEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for MemberMuteEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for MemberMuteEvent<B> {}

impl<B: BotBackend> GroupMemberEventTrait<B> for MemberMuteEvent<B> {}
impl<B: BotBackend> GroupOperableEventTrait<B> for MemberMuteEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for MemberMuteEvent<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for MemberMuteEvent<B> {}
