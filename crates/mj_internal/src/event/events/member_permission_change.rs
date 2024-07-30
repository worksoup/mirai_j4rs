use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberEventTrait, GroupMemberInfoChangeEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct MemberPermissionChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupEventTrait<B> for MemberPermissionChangeEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for MemberPermissionChangeEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for MemberPermissionChangeEvent<B> {}

impl<B: BotBackend> GroupMemberEventTrait<B> for MemberPermissionChangeEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for MemberPermissionChangeEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for MemberPermissionChangeEvent<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for MemberPermissionChangeEvent<B> {}
