use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct MemberCardChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> GroupMemberEventTrait<B> for MemberCardChangeEvent<B> {}
impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for MemberCardChangeEvent<B> {}

impl<B: BotBackend> GroupEventTrait<B> for MemberCardChangeEvent<B> {}
impl<B: BotBackend> BotEventTrait<B> for MemberCardChangeEvent<B> {}
impl<B: BotBackend> UserEventTrait<B> for MemberCardChangeEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for MemberCardChangeEvent<B> {}
