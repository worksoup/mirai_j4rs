use j4rs::Instance;

use mj_helper_macro::mj_event;

use crate::event::{BotEventTrait, GroupEventTrait, GroupOperableEventTrait};
use crate::utils::backend::BotBackend;

pub trait MessageRecallTrait<B: BotBackend>: BotEventTrait<B> {}
#[mj_event("event.events.MessageRecallEvent$FriendRecall")]
pub struct FriendRecall<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for FriendRecall<B> {}

impl<B: BotBackend> MessageRecallTrait<B> for FriendRecall<B> {}

#[mj_event("event.events.MessageRecallEvent$GroupRecall")]
pub struct GroupRecall<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for GroupRecall<B> {}

impl<B: BotBackend> MessageRecallTrait<B> for GroupRecall<B> {}

impl<B: BotBackend> GroupEventTrait<B> for GroupRecall<B> {}

impl<B: BotBackend> GroupOperableEventTrait<B> for GroupRecall<B> {}

#[mj_event]
pub struct MessageRecallEvent<B: BotBackend> {
    instance: Instance,
    _backend: B,
}

impl<B: BotBackend> BotEventTrait<B> for MessageRecallEvent<B> {}

impl<B: BotBackend> MessageRecallTrait<B> for MessageRecallEvent<B> {}
