use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::contact::{Group, Member};
use crate::event::{BotEventTrait, BotPassiveEventTrait, MessageEventTrait, OtherClientEventTrait};
use crate::utils::backend::BotBackend;

#[mj_event]
pub struct GroupMessageEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> GroupMessageEvent<B> {}

// 实现了 message 所需要的函数。
impl<B: BotBackend> MessageEventTrait<B, Member<B>, Group<B>> for GroupMessageEvent<B> {}
impl<B: BotBackend> OtherClientEventTrait<B> for GroupMessageEvent<B> {}

impl<B: BotBackend> BotEventTrait<B> for GroupMessageEvent<B> {}

impl<B: BotBackend> BotPassiveEventTrait<B> for GroupMessageEvent<B> {}
