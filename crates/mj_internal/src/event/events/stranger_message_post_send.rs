use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Stranger;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, UserMessagePostSendEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct StrangerMessagePostSendEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B:BotBackend> MessagePostSendEventTrait<B, Stranger<B>> for StrangerMessagePostSendEvent<B> {}

impl<B:BotBackend> BotEventTrait<B> for StrangerMessagePostSendEvent<B> {}

impl<B:BotBackend> BotActiveEventTrait<B> for StrangerMessagePostSendEvent<B> {}

impl<B:BotBackend> UserMessagePostSendEventTrait<B, Stranger<B>> for StrangerMessagePostSendEvent<B> {}
