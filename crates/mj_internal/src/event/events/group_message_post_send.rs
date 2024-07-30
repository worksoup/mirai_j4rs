use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Group;
use crate::event::{BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupMessagePostSendEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessagePostSendEventTrait<B, Group<B>> for GroupMessagePostSendEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupMessagePostSendEvent<B> {}

impl<B: BotBackend> BotActiveEventTrait<B> for GroupMessagePostSendEvent<B> {}
