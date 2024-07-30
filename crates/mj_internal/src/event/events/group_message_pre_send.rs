use j4rs::Instance;

use mj_helper_macro::mj_event;

// TODO
use crate::event::{
    BotActiveEventTrait, BotEventTrait, CancellableEventTrait, MessagePreSendEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupMessagePreSendEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for GroupMessagePreSendEvent<B> {}

impl<B: BotBackend> BotActiveEventTrait<B> for GroupMessagePreSendEvent<B> {}

impl<B:BotBackend> CancellableEventTrait<B> for GroupMessagePreSendEvent<B> {}

impl<B: BotBackend> MessagePreSendEventTrait<B> for GroupMessagePreSendEvent<B> {}
