use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::{OtherClient, User};
use crate::event::{BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct OtherClientMessageEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> MessageEventTrait<B, User<B>, OtherClient<B>> for OtherClientMessageEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for OtherClientMessageEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for OtherClientMessageEvent<B> {}
impl<B: BotBackend> OtherClientEventTrait<B> for OtherClientMessageEvent<B> {}
