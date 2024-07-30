use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::Stranger;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, MessageEventTrait, MessageSyncEventTrait,
    OtherClientEventTrait, StrangerEventTrait, UserEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct StrangerMessageSyncEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for StrangerMessageSyncEvent<B> {}

impl<B: BotBackend> UserEventTrait<B> for StrangerMessageSyncEvent<B> {}

impl<B: BotBackend> StrangerEventTrait<B> for StrangerMessageSyncEvent<B> {}

impl<B: BotBackend> MessageEventTrait<B, Stranger<B>, Stranger<B>> for StrangerMessageSyncEvent<B> {}

impl<B: BotBackend> OtherClientEventTrait<B> for StrangerMessageSyncEvent<B> {}

impl<B: BotBackend> MessageSyncEventTrait<B, Stranger<B>, Stranger<B>>
    for StrangerMessageSyncEvent<B>
{
}
impl<B: BotBackend> BotPassiveEventTrait<B> for StrangerMessageSyncEvent<B> {}
