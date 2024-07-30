use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::NormalMember;
use crate::event::{
    BotEventTrait, BotPassiveEventTrait, GroupAwareMessageTrait, MessageEventTrait,
    OtherClientEventTrait, TempMessageEventTrait, UserMessageEventTrait,
};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupTempMessageEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupAwareMessageTrait<B, NormalMember<B>, NormalMember<B>>
    for GroupTempMessageEvent<B>
{
}
impl<B: BotBackend> TempMessageEventTrait<B, NormalMember<B>> for GroupTempMessageEvent<B> {}
impl<B: BotBackend> UserMessageEventTrait<B, NormalMember<B>, NormalMember<B>> for GroupTempMessageEvent<B> {}

impl<B: BotBackend> MessageEventTrait<B, NormalMember<B>, NormalMember<B>> for GroupTempMessageEvent<B> {}
impl<B: BotBackend> BotEventTrait<B> for GroupTempMessageEvent<B> {}
impl<B: BotBackend> BotPassiveEventTrait<B> for GroupTempMessageEvent<B> {}
impl<B: BotBackend> OtherClientEventTrait<B> for GroupTempMessageEvent<B> {}
