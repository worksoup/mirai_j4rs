use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct MemberUnmuteEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupEventTrait<B> for MemberUnmuteEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for MemberUnmuteEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for MemberUnmuteEvent<B> {}

impl<B: BotBackend> GroupMemberEventTrait<B> for MemberUnmuteEvent<B> {}
impl<B: BotBackend> GroupOperableEventTrait<B> for MemberUnmuteEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for MemberUnmuteEvent<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for MemberUnmuteEvent<B> {}
