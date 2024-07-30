use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, BotPassiveEventTrait, OtherClientEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct OtherClientOnlineEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B:BotBackend> OtherClientEventTrait<B> for OtherClientOnlineEvent<B> {}
impl<B:BotBackend> BotPassiveEventTrait<B> for OtherClientOnlineEvent<B> {}

impl<B:BotBackend> BotEventTrait<B> for OtherClientOnlineEvent<B> {}
