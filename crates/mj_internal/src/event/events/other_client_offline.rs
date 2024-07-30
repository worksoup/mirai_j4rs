use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait, OtherClientEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct OtherClientOfflineEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> OtherClientEventTrait<B> for OtherClientOfflineEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for OtherClientOfflineEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for OtherClientOfflineEvent<B> {}
