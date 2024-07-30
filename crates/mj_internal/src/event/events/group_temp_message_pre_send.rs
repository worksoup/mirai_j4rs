use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    TempMessagePreSendEventTrait, UserMessagePreSendEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupTempMessagePreSendEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> UserMessagePreSendEventTrait<B> for GroupTempMessagePreSendEvent<B> {}

impl<B: BotBackend> MessagePreSendEventTrait<B> for GroupTempMessagePreSendEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupTempMessagePreSendEvent<B> {}

impl<B: BotBackend> BotActiveEventTrait<B> for GroupTempMessagePreSendEvent<B> {}

impl<B:BotBackend> CancellableEventTrait<B> for GroupTempMessagePreSendEvent<B> {}

impl<B: BotBackend> TempMessagePreSendEventTrait<B> for GroupTempMessagePreSendEvent<B> {}
