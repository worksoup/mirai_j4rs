use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Member;
use crate::event::{
    BotActiveEventTrait, BotEventTrait, MessagePostSendEventTrait, TempMessagePostSendEventTrait,
    UserMessagePostSendEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupTempMessagePostSendEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> TempMessagePostSendEventTrait<B> for GroupTempMessagePostSendEvent<B> {}

impl<B: BotBackend> UserMessagePostSendEventTrait<B, Member<B>> for GroupTempMessagePostSendEvent<B> {}
impl<B: BotBackend> MessagePostSendEventTrait<B, Member<B>> for GroupTempMessagePostSendEvent<B> {}
impl<B: BotBackend> BotEventTrait<B> for GroupTempMessagePostSendEvent<B> {}
impl<B: BotBackend> BotActiveEventTrait<B> for GroupTempMessagePostSendEvent<B> {}
