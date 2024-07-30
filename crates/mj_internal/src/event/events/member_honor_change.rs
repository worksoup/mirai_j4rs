use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupEventTrait, GroupMemberEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct MemberHonorChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> GroupMemberEventTrait<B> for MemberHonorChangeEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for MemberHonorChangeEvent<B> {}

impl<B: BotBackend> GroupEventTrait<B> for MemberHonorChangeEvent<B> {}
impl<B: BotBackend> BotEventTrait<B> for MemberHonorChangeEvent<B> {}
impl<B: BotBackend> UserEventTrait<B> for MemberHonorChangeEvent<B> {}
