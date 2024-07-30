use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::{Group, Member};
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    MessageSyncEventTrait, OtherClientEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupMessageSyncEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupAwareMessageTrait<B, Member<B>, Group<B>> for GroupMessageSyncEvent<B> {}
impl<B: BotBackend> MessageSyncEventTrait<B, Member<B>, Group<B>> for GroupMessageSyncEvent<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for GroupMessageSyncEvent<B> {}
impl<B: BotBackend> MessageEventTrait<B, Member<B>, Group<B>> for GroupMessageSyncEvent<B> {}
impl<B: BotBackend> OtherClientEventTrait<B> for GroupMessageSyncEvent<B> {}
impl<B: BotBackend> BotEventTrait<B> for GroupMessageSyncEvent<B> {}
