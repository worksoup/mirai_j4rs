use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::NormalMember;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    MessageSyncEventTrait, OtherClientEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupTempMessageSyncEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}
impl<B: BotBackend> MessageSyncEventTrait<B, NormalMember<B>, NormalMember<B>>
    for GroupTempMessageSyncEvent<B>
{
}
impl<B: BotBackend> GroupAwareMessageTrait<B, NormalMember<B>, NormalMember<B>>
    for GroupTempMessageSyncEvent<B>
{
}

impl<B: BotBackend> BotEventTrait<B> for GroupTempMessageSyncEvent<B> {}
impl<B: BotBackend> OtherClientEventTrait<B> for GroupTempMessageSyncEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for GroupTempMessageSyncEvent<B> {}
impl<B: BotBackend> MessageEventTrait<B, NormalMember<B>, NormalMember<B>>
    for GroupTempMessageSyncEvent<B>
{
}
