use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BaseGroupMemberInfoChangeEventTrait, BotEventTrait, BotPassiveEventTrait, GroupEventTrait,
    GroupMemberInfoChangeEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct BotMuteEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for BotMuteEvent<B> {}

impl <B: BotBackend>GroupEventTrait<B> for BotMuteEvent<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for BotMuteEvent<B> {}

impl<B: BotBackend> BaseGroupMemberInfoChangeEventTrait<B> for BotMuteEvent<B> {}

impl<B: BotBackend> GroupMemberInfoChangeEventTrait<B> for BotMuteEvent<B> {}
