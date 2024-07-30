use crate::utils::backend::BotBackend;
use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, GroupEventTrait, GroupMemberEventTrait,
    GroupMemberInfoChangeEventTrait, GroupOperableEventTrait, UserEventTrait,
};

#[mj_event]
pub struct MemberSpecialTitleChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for MemberSpecialTitleChangeEvent<B> {}

impl<B: BotBackend> GroupEventTrait<B> for MemberSpecialTitleChangeEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for MemberSpecialTitleChangeEvent<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for MemberSpecialTitleChangeEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for MemberSpecialTitleChangeEvent<B> {}

impl<B: BotBackend> GroupMemberEventTrait<B> for MemberSpecialTitleChangeEvent<B> {}
impl<B: BotBackend> GroupOperableEventTrait<B> for MemberSpecialTitleChangeEvent<B> {}
