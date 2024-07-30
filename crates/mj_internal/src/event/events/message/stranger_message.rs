use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Stranger;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait,
    StrangerEventTrait, UserEventTrait, UserMessageEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct StrangerMessageEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> MessageEventTrait<B, Stranger<B>, Stranger<B>> for StrangerMessageEvent<B> {}

impl<B: BotBackend> UserMessageEventTrait<B, Stranger<B>, Stranger<B>> for StrangerMessageEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for StrangerMessageEvent<B> {}
impl<B: BotBackend> OtherClientEventTrait<B> for StrangerMessageEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for StrangerMessageEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for StrangerMessageEvent<B> {}

impl<B: BotBackend> StrangerEventTrait<B> for StrangerMessageEvent<B> {}
