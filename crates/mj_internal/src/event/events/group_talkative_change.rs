use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait, GroupEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupTalkativeChangeEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> BotPassiveEventTrait<B> for GroupTalkativeChangeEvent<B> {}
impl<B: BotBackend> GroupEventTrait<B> for GroupTalkativeChangeEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupTalkativeChangeEvent<B> {}
