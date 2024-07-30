use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
    UserMessagePreSendEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct StrangerMessagePreSendEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessagePreSendEventTrait<B> for StrangerMessagePreSendEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for StrangerMessagePreSendEvent<B> {}

impl<B: BotBackend> BotActiveEventTrait<B> for StrangerMessagePreSendEvent<B> {}

impl<B:BotBackend> CancellableEventTrait<B> for StrangerMessagePreSendEvent<B> {}

impl<B: BotBackend> UserMessagePreSendEventTrait<B> for StrangerMessagePreSendEvent<B> {}
